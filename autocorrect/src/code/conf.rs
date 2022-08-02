// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/conf.pest"]
struct ConfParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_conf() {
        let example = r###"
# 這是一份TOML文件
title = "TOML範例"

[servers]

  # 可以使用縮排TAB或是空格,但不是必要的
  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"
"###;

        let expect = r###"
# 這是一份 TOML 文件
title = "TOML範例"

[servers]

  # 可以使用縮排 TAB 或是空格，但不是必要的
  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"
"###;

        assert_eq!(expect, format_for(example, "conf").to_string());
    }
}
