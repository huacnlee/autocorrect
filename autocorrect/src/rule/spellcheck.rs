use regex::Regex;

use crate::config::Config;

pub(crate) fn word_regexp(word: &str) -> Regex {
    let prefix = r#"([^\W]|[\p{Han}？！：，。；、]|$|^)"#;

    regexp!(
        r#"(?im){}([\s？！：，。；、]|^)+({})([\s？！：，。；、]|$)+{}"#,
        prefix,
        word.replace('-', r"\-").replace('.', r"\."),
        prefix
    )
}

// Spell check by dict
pub fn format(text: &str) -> String {
    let mut out = String::from(text);

    let config = Config::current();

    let spellcheck_dict_re = &config.spellcheck.dict_re;
    let spellcheck_dict = &config.spellcheck.dict;

    for (word, re) in spellcheck_dict_re.iter() {
        let new_word = spellcheck_dict.get(word).unwrap_or(word);
        out = re
            .replace_all(&out, |cap: &regex::Captures| {
                cap[0].replace(&cap[3], new_word)
            })
            .to_string();
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::Config;

    use super::*;
    use std::collections::HashMap;

    fn assert_spellcheck_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = format(source);
            assert_eq!(exptected, actual);
        }
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
            let mut left = l.as_str();
            let mut right = l.as_str();

            let pair = crate::config::PAIR_RE.split(l).collect::<Vec<_>>();
            if pair.len() == 2 {
                left = pair[0];
                right = pair[1];
            }

            assert_eq!(right, format(left));
            assert_eq!(right, format(&left.to_uppercase()));
            assert_eq!(right, format(&left.to_lowercase()));
        }
    }
}
