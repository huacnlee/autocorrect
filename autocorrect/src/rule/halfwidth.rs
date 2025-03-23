// autocorrect: false
use regex::Regex;
use std::{borrow::Cow, collections::HashMap};

use super::CJK_RE;

#[derive(Clone)]
enum ReplaceMode {
    Replace,
    PrefixSpace,
    SuffixSpace,
}

#[derive(Clone, PartialEq)]
enum CharType {
    LeftQuote,
    RightQuote,
    Other,
}

#[derive(Clone)]
struct ReplaceRule {
    to: char,
    mode: ReplaceMode,
    char_type: CharType,
}

impl ReplaceRule {
    fn new(to: char) -> Self {
        Self {
            to,
            mode: ReplaceMode::Replace,
            char_type: CharType::Other,
        }
    }

    fn with_suffix_space(&mut self) -> Self {
        self.mode = ReplaceMode::SuffixSpace;
        self.clone()
    }

    fn with_prefix_space(&mut self) -> Self {
        self.mode = ReplaceMode::PrefixSpace;
        self.clone()
    }

    fn left_quote(&mut self) -> Self {
        self.char_type = CharType::LeftQuote;
        self.clone()
    }

    fn right_quote(&mut self) -> Self {
        self.char_type = CharType::RightQuote;
        self.clone()
    }
}

lazy_static! {
    static ref HALF_TIME_RE: Regex = regexp!("{}", r"(\d)(：)(\d)");
    // More than 2 words and leading with words
    static ref ENGLISH_RE: Regex = regexp!("{}", r#"([\w]+[ ,.'?!&:]+[\w]+)"#);
    static ref START_WITH_WORD_RE: Regex = regexp!("{}", r#"^\s*[\w]+"#);
    static ref QUOTE_RE: Regex = regexp!("{}", r#"^\s*(["'`]).+(["'`])\s*$"#);
    static ref WORD_RE: Regex = regexp!("{}", r#"[a-zA-Z]{2,}"#);
    // %{xxx}, #{xxx}, i18n.t(
    static ref CODE_STRING_RE: Regex = regexp!("{}", r#"([#%$]\{.+\})|([\w]+\.[\w]+\()"#);

    static ref PUNCTUATION_MAP: HashMap<char, ReplaceRule> = map!(
        // The single (‘...’) and double (“...”) char is used in english typographic.
        // Option + [ and Shift + Option + [ to get “”
        // Option + ] and Shift + Option + ] to get ‘’
        // https://en.wikipedia.org/wiki/Quotation_marks_in_English

        '，' => ReplaceRule::new(',').with_suffix_space(),
        '、' => ReplaceRule::new(',').with_suffix_space(),
        '。' => ReplaceRule::new('.').with_suffix_space(),
        '：' => ReplaceRule::new(':').with_suffix_space(),
        '；' => ReplaceRule::new('.').with_suffix_space(),
        '！' => ReplaceRule::new('!').with_suffix_space(),
        '？' => ReplaceRule::new('?').with_suffix_space(),

        '（' => ReplaceRule::new('(').left_quote().with_prefix_space(),
        '【' => ReplaceRule::new('[').left_quote().with_prefix_space(),
        '「' => ReplaceRule::new('[').left_quote().with_prefix_space(),
        '《' => ReplaceRule::new('“').left_quote().with_prefix_space(),

        '）' => ReplaceRule::new(')').right_quote().with_suffix_space(),
        '】' => ReplaceRule::new(']').right_quote().with_suffix_space(),
        '」' => ReplaceRule::new(']').right_quote().with_suffix_space(),
        '》' => ReplaceRule::new('”').right_quote().with_suffix_space(),
    );
}

#[allow(dead_code)]
trait CharMatching {
    fn is_ascii_alphanumeric_punctuation(&self) -> bool;
    fn is_alphanumeric_or_space(&self) -> bool;
}

impl CharMatching for char {
    /// Match is a-z, A-Z, 0-9, all ASCII punctuations
    fn is_ascii_alphanumeric_punctuation(&self) -> bool {
        self.is_ascii_alphanumeric() || self.is_ascii_punctuation()
    }

    fn is_alphanumeric_or_space(&self) -> bool {
        self.is_ascii_alphanumeric() || matches!(self, ' ' | '\t')
    }
}

pub fn format_punctuation(text: &str) -> Cow<str> {
    // Get first non space char as quote
    let wrap_quote = text.chars().find(|c| !c.is_whitespace()).unwrap_or(' ');

    let mut changed = false;
    let lines: Vec<_> = text
        .split_inclusive('\n')
        .map(|line| match format_line(line, wrap_quote) {
            Cow::Borrowed(s) => Cow::Borrowed(s),
            Cow::Owned(s) => {
                changed = true;
                Cow::Owned(s)
            }
        })
        .collect();

    if changed {
        Cow::Owned(lines.into_iter().collect::<String>())
    } else {
        Cow::Borrowed(text)
    }
}

/// Normalize chars to use general half width in Chinese contents.
pub fn format_word(text: &str) -> Cow<str> {
    let mut changed = false;
    let out = text
        .chars()
        .map(|c| match c {
            // Unicode Fullwidth ASCII variants (Only numbers and alphabetics)
            // ０ .. ９ | Ａ .. Ｚ | ａ .. ｚ
            // https://www.unicode.org/charts/nameslist/n_FF00.html
            '\u{FF10}'..='\u{FF19}' | '\u{FF21}'..='\u{FF3A}' | '\u{FF41}'..='\u{FF5A}' => {
                changed = true;
                // checked char is in range of fullwidth number and alphabetic
                unsafe { char::from_u32_unchecked(c as u32 - 0xFEE0) }
            }
            // Ideographic Space:
            // https://en.wikipedia.org/wiki/Whitespace_character#Unicode
            '\u{3000}' => {
                changed = true;
                ' '
            }
            _ => c,
        })
        .collect::<String>();

    if changed {
        let out = HALF_TIME_RE.replace_all(&out, |cap: &regex::Captures| cap[0].replace('：', ":"));
        Cow::Owned(out.into_owned())
    } else {
        HALF_TIME_RE.replace_all(text, |cap: &regex::Captures| cap[0].replace('：', ":"))
    }
}

fn is_may_only_english(text: &str) -> bool {
    if CJK_RE.is_match(text) {
        return false;
    }

    // Characters which pass char width replacement
    if ENGLISH_RE.is_match(text) && START_WITH_WORD_RE.is_match(text) {
        // Maybe English, pass
        return true;
    }

    // In quote and including words
    if QUOTE_RE.is_match(text) && WORD_RE.is_match(text) {
        // If there not english and space or there have complex punctuation, skip
        // `${this.$t('hello')}：${items.join('，')}`, `%{foo}，hello`
        if CODE_STRING_RE.is_match(text) {
            return false;
        }

        return true;
    }

    false
}

fn format_line(text: &str, wrap_quote: char) -> Cow<str> {
    if !is_may_only_english(text) {
        return Cow::Borrowed(text);
    }

    let mut out = String::with_capacity(text.len());
    let mut changed = false;

    let mut parts = text.chars().peekable();
    while let Some(part) = parts.next() {
        // Fix punctuation without CJK contents
        if let Some(rule) = PUNCTUATION_MAP.get(&part) {
            let next_part = parts.peek();
            // Do not change left quote when is last char.
            if next_part.is_none() && rule.char_type == CharType::LeftQuote {
                out.push(part);
                continue;
            }

            match rule.mode {
                ReplaceMode::SuffixSpace => {
                    escape_quote(&mut out, wrap_quote, rule.to);
                    if next_part.map(|c| c.is_alphanumeric()).unwrap_or_default() {
                        out.push(' ');
                    }
                }
                ReplaceMode::PrefixSpace => {
                    let last_part = out.chars().last();
                    if last_part.map(|c| c.is_alphanumeric()).unwrap_or_default() {
                        out.push(' ');
                    }
                    escape_quote(&mut out, wrap_quote, rule.to);
                }
                ReplaceMode::Replace => {
                    escape_quote(&mut out, wrap_quote, rule.to);
                }
            }
            changed = true;
        } else {
            out.push(part);
        }
    }
    if changed {
        Cow::Owned(out)
    } else {
        Cow::Borrowed(text)
    }
}

fn escape_quote(out: &mut String, wrap_quote: char, quote: char) {
    if quote != '"' && quote != '\'' || wrap_quote != quote {
        out.push(quote);
    } else {
        out.push('\\');
        out.push(quote);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn assert_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = format_punctuation(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn test_halfwidth_alphabetic_numbers() {
        let source = "测试:ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ１２３４５６７８９０";
        assert_eq!(
            "测试:abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
            format_word(source)
        );

        assert_eq!(
            "他说：我们将在16:32分出发去CBD中心。",
            format_word("他说：我们将在１６：３２分出发去ＣＢＤ中心。")
        );

        // Fullwidth space
        assert_eq!(
            "ジョイフル－後場売り気配 200 店舗を閉鎖へ 7 月以降、不採算店中心に",
            format_word("ジョイフル－後場売り気配　200 店舗を閉鎖へ　7 月以降、不採算店中心に")
        );
    }

    #[test]
    fn test_halfwidth_punctuation_ignores() {
        let cases = map! [
            "。" => "。",
            "，" => "，",
            "SHA1。" => "SHA1。",
            "a。" => "a。",
            "foo-bar-dar。" => "foo-bar-dar。",
            "hello)。" => "hello)。",
            "说：你好 english。" => "说：你好 english。",
            "‘腾讯’ - 发布 - ‘新版’本微信" => "‘腾讯’ - 发布 - ‘新版’本微信",
            "${item.name}（ID ${item.id}）" => "${item.name}（ID ${item.id}）",
            "{{ t('name') }}：{{ item.extraKeys.join(' | ') }}" => "{{ t('name') }}：{{ item.extraKeys.join(' | ') }}",
            "The Exchange’s" => "The Exchange’s",
            "It's revenue \"conditions\" among the suppliers’ “customers”" => "It's revenue \"conditions\" among the suppliers’ “customers”",
        ];
        assert_cases(cases);
    }

    #[test]
    fn test_halfwidth_punctuation() {
        let cases = map! [
            "hello。" => "hello。",
            "hello 你好。" => "hello 你好。",
            "中文1\nhello world。\n中文2" => "中文1\nhello world.\n中文2",
            "  \n  Said：Come and，Join us！  \n  " => "  \n  Said: Come and, Join us!  \n  ",
            "Said：Come and，Join us！" => "Said: Come and, Join us!",
            "_（HTML5 Rocks）_" => "_(HTML5 Rocks)_",
            "  Start with space next word？Join us?" => "  Start with space next word? Join us?",
            ", Not start with word will not change。" => ", Not start with word will not change。",
            "：“Not start with word will not change”" => "：“Not start with word will not change”",
            "Come and， Join us！" => "Come and, Join us!",
            "The microphone or camera is occupied，Please check and re-record the video。" => "The microphone or camera is occupied, Please check and re-record the video.",
            "The “Convertible Amount” case。" => r#"The “Convertible Amount” case."#,
            "The“Convertible Amount”case。" => r#"The“Convertible Amount”case."#,
            "The（Convertible Amount）case！" => r#"The (Convertible Amount) case!"#,
            "The【Convertible Amount】case？" => "The [Convertible Amount] case?",
            "The「Convertible Amount」case：" => "The [Convertible Amount] case:",
            "The《Convertible Amount》case，" => r#"The “Convertible Amount” case,"#,
            "Reason: CORS header ‘Origin’ cannot be added" => "Reason: CORS header ‘Origin’ cannot be added",
        ];

        assert_cases(cases);
    }

    #[test]
    fn test_ignore_left_quote_in_last() {
        let cases = map! [
            "Escher puzzle (" => "Escher puzzle (",
            "Escher puzzle【" => "Escher puzzle【",
            "Escher puzzle《" => "Escher puzzle《",
            "Escher puzzle“" => "Escher puzzle“",
            "Escher puzzle‘" => "Escher puzzle‘",
            "Escher puzzle「" => "Escher puzzle「",
        ];

        assert_cases(cases);
    }

    #[test]
    fn test_halfwidth_punctuation_with_in_quote() {
        let cases = map! [
            r#""，""# => r#""，""#,
            r#""。""# => r#""。""#,
            r#""a。""# => r#""a。""#,
            r#""Hi！""# => r#""Hi!""#,
            r#""hello-world。""# => r#""hello-world.""#,
            r#"'hello “world”。'"# => r#"'hello “world”.'"#,
            r#""hello “world”。""# => r#""hello “world”.""#,
            r#""hello ‘world’。""# => r#""hello ‘world’.""#,
            r#"'hello ‘world’。'"# => r#"'hello ‘world’.'"#,
            r#""Only the first time break。""# => r#""Only the first time break.""#,
            r#"'Only the first time break？'"# => r#"'Only the first time break?'"#,
            r#"`Only the first time break！`"# => r#"`Only the first time break!`"#,
            r#"`${this.$t('hello')}：${items.join('，')}`"# => r#"`${this.$t('hello')}：${items.join('，')}`"#,
            r#"`${t('hello')}：${user.name}`"# => r#"`${t('hello')}：${user.name}`"#,
            r##""#{vars.join("，")}""## => r##""#{vars.join("，")}""##
        ];

        assert_cases(cases);
    }
}
