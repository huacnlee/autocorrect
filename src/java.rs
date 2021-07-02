// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/java.pest"]
struct JavaParser;

pub fn format_java(text: &str, lint: bool) -> String {
  let result = JavaParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_java_pair(&mut out, item, lint);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_java_pair(text: &mut String, item: Pair<Rule>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  match item.as_rule() {
    Rule::string | Rule::comment => format_or_lint(text, part, true, lint, line, col),
    Rule::item => {
      for sub in item.into_inner() {
        format_java_pair(text, sub, lint);
      }
    }
    _ => format_or_lint(text, part, true, lint, line, col),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_java() {
    let example = r###"
// 第1行注释
// 第2行注释
public String helloWorld() {
  // 第3行注释
  String singleLineString = "第1个字符串string"

  /**
   * 第4行注释
   * 第5行注释
   */
  String quotation = """
  这是多行string里面包含"双引号"
  "Begin at the beginning," the King said gravely.
  """
}
"###;

    let expect = r###"
// 第 1 行注释
// 第 2 行注释
public String helloWorld() {
  // 第 3 行注释
  String singleLineString = "第 1 个字符串 string"

  /**
   * 第 4 行注释
   * 第 5 行注释
   */
  String quotation = """
  这是多行 string 里面包含"双引号"
  "Begin at the beginning," the King said gravely.
  """
}
"###;

    assert_eq!(expect, format_java(example, false));
  }
}
