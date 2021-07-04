// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/swift.pest"]
struct SwiftParser;

pub fn format_swift(text: &str, lint: bool) -> String {
  let pairs = SwiftParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_swift() {
    let example = r###"
// 第1行注释
// 第2行注释
func helloWorld(name: String) -> String {
  // 第3行注释
  let singleLineString = "第1个字符串string"

  let quotation = """
这是多行string里面包含"双引号"
"Begin at the beginning," the King said gravely.
"""
}
"###;

    let expect = r###"
// 第 1 行注释
// 第 2 行注释
func helloWorld(name: String) -> String {
  // 第 3 行注释
  let singleLineString = "第 1 个字符串 string"

  let quotation = """
这是多行 string 里面包含"双引号"
"Begin at the beginning," the King said gravely.
"""
}
"###;

    assert_eq!(expect, format_swift(example, false));
  }
}
