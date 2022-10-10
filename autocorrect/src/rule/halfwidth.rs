// autocorrect: false
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref CHAR_WIDTH_MAP: HashMap<&'static str, &'static str> = map!(
      "ａ" => "a", "ｂ" => "b", "ｃ" => "c", "ｄ" => "d", "ｅ" => "e", "ｆ" => "f", "ｇ" => "g", "ｈ" => "h", "ｉ" => "i", "ｊ" => "j", "ｋ" => "k", "ｌ" => "l", "ｍ" => "m", "ｎ" => "n", "ｏ" => "o", "ｐ" => "p", "ｑ" => "q", "ｒ" => "r", "ｓ" => "s", "ｔ" => "t", "ｕ" => "u", "ｖ" => "v", "ｗ" => "w", "ｘ" => "x", "ｙ" => "y", "ｚ" => "z", "Ａ" => "A", "Ｂ" => "B", "Ｃ" => "C", "Ｄ" => "D", "Ｅ" => "E", "Ｆ" => "F", "Ｇ" => "G", "Ｈ" => "H", "Ｉ" => "I", "Ｊ" => "J", "Ｋ" => "K", "Ｌ" => "L", "Ｍ" => "M", "Ｎ" => "N", "Ｏ" => "O", "Ｐ" => "P", "Ｑ" => "Q", "Ｒ" => "R", "Ｓ" => "S", "Ｔ" => "T", "Ｕ" => "U", "Ｖ" => "V", "Ｗ" => "W", "Ｘ" => "X", "Ｙ" => "Y", "Ｚ" => "Z", "１" => "1", "２" => "2", "３" => "3", "４" => "4", "５" => "5", "６" => "6", "７" => "7", "８" => "8", "９" => "9", "０" => "0", "　" => " ",
    );
    static ref PUNCTUATION_WITHOUT_SPACE_MAP: HashMap<&'static str, &'static str> = map!(
        "’" => "'",
    );
    static ref PUNCTUATION_WITH_SPACE_SUFFIX_MAP: HashMap<&'static str, &'static str> = map!(
        "，" => ",",
        "、" => ",",
        "。" => ".",
        "：" => ":",
        "；" => ".",
        "！" => "!",
        "？" => "?",
        "”" => r#"""#,
        "）" => ")",
        "】" => "]",
        "」" => "]",
        "》" => r#"""#,
    );
    static ref PUNCTUATION_WITH_SPACE_PREFIX_MAP: HashMap<&'static str, &'static str> = map!(
        "“" => r#"""#,
        "（" => "(",
        "【" => "[",
        "「" => "[",
        "《" => r#"""#,
    );
    static ref HALF_TIME_RE: Regex = regexp!("{}", r"(\d)(：)(\d)");
    static ref CJK_RE: Regex = regexp!("{}", r"\p{CJK}");
}

trait CharMatching {
    fn is_ascii_alphanumeric_punctuation(&self) -> bool;
}

impl CharMatching for char {
    /// Match is a-z, A-Z, 0-9, all ASCII punctuations
    fn is_ascii_alphanumeric_punctuation(&self) -> bool {
        self.is_ascii_alphanumeric() || self.is_ascii_punctuation()
    }
}

pub fn format(text: &str) -> String {
    let has_cjk = CJK_RE.is_match(text);
    let mut out = String::new();

    let mut parts = text.split("").peekable();
    while let Some(mut part) = parts.next() {
        let next_part = parts.peek().unwrap_or(&"");
        let last_part = out.chars().last().unwrap_or(' ');

        // TODO: Here has force disable this feature, still working...
        if !has_cjk && false == true {
            // Remove duplicate space without CJK contents
            if part.ends_with(|s: char| s.is_whitespace())
                && !next_part.starts_with(|s: char| s.is_ascii_alphanumeric_punctuation())
            {
                part = "";
            }

            // Fix punctuation without CJK contents
            if let Some(new_str) = PUNCTUATION_WITH_SPACE_SUFFIX_MAP.get(part) {
                out.push_str(new_str);
                // Suffix with a space, if next is alphanumeric or punctuation
                if next_part.starts_with(|s: char| s.is_ascii_alphanumeric_punctuation()) {
                    out.push(' ');
                }
                continue;
            }

            if let Some(new_str) = PUNCTUATION_WITH_SPACE_PREFIX_MAP.get(part) {
                // Prefix with a space, if last char is alphanumeric or punctuation
                if last_part.is_ascii_alphanumeric_punctuation() {
                    out.push(' ')
                }
                out.push_str(new_str);
                continue;
            }

            if let Some(new_str) = PUNCTUATION_WITHOUT_SPACE_MAP.get(part) {
                out.push_str(new_str);
                continue;
            }
        }

        if let Some(new_str) = CHAR_WIDTH_MAP.get(part) {
            out.push_str(new_str);
            continue;
        }

        out.push_str(part);
    }

    // Fix 12：00 -> 12:00
    out = HALF_TIME_RE
        .replace_all(&out, |cap: &regex::Captures| cap[0].replace('：', ":"))
        .to_string();

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = format(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn test_halfwidth_alphabetic_numbers() {
        let source = "ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ１２３４５６７８９０";
        assert_eq!(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
            format(source)
        );

        assert_eq!(
            "他说：我们将在16:32分出发去CBD中心。",
            format("他说：我们将在１６：３２分出发去ＣＢＤ中心。")
        );

        // Fullwidth space
        assert_eq!(
            "ジョイフル－後場売り気配 200 店舗を閉鎖へ 7 月以降、不採算店中心に",
            format("ジョイフル－後場売り気配　200 店舗を閉鎖へ　7 月以降、不採算店中心に")
        );
    }

    #[test]
    fn test_halfwidth_punctuation() {
        let cases = map! [
            // "说：你好 english。" => "说：你好 english。",
            // "‘腾讯’ - 发布 - ‘新版’本微信" => "‘腾讯’ - 发布 - ‘新版’本微信",
            // "Said：Come and，Join us！" => "Said: Come and, Join us!",
            // "Said： Come  and， [Join]   us  " => "Said: Come and, [Join] us",
            // "Come and？Join us?" => "Come and? Join us?",
            // "Come and， Join us！" => "Come and, Join us!",
            // "The microphone or camera is occupied，Please check and re-record the video。" => "The microphone or camera is occupied, Please check and re-record the video.",
            // "Exchange’s" => "Exchange's",
            // "The“Convertible Amount”case。" => r#"The "Convertible Amount" case."#,
            // "The（Convertible Amount）case！" => r#"The (Convertible Amount) case!"#,
            // "The【Convertible Amount】case？" => "The [Convertible Amount] case?",
            // "The「Convertible Amount」case：" => "The [Convertible Amount] case:",
            // "The《Convertible Amount》case，" => r#"The "Convertible Amount" case,"#,
        ];

        assert_cases(cases);
    }
}
