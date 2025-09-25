// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/lua.pest"]
struct LuaParser;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_lua() {
        let example = indoc! {r###"
        -- 第1行注释
        -- 第2行注释
        function hello(a)
          re = string.find("hello你好")
          
          a = "hello世界"
          b = '你好hello世界'
          c = [[多行
          字符串]]
        end
        "###};

        let expect = indoc! {r###"
        -- 第 1 行注释
        -- 第 2 行注释
        function hello(a)
          re = string.find("hello你好")
          
          a = "hello 世界"
          b = '你好 hello 世界'
          c = [[多行
          字符串]]
        end
        "###};

        assert_eq!(expect, format_for(example, "lua").to_string());
    }
}