// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/css.pest"]
struct CSSParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

        assert_eq!(expect, format_for(example, "css").to_string());
    }
}
