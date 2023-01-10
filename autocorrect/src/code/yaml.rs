// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/yaml.pest"]
struct YAMLParser;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_yaml() {
        let example = indoc! {r###"
        # 这是line_comment第1行
        foo: 'hello世界'
        region:
          cn-north-1
        "en":
          name: "你好Hello世界"
          foo: Bar
          dar:
            - foo: 1
            - bar: "数字2"
          "abc字段": abc字段
        "###};

        let expect = indoc! {r###"
        # 这是 line_comment 第 1 行
        foo: 'hello 世界'
        region:
          cn-north-1
        "en":
          name: "你好 Hello 世界"
          foo: Bar
          dar:
            - foo: 1
            - bar: "数字 2"
          "abc字段": abc 字段
        "###};

        assert_eq!(expect, format_for(example, "yaml").to_string())
    }
}
