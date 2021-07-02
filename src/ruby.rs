// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/ruby.pest"]
struct RubyParser;

#[allow(dead_code)]
pub fn format_ruby(text: &str, lint: bool) -> String {
  let result = RubyParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_ruby_pair(&mut out, item, lint);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_ruby_pair(text: &mut String, item: Pair<Rule>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  match item.as_rule() {
    Rule::string | Rule::comment => format_or_lint(text, part, true, lint, line, col),
    Rule::item => {
      for sub in item.into_inner() {
        format_ruby_pair(text, sub, lint);
      }
    }
    _ => format_or_lint(text, part, true, lint, line, col),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_ruby() {
    let example = r###"
# 第1行注释
# 第2行注释
def hello(a, b: "第1个参数")
  a = "hello世界#{a}"
  b = '你好hello世界'
end
"###;

    let expect = r###"
# 第 1 行注释
# 第 2 行注释
def hello(a, b: "第 1 个参数")
  a = "hello 世界#{a}"
  b = '你好 hello 世界'
end
"###;

    assert_eq!(expect, format_ruby(example, false));
  }
}
