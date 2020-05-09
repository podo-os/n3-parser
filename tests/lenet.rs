#[test]
fn test_lenet_model() {
  const MODEL: &str = "
[LeNet]
  [Conv2d]
    * kernel size = 5
    * stride = 2

  #0 Input RGB image = 1, 28, 28

  #1 Conv2d + ReLU = 32, 14, 14
  #2 Conv2d + ReLU = 64, 7, 7
  #3 Transform = 64 * 7 * 7
  #4 Linear + Sigmoid = 10
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
