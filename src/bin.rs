extern crate smworker;

use smworker::{SMWorker, Function};

struct TestFn;

impl Function for TestFn {
  fn callback(&self, message: String) {
    println!("{:?}", message);
  }
}

fn main() {
  let test_fn = TestFn;

  println!("Hello");
  let worker = SMWorker::new(test_fn);
  let code = "     \
  _send(\"World\"); \
  ";
  worker.execute("test_js".to_string(), code.to_string());
}
