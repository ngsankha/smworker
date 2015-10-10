extern crate smworker;

#[test]
fn test_js() {
  let worker = smworker::new();
  let code = "     \
  var a = 1 + 2;   \
  var b = \"abc\"; \
  a == b;          \
  ";
  assert!(worker.execute("test_js".to_string(), code.to_string()).is_ok());
}

#[test]
fn test_builtin() {
  let worker = smworker::new();
  let code = "       \
  var a = \"abc\";   \
  a.substring(0, 1); \
  ";
  assert!(worker.execute("test_builtin".to_string(), code.to_string()).is_ok());
}

#[test]
fn test_builtin_err() {
  let worker = smworker::new();
  let code = "       \
  var a = \"abc\";   \
  a.abc(0, 1);       \
  ";
  assert!(worker.execute("test_builtin_err".to_string(), code.to_string()).is_err());
}
