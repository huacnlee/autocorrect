// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/css.pest"]
struct CSSParser;

pub fn format_css(text: &str, lint: bool) -> String {
  let result = CSSParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_css_pair(&mut out, item, lint);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_css_pair(text: &mut String, item: Pair<Rule>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  match item.as_rule() {
    Rule::comment => format_or_lint(text, part, true, lint, line, col),
    Rule::item => {
      for sub in item.into_inner() {
        format_css_pair(text, sub, lint);
      }
    }
    _ => format_or_lint(text, part, true, lint, line, col),
  }
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

    assert_eq!(expect, format_css(example, false));
  }
}
