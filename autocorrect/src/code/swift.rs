// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/swift.pest"]
struct SwiftParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

  NSLocalizedString("hello世界不会修改", nil)

  NSLocalizedString(
    "hello世界不会修改", nil)

  let val = try! NSLocalizedString(key: "key名称不会处理")

  let re = try! NSRegularExpression(pattern:    "re正则")
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

  NSLocalizedString("hello世界不会修改", nil)

  NSLocalizedString(
    "hello世界不会修改", nil)

  let val = try! NSLocalizedString(key: "key名称不会处理")

  let re = try! NSRegularExpression(pattern:    "re正则")
}
"###;

        assert_eq!(expect, format_for(example, "swift").to_string());
    }
}
