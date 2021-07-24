// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/csharp.pest"]
struct CSharpParser;

#[allow(dead_code)]
pub fn format_csharp(text: &str) -> code::FormatResult {
  let pairs = CSharpParser::parse(Rule::item, text);
  let text = code::FormatResult::new(text);
  return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_csharp(text: &str) -> code::LintResult {
  let pairs = CSharpParser::parse(Rule::item, text);
  let text = code::LintResult::new(text);
  return code::format_pairs(text, pairs);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_csharp() {
    let example = r###"
/**
 * 第1行注释
 * 第2行注释
 */
public String helloWorld(stirng name) {
  // 第3行注释
  string singleLineString = "第1个字符串string";
  string stringLiteral = $"这是stringLiteral {name}!";

  string quotation = @"
  这是多行string第1行
  这是多行string第2行
  ";

  Regex rx = new Regex( @"re正则", RegexOptions.Compiled  | RegexOptions.IgnoreCase);
}
"###;

    let expect = r###"
/**
 * 第 1 行注释
 * 第 2 行注释
 */
public String helloWorld(stirng name) {
  // 第 3 行注释
  string singleLineString = "第 1 个字符串 string";
  string stringLiteral = $"这是 stringLiteral {name}!";

  string quotation = @"
  这是多行 string 第 1 行
  这是多行 string 第 2 行
  ";

  Regex rx = new Regex( @"re正则", RegexOptions.Compiled  | RegexOptions.IgnoreCase);
}
"###;

    assert_eq!(expect, format_csharp(example).to_string());
  }
}
