#[test]
fn test_simple_graph() {
  const MODEL: &str = "
[SimpleModel]
  #0 Input = 3, H, W
  #1 Simple Layer
  #2 Simple Layer = 3, H, W
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}

#[test]
fn test_inline_model() {
  const MODEL: &str = "
[RootModel]
  #0 Input = 3, H, W
  #1 [Inline Model]
    #1 Simple Layer
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}

#[test]
fn test_passing_pattern() {
  const MODEL: &str = "
[Passing]
  #0 Input = 3, H, W
  #1 First layer + Second layer
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}

#[test]
fn test_residual_pattern() {
  const MODEL: &str = "
[ResidualBlockRoot]
  [ResidualBlock]
  #1 Conv2d + BatchNorm + ReLU
  #2 Conv2d + BatchNorm
  #3 Concat (#1, #2, D=1)
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
