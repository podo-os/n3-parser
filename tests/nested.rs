#[test]
fn test_nested_models() {
  const MODEL: &str = "
[RootModel]
  [ParentModel]
    [ChildModel]
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
