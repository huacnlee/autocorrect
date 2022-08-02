// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/elixir.pest"]
struct ElixirParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_elixir() {
        let example = r###"
defmodule Test do
  @moduledoc """
  多行注释第1行
  multiline comment第2行
  """

  def hello do
    # 单行comment注释
    str1 = "hello你好双引号"
    str2 = 'hello你好单引号'
    str3 = ~s(hello你好)
    str4 = ~c(hello你好)

    multiline_str = ~S"""
    多行字符串第1行
    多行string第2行
    """

    pattern1 = ~r/hello正则/
    pattern2 = Regex.compile("hello正则")
  end
end
"###;

        let expect = r###"
defmodule Test do
  @moduledoc """
  多行注释第 1 行
  multiline comment 第 2 行
  """

  def hello do
    # 单行 comment 注释
    str1 = "hello 你好双引号"
    str2 = 'hello 你好单引号'
    str3 = ~s(hello 你好)
    str4 = ~c(hello 你好)

    multiline_str = ~S"""
    多行字符串第 1 行
    多行 string 第 2 行
    """

    pattern1 = ~r/hello正则/
    pattern2 = Regex.compile("hello正则")
  end
end
"###;

        assert_eq!(expect, format_for(example, "elixir").to_string());
    }
}
