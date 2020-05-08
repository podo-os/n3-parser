#[test]
fn test_comments() {
  const MODEL: &str = "
// Comment Model
[CommentModel]

// Yes, comment model

  [Conv2d]  // override convolution layer
    * K: kernel size = 3  // default kernel size

  #0 Input = 3, H, W  // define input layer
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
