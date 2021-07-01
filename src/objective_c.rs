// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/objective_c.pest"]
struct ObjectiveCParser;

pub fn format_objective_c(text: &str) -> String {
  let result = ObjectiveCParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_objective_c_pair(&mut out, item);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_objective_c_pair(text: &mut String, item: Pair<Rule>) {
  match item.as_rule() {
    Rule::string | Rule::comment => text.push_str(format(item.as_str()).as_str()),
    Rule::item => {
      for sub in item.into_inner() {
        format_objective_c_pair(text, sub);
      }
    }
    _ => text.push_str(item.as_str()),
  }
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

    assert_eq!(expect, format_objective_c(example));
  }
}
