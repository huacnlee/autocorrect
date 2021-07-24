// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/ruby.pest"]
struct RubyParser;

#[allow(dead_code)]
pub fn format_ruby(text: &str) -> code::FormatResult {
  let pairs = RubyParser::parse(Rule::item, text);
  let text = code::FormatResult::new(text);
  return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_ruby(text: &str) -> code::LintResult {
  let pairs = RubyParser::parse(Rule::item, text);
  let text = code::LintResult::new(text);
  return code::format_pairs(text, pairs);
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
  re = /hello你好/
  re1 = %r{hello你好}
  re2 = Regexp.new('hello你好' )
  re3 = Regexp.new( "hello你好")

  a = "hello世界#{a}"
  b = '你好hello世界'
end
"###;

    let expect = r###"
# 第 1 行注释
# 第 2 行注释
def hello(a, b: "第 1 个参数")
  re = /hello你好/
  re1 = %r{hello你好}
  re2 = Regexp.new('hello你好' )
  re3 = Regexp.new( "hello你好")

  a = "hello 世界#{a}"
  b = '你好 hello 世界'
end
"###;

    assert_eq!(expect, format_ruby(example).to_string());
  }
}
