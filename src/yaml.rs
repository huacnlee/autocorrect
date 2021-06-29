use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/yaml.pest"]
struct YAMLParser;

pub fn format_yaml(text: &str) -> String {
  let result = YAMLParser::parse(Rule::item, text);

  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        out.push_str(format_yaml_pair(item).as_str());
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_yaml_pair(item: Pair<Rule>) -> String {
  let mut text = String::new();
  match item.as_rule() {
    Rule::value | Rule::comment => text.push_str(format(item.as_str()).as_str()),
    Rule::item => {
      for sub in item.into_inner() {
        text.push_str(format_yaml_pair(sub).as_str());
      }
    }
    _ => text.push_str(item.as_str()),
  };

  return text;
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

    assert_eq!(expect, format_yaml(example))
  }
}
