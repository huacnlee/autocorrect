// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/python.pest"]
struct PythonParser;

#[allow(dead_code)]
pub fn format_python(text: &str) -> code::FormatResult {
    let pairs = PythonParser::parse(Rule::item, text);
    let text = code::FormatResult::new(text);
    return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_python(text: &str) -> code::LintResult {
    let pairs = PythonParser::parse(Rule::item, text);
    let text = code::LintResult::new(text);
    return code::format_pairs(text, pairs);
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

        assert_eq!(expect, format_python(example).to_string());
    }
}
