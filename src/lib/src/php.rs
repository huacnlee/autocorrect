// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/php.pest"]
struct PHPParser;

#[allow(dead_code)]
pub fn format_php(text: &str) -> code::FormatResult {
    let pairs = PHPParser::parse(Rule::item, text);
    let text = code::FormatResult::new(text);
    return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_php(text: &str) -> code::LintResult {
    let pairs = PHPParser::parse(Rule::item, text);
    let text = code::LintResult::new(text);
    return code::format_pairs(text, pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_format_php() {
        let example = r###"
<div class="container">
  <p>目前html tag里的无法处理</p>
  <?php
  /** 
   * 第1行注释
   * 第2行注释
   */
  class HelloWorld {
      // 这是第3行注释
      var singleLineString: String = "单行string测试"
      var multilineString: String = "多行string测试
      第2行字符串"

      var re0 = preg_match( "re1正则", singleLineString )
      var re1 = preg_match_all("re2正则" ,  multilineString )
  }
  ?>
</div>
"###;

        let expect = r###"
<div class="container">
  <p>目前html tag里的无法处理</p>
  <?php
  /** 
   * 第 1 行注释
   * 第 2 行注释
   */
  class HelloWorld {
      // 这是第 3 行注释
      var singleLineString: String = "单行 string 测试"
      var multilineString: String = "多行 string 测试
      第 2 行字符串"

      var re0 = preg_match( "re1正则", singleLineString )
      var re1 = preg_match_all("re2正则" ,  multilineString )
  }
  ?>
</div>
"###;

        assert_eq!(expect, format_php(example).to_string());
    }
}
