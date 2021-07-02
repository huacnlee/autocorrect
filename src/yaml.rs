// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/yaml.pest"]
struct YAMLParser;

pub fn format_yaml(text: &str, lint: bool) -> String {
  let result = YAMLParser::parse(Rule::item, text);

  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_yaml_pair(&mut out, item, lint);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_yaml_pair(text: &mut String, item: Pair<Rule>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  match item.as_rule() {
    Rule::value | Rule::comment => format_or_lint(text, part, true, lint, line, col),
    Rule::item => {
      for sub in item.into_inner() {
        format_yaml_pair(text, sub, lint);
      }
    }
    _ => format_or_lint(text, part, true, lint, line, col),
  };
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
