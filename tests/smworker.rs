extern crate smworker;

#[test]
fn test_sm() {
  let worker = smworker::new();
  worker.execute("add.js".to_string(), "1+2;".to_string());
}
