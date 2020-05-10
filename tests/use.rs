#[test]
fn test_use_other_models() {
  const MODEL: &str = "
use LeNet by Yann LeCun et al
use ResNet by \"https://arxiv.org/pdf/1512.03385.pdf\"

[IntegratedModel]
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
