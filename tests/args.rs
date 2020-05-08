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
[InlinePassing]

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
