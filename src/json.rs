// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/json.pest"]
struct JSONParser;

#[allow(dead_code)]
pub fn format_json(text: &str, lint: bool) -> String {
  let pairs = JSONParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_json() {
    let example = r###"
{
  "name": "你好hello世界",
  "displayName": "JSON格式测试",
  "publisher": "huacnlee",
  "meta": {
    // 第1行注释
    "title": "第1个meta", 
    /** 
     * 第2行注释
     * 第3行注释
     */
    "description": "第2个meta", 
    "测试key不格式化": false
  }
}
"###;

    let expect = r###"
{
  "name": "你好 hello 世界",
  "displayName": "JSON 格式测试",
  "publisher": "huacnlee",
  "meta": {
    // 第 1 行注释
    "title": "第 1 个 meta", 
    /** 
     * 第 2 行注释
     * 第 3 行注释
     */
    "description": "第 2 个 meta", 
    "测试key不格式化": false
  }
}
"###;

    assert_eq!(expect, format_json(example, false));
  }
}
