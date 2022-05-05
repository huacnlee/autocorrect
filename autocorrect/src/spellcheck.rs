use regex::Regex;

use crate::config::CONFIG;

pub(crate) fn word_regexp(word: &str) -> Regex {
    regexp!(
        r"(?im)([\s，。、？！]|^)+({})([\s，。、？！]|$)+",
        word.replace('-', r"\-").replace('.', r"\.")
    )
}

// Spell check by diect
pub fn spellcheck(text: &str) -> String {
    let mut out = String::from(text);

    let config = CONFIG.lock().unwrap();
    if config.spellcheck.is_disabled() {
        return out;
    }

    let spellcheck_dict_re = &config.spellcheck.dict_re;
    let spellcheck_dict = &config.spellcheck.dict;

    for (word, re) in spellcheck_dict_re.iter() {
        let new_word = spellcheck_dict.get(word).unwrap_or(word);
        out = re
            .replace_all(&out, |cap: &regex::Captures| {
                cap[0].replace(&cap[2], new_word)
            })
            .to_string();
    }

    out
}

#[cfg(test)]
mod tests {
    use super::CONFIG;
    use super::*;
    use std::collections::HashMap;

    fn assert_spellcheck_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = spellcheck(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn test_spellcheck_basic() {
        let cases = map! [
            "ios" => "iOS",
            "this is ipad ios website, and the IOS download url" => "this is iPad iOS website, and the iOS download url",
            "Ios download" => "iOS download",
            "Download iOs" => "Download iOS",
            "openios" => "openios",
            "https://ios.com" => "https://ios.com",
            "support@ios.com" => "support@ios.com",
            "开放 IOS 接口" => "开放 iOS 接口",
            "开放接口 IOS。" => "开放接口 iOS。",
            "开放接口 IOS？" => "开放接口 iOS？",
            "开放接口 IOS！" => "开放接口 iOS！",
            "开放接口 IOS，" => "开放接口 iOS，",
            "开放，ios 接口" => "开放，iOS 接口"
        ];

        assert_spellcheck_cases(cases);
    }

    #[test]
    fn test_speelcheck_cases() {
        let cases = map! [
            "打开 wifi 并找到就近的 WIFI，点击输入 wi-fi 密码" => "打开 Wi-Fi 并找到就近的 Wi-Fi，点击输入 Wi-Fi 密码"
        ];
        assert_spellcheck_cases(cases);
    }

    #[test]
    fn test_spellcheck_all() {
        let words = CONFIG.lock().unwrap().spellcheck.words.clone();
        for l in words.iter() {
            let mut left = l.as_str();
            let mut right = l.as_str();

            let pair = crate::config::PAIR_RE.split(l).collect::<Vec<_>>();
            if pair.len() == 2 {
                left = pair[0];
                right = pair[1];
            }

            assert_eq!(right, spellcheck(left));
            assert_eq!(right, spellcheck(&left.to_uppercase()));
            assert_eq!(right, spellcheck(&left.to_lowercase()));
        }
    }
}
