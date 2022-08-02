// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/csharp.pest"]
struct CSharpParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

        assert_eq!(expect, format_for(example, "csharp").to_string());
    }
}
