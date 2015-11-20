extern crate smworker;

fn main() {
  println!("Hello");
  let worker = smworker::new(|x| println!("{:?}", x));
  let code = "     \
  puts(\"World\"); \
  ";
  worker.execute("test_js".to_string(), code.to_string());
}
