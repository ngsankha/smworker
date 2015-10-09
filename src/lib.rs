extern crate js;
extern crate libc;


use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::ffi::CStr;
use std::ptr;
use std::str;

use js::{JSCLASS_RESERVED_SLOTS_MASK,JSCLASS_RESERVED_SLOTS_SHIFT,JSCLASS_GLOBAL_SLOT_COUNT,JSCLASS_IS_GLOBAL, JSCLASS_IMPLEMENTS_BARRIERS};
use js::jsapi::JS_GlobalObjectTraceHook;
use js::jsapi::{CallArgs,CompartmentOptions,OnNewGlobalHookOption,Rooted,Value};
use js::jsapi::{JS_DefineFunction,JS_Init,JS_NewGlobalObject, JS_InitStandardClasses,JS_EncodeStringToUTF8, JS_ReportPendingException, JS_BufferIsCompilableUnit, JS_DestroyContext, JS_DestroyRuntime, JS_ShutDown, CurrentGlobalOrNull, JS_LeaveCompartment};
use js::jsapi::{JSAutoCompartment,JSAutoRequest,JSContext,JSClass};
use js::jsapi::{JS_SetGCParameter, JSGCParamKey, JSGCMode};
use js::jsapi::{RootedValue, RootedObject, HandleObject, HandleValue};
use js::jsval::UndefinedValue;
use js::rust::Runtime;

static CLASS: &'static JSClass = &JSClass {
  name: b"global\0" as *const u8 as *const libc::c_char,
  flags: JSCLASS_IS_GLOBAL | JSCLASS_IMPLEMENTS_BARRIERS | ((JSCLASS_GLOBAL_SLOT_COUNT & JSCLASS_RESERVED_SLOTS_MASK) << JSCLASS_RESERVED_SLOTS_SHIFT),
  addProperty: None,
  delProperty: None,
  getProperty: None,
  setProperty: None,
  enumerate: None,
  resolve: None,
  convert: None,
  finalize: None,
  call: None,
  hasInstance: None,
  construct: None,
  trace: Some(JS_GlobalObjectTraceHook),
  reserved: [0 as *mut _; 25]
};

struct JSOptions {
  interactive: bool,
  disable_baseline: bool,
  disable_ion: bool,
  disable_asmjs: bool,
  disable_native_regexp: bool,
  script: String,
}

pub struct SMWorker {
  ac: JSAutoCompartment,
  cx: *mut JSContext,
  runtime: Runtime,
  tx: Sender<String>,
  rx: Receiver<String>
}

impl SMWorker {
  pub fn execute(&self, label: String, script: String) {
    let cx = self.cx;
    let global = unsafe { CurrentGlobalOrNull(cx) };
    let global_root = Rooted::new(cx, global);
    let global = global_root.handle();
    match self.runtime.evaluate_script(global, script, label, 1) {
      Err(_) => unsafe { JS_ReportPendingException(cx); },
      _ => ()
    }
  }
}

pub fn new() -> SMWorker {
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

  unsafe {
    JS_Init();
  }

  let runtime = Runtime::new();
  let cx = runtime.cx();
  let h_option = OnNewGlobalHookOption::FireOnNewGlobalHook;
  let c_option = CompartmentOptions::default();
  let _ar = JSAutoRequest::new(cx);
  let global = unsafe { JS_NewGlobalObject(cx, CLASS, ptr::null_mut(), h_option, &c_option) };
  let global_root = Rooted::new(cx, global);
  let global = global_root.handle();
  let ac = JSAutoCompartment::new(cx, global.get());

  unsafe {
    JS_SetGCParameter(runtime.rt(), JSGCParamKey::JSGC_MODE, JSGCMode::JSGC_MODE_INCREMENTAL as u32);
    JS_InitStandardClasses(cx, global);
    let function = JS_DefineFunction(cx, global, b"puts\0".as_ptr() as *const libc::c_char,
                                   Some(puts), 1, 0);
    assert!(!function.is_null());
  }

  SMWorker { runtime: runtime, cx: cx, ac: ac, tx: tx, rx: rx }
}

unsafe extern "C" fn puts(context: *mut JSContext, argc: u32, vp: *mut Value) -> u8 {
    let args = CallArgs::from_vp(vp, argc);

    /*if args._base.argc_ != 1 {
        JS_ReportError(context, b"puts() requires exactly 1 argument\0".as_ptr() as *const libc::c_char);
        return 1;
    }*/

    let arg = args.get(0);
    let js = js::rust::ToString(context, arg);
    let message_root = Rooted::new(context, js);
    let message = JS_EncodeStringToUTF8(context, message_root.handle());
    let message = CStr::from_ptr(message);
    println!("{}", str::from_utf8(message.to_bytes()).unwrap());

    args.rval().set(UndefinedValue());
    return 0;
}
