#[test]
fn test_inline_passes() {
  const MODEL: &str = "
[InlinePassing]
  #0 Input = 3, H, W
  #1 Simple layer
  #2 Simple layer
  #3 Residual layer (#1 + #2)
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}

#[test]
fn test_keyword_args() {
  const MODEL: &str = "
[CustomModel]

  [Conv2d]
    * K: kernel size = 3

  #0 Input = 3, H, W
  #1 Simple layer
  #2 Conv2d (K=5)
  #3 Conv2d (#2, K=5)
  #4 Conv2d (#3)
  #5 Conv2d
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}

#[test]
fn test_keyword_bool_args() {
  const MODEL: &str = "
[Transform]

  #0 dynamic (transform=yes)
  #1 dynamic
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}

#[test]
fn test_multiple_inputs() {
  const MODEL: &str = "
[Reduce]

  #0 fixed = [0: 42], [1: 24],
  #1 Linear (#0:0) = 12
  #2 Linear (#0:1) = 23
  #3 Join (#1, #2) = 35
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
