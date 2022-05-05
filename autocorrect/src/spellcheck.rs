use regex::Regex;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/spellchecks.rs"));

// Spell check by diect
pub fn spellcheck(text: &str) -> String {
    let mut out = String::from(text);

    for (word, re) in &*SPELLCHECK_RE_DICT {
        let new_word = SPELLCHECK_DICT.get(word).unwrap_or(word);
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
    use super::*;

    fn assert_spellcheck_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = spellcheck(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn test_spellcheck_basic() {
        let cases = map! [
            "sdk" => "SDK",
            "this is Api sDk website, and the sDk download url" => "this is API SDK website, and the SDK download URL",
            "sdk download" => "SDK download",
            "Download Sdk" => "Download SDK",
            "openSdk" => "openSdk",
            "https://sdk.com" => "https://sdk.com",
            "support@apple.com" => "support@apple.com",
            "开放 SDK 接口" => "开放 SDK 接口"
        ];

        assert_spellcheck_cases(cases);
    }

    #[test]
    fn test_speelcheck_cases() {
        let cases = map! [
            "打开 appstore 并查找" => "打开 App Store 并查找",
            "打开 app store 并查找" => "打开 App Store 并查找",
            "mac 电脑"=> "Mac 电脑",
            "开放 Api 接口" => "开放 API 接口"
        ];
        assert_spellcheck_cases(cases);
    }

    #[test]
    fn test_spellcheck_all() {
        let pair_re: regex::Regex = regex::Regex::new(r"\s*=\s*").unwrap();

        let data = std::fs::read_to_string(std::path::Path::new("./spellcheck/noun.txt")).unwrap();
        let lines = data
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect::<Vec<_>>();

        for l in lines.iter() {
            let mut left = *l;
            let mut right = *l;

            let pair = pair_re.split(l).collect::<Vec<_>>();
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
