use std::collections::HashMap;

use crate::{config::Config, keyword::MatchedResult};

lazy_static! {
    static ref DISALLOW_CHAR_RE: regex::Regex =
        regexp!("{}", r#"([^\p{Han}\s？！：，。；、「」“”‘’【】《》])"#);
}

// Spell check by dict
pub fn format(text: &str) -> String {
    let config = Config::current();

    let word_map = &config.spellcheck.word_map;
    let matcher = &config.spellcheck.matcher;

    let matched_words = matcher.match_keywords(text);
    replace_with_spans(text, &matched_words, word_map)
}

#[derive(Debug)]
struct SpanInfo<'a> {
    old: &'a String,
    new: &'a String,
    span: &'a crate::keyword::Span,
}

fn replace_with_spans(
    text: &str,
    words: &MatchedResult,
    word_map: &HashMap<String, String>,
) -> String {
    let mut span_infos = vec![];

    for (old, spans) in words {
        if let Some(new) = word_map.get(old) {
            for span in spans {
                span_infos.push(SpanInfo { old, new, span })
            }
        }
    }

    span_infos.sort_by(|a, b| a.span.start.cmp(&b.span.start));

    let mut text_chars = text.chars().collect::<Vec<_>>();
    let mut offset_change = 0;

    for span_info in span_infos.iter() {
        let old_str = span_info.old;
        let new_str = span_info.new;
        let span_start = span_info.span.start;
        let span_end = span_info.span.end;

        let offset_start = span_start + offset_change;
        let offset_end = span_end + offset_change;

        // Check whether the left and right 1 characters are allowed
        // If not allowed, skip this replacement
        let l_c = if offset_start == 0 {
            None
        } else {
            text_chars.get(offset_start - 1)
        };
        let r_c = text_chars.get(offset_end);

        if DISALLOW_CHAR_RE.is_match(&l_c.unwrap_or(&' ').to_string())
            || DISALLOW_CHAR_RE.is_match(&r_c.unwrap_or(&' ').to_string())
        {
            // println!("---- `{:?}`|`{:?}`", l_c, r_c);
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

    #[allow(clippy::bool_assert_comparison)]
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
            "ios_url" => "ios_url",
            "ios-url" => "ios-url",
            "ios+1" => "ios+1",
            "`ios 的`" => "`ios 的`",
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
            let (left, right) = if l.contains('=') {
                let pars = l.split('=').collect::<Vec<_>>();
                (pars[0].trim(), pars[1].trim())
            } else {
                (l.as_str(), l.as_str())
            };

            assert_eq!(right, format(left));
            assert_eq!(right, format(&left.to_uppercase()));
            assert_eq!(right, format(&left.to_lowercase()));
        }
    }
}
