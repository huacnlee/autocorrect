// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/php.pest"]
struct PHPParser;

#[allow(dead_code)]
pub fn format_php(text: &str, lint: bool) -> String {
  let pairs = PHPParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
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
  }
  ?>
</div>
"###;

    assert_eq!(expect, format_php(example, false));
  }
}
