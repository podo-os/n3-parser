#[test]
fn test_empty_model() {
  const MODEL: &str = "
[EmptyModel]
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
