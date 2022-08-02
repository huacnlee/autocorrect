// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/dart.pest"]
struct DartParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_dart() {
        let example = r###"
/** 
 * 第1行注释
 * 第2行注释
 */
String helloWorld(String name) {
  // 第3行注释
  var singleLineString = "第1个字符串string";
  var singleLineString = '第2个字符串string';

  var quotation = """
  这是第3行字符串
  这是第4行
  """;

  let quotation = '''
  这是第5行字符串
  这是第6行
  ''';

  let re0 = r"re正则"
  let re1 = r're正则'
}
"###;

        let expect = r###"
/** 
 * 第 1 行注释
 * 第 2 行注释
 */
String helloWorld(String name) {
  // 第 3 行注释
  var singleLineString = "第 1 个字符串 string";
  var singleLineString = '第 2 个字符串 string';

  var quotation = """
  这是第 3 行字符串
  这是第 4 行
  """;

  let quotation = '''
  这是第 5 行字符串
  这是第 6 行
  ''';

  let re0 = r"re正则"
  let re1 = r're正则'
}
"###;

        assert_eq!(expect, format_for(example, "dart").to_string());
    }
}
