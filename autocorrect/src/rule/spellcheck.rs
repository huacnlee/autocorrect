use std::collections::HashMap;

use crate::{config::Config, keyword::MatchedResult};

pub(crate) fn word_regexp(word: &str) -> regex::Regex {
    let prefix = r#"([^\W]|[\p{Han}？！：，。；、]|$|^)"#;

    regexp!(
        r#"(?im){}([\s？！：，。；、]|^)+({})([\s？！：，。；、]|$)+{}"#,
        prefix,
        word.replace('-', r"\-").replace('.', r"\."),
        prefix
    )
}

lazy_static! {
    static ref DISALLOW_CHAR_RE: regex::Regex = regexp!("{}", r#"([^\p{Han}\s？！：，。；、])"#);
}

// Spell check by dict
pub fn format(text: &str) -> String {
    let mut out = String::from(text);

    let config = Config::current();

    let word_map = &config.spellcheck.word_map;
    let word_re = &config.spellcheck.word_re;
    let matcher = &config.spellcheck.matcher;

    let matched_words = matcher.match_keywords(text);
    return replace_with_spans(text, &matched_words, word_map);
}

struct SpanInfo {
    old: String,
    new: String,
    span: crate::keyword::Span,
}

fn replace_with_spans(
    text: &str,
    words: &MatchedResult,
    word_map: &HashMap<String, String>,
) -> String {
    let mut spans = words
        .iter()
        .map(|(w, spans)| {
            spans.iter().map(move |span| SpanInfo {
                old: w.to_string(),
                new: word_map.get(w).unwrap().to_string(),
                span: span.clone(),
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    spans.sort_by(|a, b| a.span.start.cmp(&b.span.start));

    let mut text_chars = text.chars().collect::<Vec<_>>();
    let mut offset_change = 0;

    for span_info in spans.iter() {
        let old_str = &span_info.old;
        let new_str = &span_info.new;
        let span_start = span_info.span.start;
        let span_end = span_info.span.end;

        let offset_start = span_start + offset_change;
        let offset_end = span_end + offset_change;

        // Check whether the left and right 1 characters are allowed
        let left_start = offset_start as isize - 1;
        let left_start = if left_start < 0 {
            0
        } else {
            left_start as usize
        };
        let left_str: String = text_chars
            .get(left_start..offset_start)
            .unwrap_or_default()
            .iter()
            .collect();
        let right_str: String = text_chars
            .get(offset_end..offset_end + 1)
            .unwrap_or_default()
            .iter()
            .collect();

        println!(
            "----------------- l: `{}` {}..{}, r: `{}`",
            left_str, left_start, offset_start, right_str
        );

        if DISALLOW_CHAR_RE.is_match(&left_str) || DISALLOW_CHAR_RE.is_match(&right_str) {
            // println!("------- not allow:{}", old_str);
            continue;
        }

        // Perform replacement
        let new_str_chars = new_str.chars().collect::<Vec<_>>();
        text_chars.splice(offset_start..offset_end, new_str_chars);

        // Update offset_change due to length change after replacement
        offset_change += new_str.chars().count() - old_str.chars().count();
    }

    text_chars.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::Config;

    use super::*;
    use std::collections::HashMap;

    #[track_caller]
    fn assert_spellcheck_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = format(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn test_disallow_char_re() {
        assert_eq!(DISALLOW_CHAR_RE.is_match(","), true);
        assert_eq!(DISALLOW_CHAR_RE.is_match("a"), true);
        assert_eq!(DISALLOW_CHAR_RE.is_match("-"), true);
        assert_eq!(DISALLOW_CHAR_RE.is_match("a你"), true);
        assert_eq!(DISALLOW_CHAR_RE.is_match("a\n"), true);
        assert_eq!(DISALLOW_CHAR_RE.is_match("a "), true);

        assert_eq!(DISALLOW_CHAR_RE.is_match(""), false);
        assert_eq!(DISALLOW_CHAR_RE.is_match("你 "), false);
        assert_eq!(DISALLOW_CHAR_RE.is_match("你好"), false);
        assert_eq!(DISALLOW_CHAR_RE.is_match("你，"), false);
        assert_eq!(DISALLOW_CHAR_RE.is_match(" ，"), false);
        assert_eq!(DISALLOW_CHAR_RE.is_match("？"), false);
    }

    #[test]
    fn test_spellcheck_basic() {
        crate::config::setup_test();

        let cases = map! [
            "ios" => "iOS",
            "this is ipad ios website, and the IOS download url" => "this is iPad iOS website, and the iOS download URL",
            "Ios download" => "iOS download",
            "Download iOs" => "Download iOS",
            "hello_ios" => "hello_ios",
            "ios_hello" => "ios_hello",
            "'ios'" => "'ios'",
            "openios" => "openios",
            "diff_ws_ios\n" => "diff_ws_ios\n",
            "diff_ws_ios " => "diff_ws_ios ",
            "this-is-ios" => "this-is-ios",
            "[1]: https://example.com/xxx/yyy/zzz-ios" => "[1]: https://example.com/xxx/yyy/zzz-ios",
            "https://ios.com" => "https://ios.com",
            "support@ios.com" => "support@ios.com",
            "开放 IOS 接口" => "开放 iOS 接口",
            "开放接口 IOS。" => "开放接口 iOS。",
            "开放接口 IOS？" => "开放接口 iOS？",
            "开放接口 IOS！" => "开放接口 iOS！",
            "开放接口 IOS，" => "开放接口 iOS，",
            "开放，ios 接口" => "开放，iOS 接口",
            "打开 wifi 并找到就近的 WIFI，点击输入 wi-fi 密码" => "打开 Wi-Fi 并找到就近的 Wi-Fi，点击输入 Wi-Fi 密码"
        ];

        assert_spellcheck_cases(cases);
    }

    #[test]
    fn test_spellcheck_for_special_cases() {
        crate::config::setup_test();

        let cases = map! [
            "var ios = '1.0.0'" => "var ios = '1.0.0'",
            "let wifi = ios" => "let wifi = ios",
            "ipad + ios" => "ipad + ios",
            "html { color: #999; }" => "html { color: #999; }",
            "> IOS" => "> IOS",
            "ios => {}" => "ios => {}",
            "if ios > 0" => "if ios > 0",
            r#""IOS""# => r#""IOS""#,
            r#"'IOS'"# => r#"'IOS'"#,
            r#""IOS 11""# => r#""IOS 11""#,
            r#"key: "ios", value: "ipad""# => r#"key: "ios", value: "ipad""#,
        ];

        assert_spellcheck_cases(cases);
    }

    #[test]
    fn test_spellcheck_all() {
        crate::config::setup_test();

        let words = Config::current().spellcheck.words.clone();
        for l in words.iter() {
            let left = l.as_str();
            let right = l.as_str();

            assert_eq!(right, format(left));
            assert_eq!(right, format(&left.to_uppercase()));
            assert_eq!(right, format(&left.to_lowercase()));
        }
    }
}
