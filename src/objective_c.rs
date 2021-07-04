// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/objective_c.pest"]
struct ObjectiveCParser;

#[allow(dead_code)]
pub fn format_objective_c(text: &str, lint: bool) -> String {
  let pairs = ObjectiveCParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_objective_c() {
    let example = r###"
// 第1行注释
// 第2行注释
- (void) helloWorld {
  // 第3行注释
  NSString *geotestUrl = @"第1个字符串string";
}
"###;

    let expect = r###"
// 第 1 行注释
// 第 2 行注释
- (void) helloWorld {
  // 第 3 行注释
  NSString *geotestUrl = @"第 1 个字符串 string";
}
"###;

    assert_eq!(expect, format_objective_c(example, false));
  }
}
