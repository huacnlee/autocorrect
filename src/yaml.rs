// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/yaml.pest"]
struct YAMLParser;

#[allow(dead_code)]
pub fn format_yaml(text: &str, lint: bool) -> String {
  let pairs = YAMLParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_yaml() {
    let example = r#"# this is comment line
foo: 'hello世界'
region:
  cn-north-1
"en":
  name: "你好Hello世界"
  foo: Bar
  dar:
    - foo: 1
    - bar: "数字2"
  "abc字段": abc字段
"#;

    let expect = r#"# this is comment line
foo: 'hello 世界'
region:
  cn-north-1
"en":
  name: "你好 Hello 世界"
  foo: Bar
  dar:
    - foo: 1
    - bar: "数字 2"
  "abc字段": abc 字段
"#;

    assert_eq!(expect, format_yaml(example, false))
  }
}
