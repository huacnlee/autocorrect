// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/swift.pest"]
struct SwiftParser;

#[allow(dead_code)]
pub fn format_swift(text: &str) -> code::FormatResult {
    let pairs = SwiftParser::parse(Rule::item, text);
    let text = code::FormatResult::new(text);
    return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_swift(text: &str) -> code::LintResult {
    let pairs = SwiftParser::parse(Rule::item, text);
    let text = code::LintResult::new(text);
    return code::format_pairs(text, pairs);
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

        assert_eq!(expect, format_swift(example).to_string());
    }
}
