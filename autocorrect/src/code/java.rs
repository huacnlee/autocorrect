// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/java.pest"]
struct JavaParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_java() {
        let example = r###"
// 第1行注释
// 第2行注释
public String helloWorld() {
  // 第3行注释
  String singleLineString = "第1个字符串string"

  Pattern re0 = Pattern.compile("re正则" );
  Pattern.matches( "re1正则" , "foobar你好");

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

  Pattern re0 = Pattern.compile("re正则" );
  Pattern.matches( "re1正则" , "foobar你好");

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

        assert_eq!(expect, format_for(example, "java").to_string());
    }
}
