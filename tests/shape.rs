#[test]
fn test_shape_expr() {
  const MODEL: &str = "
[Encoder]
  #0 Input = 3, H, W
  #1 Pooling = 3, H+4, W+4
  #2 Padding = 3, (H+4)/2, (W+4)/2
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
