// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/gettext.pest"]
struct GettextParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_gettext() {
        let example = r###"
msgid "这是msgid"
msgstr "这是msgid"

"Project-Id-Version: Admin \n"
"Report-Msgid-Bugs-To: \n"

#: ref:620e039d1a4c6d48680001bd
msgid "请输入Email地址:"
msgstr "请输入Email地址:"
msgstr[0] "这是msgstr0"
msgstr[1] "这是msgstr1"

# 这是Commit评论.
msgctxt "Foo"
msgid_plural "密码长度要求最少6个字符."
msgstr "密码长度要求最少6个字符."
"###;

        let expect = r###"
msgid "这是msgid"
msgstr "这是 msgid"

"Project-Id-Version: Admin \n"
"Report-Msgid-Bugs-To: \n"

#: ref:620e039d1a4c6d48680001bd
msgid "请输入Email地址:"
msgstr "请输入 Email 地址："
msgstr[0] "这是 msgstr0"
msgstr[1] "这是 msgstr1"

# 这是 Commit 评论。
msgctxt "Foo"
msgid_plural "密码长度要求最少6个字符."
msgstr "密码长度要求最少 6 个字符。"
"###;

        assert_eq!(expect, format_for(example, "gettext").to_string());
    }
}
