// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;
#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/python.pest"]
struct PythonParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

  re = r'包含#regexp测试'
  re1 = r"""
    包含re0测试
    包含re1测试
  """
  re2 = re.compile( "hello你" + "world好")

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

  re = r'包含#regexp测试'
  re1 = r"""
    包含re0测试
    包含re1测试
  """
  re2 = re.compile( "hello你" + "world好")

  # 第 4 个注释
  print("你好 hello 世界")
  print('你好 hello 世界')
"###;

        assert_eq!(expect, format_for(example, "python").to_string());
    }
}
