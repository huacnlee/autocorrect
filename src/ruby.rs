// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/ruby.pest"]
struct RubyParser;

#[allow(dead_code)]
pub fn format_ruby(text: &str, lint: bool) -> String {
  let pairs = RubyParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_ruby() {
    let example = r###"
# 第1行注释
# 第2行注释
def hello(a, b: "第1个参数")
  a = "hello世界#{a}"
  b = '你好hello世界'
end
"###;

    let expect = r###"
# 第 1 行注释
# 第 2 行注释
def hello(a, b: "第 1 个参数")
  a = "hello 世界#{a}"
  b = '你好 hello 世界'
end
"###;

    assert_eq!(expect, format_ruby(example, false));
  }
}
