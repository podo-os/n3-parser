#[test]
fn test_variables() {
  const MODEL: &str = "
[VarModel]
  * F: F o o
  * S: stride = 3
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
