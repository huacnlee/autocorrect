use std::{borrow::Cow, collections::HashMap};

use crate::{config::Config, keyword::MatchedResult};

// Spell check by dict
pub fn format(text: &str) -> Cow<str> {
    let config = Config::current();

    let word_map = &config.spellcheck.word_map;
    let matcher = &config.spellcheck.matcher;

    let matched_words = matcher.match_keywords(text);
    if matched_words.is_empty() {
        return Cow::Borrowed(text);
    }
    replace_with_spans(text, matched_words, word_map)
}

#[derive(Debug)]
struct SpanInfo<'a> {
    new: &'a String,
    old_chars_count: usize,
    span: usize,
}

fn replace_with_spans<'a>(
    text: &'a str,
    words: MatchedResult,
    word_map: &HashMap<String, String>,
) -> Cow<'a, str> {
    let mut text_chars = text.chars().collect::<Vec<_>>();
    let mut span_infos = Vec::with_capacity(words.len());
    for (old, (old_chars_count, spans)) in words {
        if let Some(new) = word_map.get(old) {
            for span in spans {
                // skip if the new chars are the same as the old chars
                if text_chars[span..span + old_chars_count]
                    .iter()
                    .copied()
                    .ne(new.chars())
                {
                    span_infos.push(SpanInfo {
                        new,
                        span,
                        old_chars_count,
                    });
                }
            }
        }
    }
    span_infos.sort_unstable_by_key(|s| s.span);

    let mut offset_change = 0;
    let mut changed = false;

    for span_info in span_infos.iter() {
        let new_str = span_info.new;
        let old_chars_count = span_info.old_chars_count;
        let span_start = span_info.span;
        let span_end = span_info.span + old_chars_count;

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

        if l_c.map(is_disallowed_char).unwrap_or_default()
            || r_c.map(is_disallowed_char).unwrap_or_default()
        {
            // println!("---- `{:?}`|`{:?}`", l_c, r_c);
            continue;
        }

        // Perform replacement
        text_chars.splice(offset_start..offset_end, new_str.chars());

        // Update offset_change due to length change after replacement
        offset_change += new_str.chars().count() - old_chars_count;
        changed = true;
    }

    if changed {
        Cow::Owned(text_chars.into_iter().collect::<String>())
    } else {
        Cow::Borrowed(text)
    }
}

fn is_disallowed_char(c: &char) -> bool {
    if c.is_whitespace() {
        return false;
    }
    // CJK Unified Ideographs
    // https://en.wikipedia.org/wiki/CJK_Unified_Ideographs_(Unicode_block)
    if ('\u{4E00}'..='\u{9FFF}').contains(c) {
        return false;
    }
    // CJK punctuation characters
    !"？！：，。；、「」“”‘’【】《》".contains(*c)
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
    fn test_disallow_char() {
        assert_eq!(is_disallowed_char(&','), true);
        assert_eq!(is_disallowed_char(&'a'), true);
        assert_eq!(is_disallowed_char(&'-'), true);
        assert_eq!(is_disallowed_char(&'你'), false);
        assert_eq!(is_disallowed_char(&'，'), false);
        assert_eq!(is_disallowed_char(&'？'), false);
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
