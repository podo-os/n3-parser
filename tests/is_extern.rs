#[test]
fn test_extern_model() {
  const MODEL: &str = "
extern [ExternModel]
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
