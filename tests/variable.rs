#[test]
fn test_variables() {
  const MODEL: &str = "
[VarModel]
  * F: F o o
  * S: stride = 3

  * unsigned integer = 42
  * negative integer = -42
  * real = 3.14
  * bool true = yes
  * bool false = no
  * [Model Variable]
  * [Optional Model Variable] = Default Model
";

  n3_parser::parser::parse_file(MODEL).unwrap();
}
