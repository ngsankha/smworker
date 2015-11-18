extern crate smworker;

fn main() {
  println!("Hello");
  let worker = smworker::new();
  let code = "     \
  puts(\"World\"); \
  ";
  worker.execute("test_js".to_string(), code.to_string());
}
