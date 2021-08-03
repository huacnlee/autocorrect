// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/css.pest"]
struct CSSParser;

#[allow(dead_code)]
pub fn format_css(text: &str) -> code::FormatResult {
  let pairs = CSSParser::parse(Rule::item, text);
  let text = code::FormatResult::new(text);
  return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_css(text: &str) -> code::LintResult {
  let pairs = CSSParser::parse(Rule::item, text);
  let text = code::LintResult::new(text);
  return code::format_pairs(text, pairs);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_css_pair() {
    let example = r###"
// 这是comment在CSS里面

/* 
  这是多行CSS第1行
  这是第2行
*/
.btn /* 多行comment
在CSS元素中间
*/ {
  .strong { font-weight: bold; }
  padding: 10px; // comment在属性后面
  font: Helvetica, // comment在属性后面，后面还有
    sans-serif;
}
"###;

    let expect = r###"
// 这是 comment 在 CSS 里面

/* 
  这是多行 CSS 第 1 行
  这是第 2 行
*/
.btn /* 多行 comment
在 CSS 元素中间
*/ {
  .strong { font-weight: bold; }
  padding: 10px; // comment 在属性后面
  font: Helvetica, // comment 在属性后面，后面还有
    sans-serif;
}
"###;

    assert_eq!(expect, format_css(example).to_string());
  }
}
