use crate::config::CONFIG;

// Spell check by diect
pub fn spellcheck(text: &str) -> String {
    let mut out = String::from(text);

    let config = CONFIG.lock().unwrap();

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
