// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/python.pest"]
struct PythonParser;

pub fn format_python(text: &str) -> String {
  let result = PythonParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_python_pair(&mut out, item);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_python_pair(text: &mut String, item: Pair<Rule>) {
  match item.as_rule() {
    Rule::string | Rule::comment => text.push_str(format(item.as_str()).as_str()),
    Rule::item => {
      for sub in item.into_inner() {
        format_python_pair(text, sub);
      }
    }
    _ => text.push_str(item.as_str()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_python() {
    let example = r###"
'''
这是多行1注释
这是多行2注释
这是多行3注释
'''
def hello(a):
  multi_str = """
  第1行多行字符串
  第2行多行字符串
  """

  # 第4个注释
  print("你好hello世界")
  print('你好hello世界')
"###;

    let expect = r###"
'''
这是多行 1 注释
这是多行 2 注释
这是多行 3 注释
'''
def hello(a):
  multi_str = """
  第 1 行多行字符串
  第 2 行多行字符串
  """

  # 第 4 个注释
  print("你好 hello 世界")
  print('你好 hello 世界')
"###;

    assert_eq!(expect, format_python(example));
  }
}
