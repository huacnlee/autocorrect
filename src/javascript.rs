// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/javascript.pest"]
struct JavaScriptParser;

#[allow(dead_code)]
pub fn format_javascript(text: &str, lint: bool) -> String {
  let pairs = JavaScriptParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_javascript() {
    let example = r###"
// 第 1 行注释
// 第 2 行注释
function helloWorld(a) {
  const a = '第 1 个';
  const b = "第 2 个" + "第 3 个";
}
"###;

    let expect = r###"
// 第 1 行注释
// 第 2 行注释
function helloWorld(a) {
  const a = '第 1 个';
  const b = "第 2 个" + "第 3 个";
}
"###;

    assert_eq!(expect, format_javascript(example, false));
  }
}
