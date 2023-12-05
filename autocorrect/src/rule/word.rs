// autocorrect: false
use super::{strategery::Strategery, CJK_RE};

lazy_static! {
    // Strategies all rules
    static ref WORD_STRATEGIES: Vec<Strategery> = vec![
        // EnglishLetter, Number
        // Avoid add space when Letter, Number has %, $, \ prefix, eg. %s, %d, $1, $2, \1, \2, \d, \r, \p ... in source code
        Strategery::new(r"\p{CJK}[^%\$\\]", r"[a-zA-Z0-9]"),
        Strategery::new(r"[^%\$\\][a-zA-Z0-9]", r"\p{CJK}"),
        // Number, -100, +100
        Strategery::new(r"\p{CJK}", r"[\-+][\d]+").with_reverse(),
        // Spcial format Letter, Number leading case, because the before Strategery can't cover eg. A开头的case测试
        Strategery::new(r"^[a-zA-Z0-9]", r"\p{CJK}"),
        // 10%中文
        Strategery::new(r"[0-9][%]", r"\p{CJK}"),
        // 300+单词，A+评分，C++中文，C#中文, 100#中文
        // The `#` can'not work, because is related to URL anchor, can't do it.
        Strategery::new(r"[a-zA-Z0-9][+#]+", r"\p{CJK}"),
    ];

    static ref PUNCTUATION_STRATEGIES: Vec<Strategery> = vec![
        // SpecialSymbol
        Strategery::new(r"[\p{CJK_N}”’]", r"[\-\|+][\p{CJK_N}\s（【「《“‘]"),
        Strategery::new(r"[\p{CJK_N}\s）】」”’》][\-\|+]", r"[\p{CJK_N}“‘]"),
        Strategery::new(r"[!]", r"\p{CJK}"),
    ];

    static ref BRACKETS_STRATEGIES: Vec<Strategery> = vec![
        // Add space before and after brackets [] or () near the CJK
        Strategery::new(r"\p{CJK}", r"[\[\(]"),
        Strategery::new(r"[\]\)]", r"\p{CJK}"),
    ];

    static ref BACKTICKS_STRATEGIES: Vec<Strategery> = vec![
        // Add space before and after backtick ` near the CJK
        Strategery::new(r"\p{CJK}", r"`.+`"),
        Strategery::new(r"`.+`", r"\p{CJK}"),
    ];

    static ref DASH_RE : regex::Regex = regexp!(r"([\p{CJK}])[\-]([\p{CJK}])");

    static ref NO_SPACE_FULLWIDTH_STRATEGIES: Vec<Strategery> = vec![
        // FullwidthPunctuation remove space case, Fullwidth can safe to remove spaces
        Strategery::new(r"\w|\p{CJK}|`", r"[，。、！？：；（）「」《》【】]").with_remove_space().with_reverse(),
    ];

    static ref NO_SPACE_FULLWIDTH_QUOTE_STRATEGIES : Vec<Strategery> = vec![
        // Remove space around fullwidth quotes
        Strategery::new(r"\w|\p{CJK}", r"[“”‘’]").with_remove_space().with_reverse(),
    ];
}

pub fn format_space_word(input: &str) -> String {
    let mut out = String::from(input);
    WORD_STRATEGIES.iter().for_each(|s| out = s.format(&out));
    out
}

pub fn format_space_punctuation(input: &str) -> String {
    let mut out = String::from(input);
    PUNCTUATION_STRATEGIES
        .iter()
        .for_each(|s| out = s.format(&out));
    out
}

pub fn format_space_bracket(input: &str) -> String {
    let mut out = String::from(input);
    BRACKETS_STRATEGIES
        .iter()
        .for_each(|s| out = s.format(&out));
    out
}

pub fn format_space_dash(input: &str) -> String {
    DASH_RE
        .replace_all(input, |cap: &regex::Captures| {
            format!("{} - {}", &cap[1], &cap[2])
        })
        .to_string()
}

pub fn format_space_backticks(input: &str) -> String {
    let mut out = String::from(input);
    BACKTICKS_STRATEGIES
        .iter()
        .for_each(|s| out = s.format(&out));
    out
}

pub fn format_no_space_fullwidth(input: &str) -> String {
    let mut out = String::from(input);

    if !CJK_RE.is_match(input) {
        return out;
    }

    NO_SPACE_FULLWIDTH_STRATEGIES
        .iter()
        .for_each(|s| out = s.format(&out));
    out
}

pub fn format_no_space_fullwidth_quote(input: &str) -> String {
    let mut out = String::from(input);

    if !CJK_RE.is_match(input) {
        return out;
    }

    NO_SPACE_FULLWIDTH_QUOTE_STRATEGIES
        .iter()
        .for_each(|s| out = s.format(&out));
    out
}

#[cfg(test)]
mod tests {
    use crate::rule::word::{format_space_backticks, format_space_dash};

    #[test]
    fn test_format_space_bracket() {
        assert_eq!(format_space_dash("你好-世界"), "你好 - 世界");
        assert_eq!(format_space_dash("foo-世界"), "foo-世界");
        assert_eq!(format_space_dash("你好-world"), "你好-world");
        assert_eq!(format_space_dash("hello-world"), "hello-world");
    }

    #[test]
    fn test_format_space_backticks() {
        assert_eq!(format_space_backticks("代码`code`"), "代码 `code`");
        assert_eq!(format_space_backticks("代码`code`代码"), "代码 `code` 代码");
        assert_eq!(
            format_space_backticks("`code`代码`code`"),
            "`code` 代码 `code`"
        );
        assert_eq!(
            format_space_backticks("`code`hello`code`"),
            "`code`hello`code`"
        );

        assert_eq!(format_space_backticks("```rs"), "```rs");
        assert_eq!(format_space_backticks("``代码第1行"), "``代码第1行");
        assert_eq!(format_space_backticks("`代码第1行"), "`代码第1行");
        assert_eq!(format_space_backticks("代码第2行`"), "代码第2行`");
    }
}
