// autocorrect: false
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref CHAR_WIDTH_MAP: HashMap<&'static str, &'static str> = map!(
      "ａ" => "a", "ｂ" => "b", "ｃ" => "c", "ｄ" => "d", "ｅ" => "e", "ｆ" => "f", "ｇ" => "g", "ｈ" => "h", "ｉ" => "i", "ｊ" => "j", "ｋ" => "k", "ｌ" => "l", "ｍ" => "m", "ｎ" => "n", "ｏ" => "o", "ｐ" => "p", "ｑ" => "q", "ｒ" => "r", "ｓ" => "s", "ｔ" => "t", "ｕ" => "u", "ｖ" => "v", "ｗ" => "w", "ｘ" => "x", "ｙ" => "y", "ｚ" => "z", "Ａ" => "A", "Ｂ" => "B", "Ｃ" => "C", "Ｄ" => "D", "Ｅ" => "E", "Ｆ" => "F", "Ｇ" => "G", "Ｈ" => "H", "Ｉ" => "I", "Ｊ" => "J", "Ｋ" => "K", "Ｌ" => "L", "Ｍ" => "M", "Ｎ" => "N", "Ｏ" => "O", "Ｐ" => "P", "Ｑ" => "Q", "Ｒ" => "R", "Ｓ" => "S", "Ｔ" => "T", "Ｕ" => "U", "Ｖ" => "V", "Ｗ" => "W", "Ｘ" => "X", "Ｙ" => "Y", "Ｚ" => "Z", "１" => "1", "２" => "2", "３" => "3", "４" => "4", "５" => "5", "６" => "6", "７" => "7", "８" => "8", "９" => "9", "０" => "0", "　" => " "
    );
    static ref HALF_TIME_RE: Regex = regexp!("{}", r"(\d)(：)(\d)");
}

pub fn halfwidth(text: &str) -> String {
    let mut out = String::new();

    for str in text.split("") {
        let new_str = CHAR_WIDTH_MAP.get(str);
        if new_str != None {
            out.push_str(new_str.unwrap())
        } else {
            out.push_str(str)
        }
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

    #[test]
    fn test_halfwidth() {
        let source = "ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ１２３４５６７８９０";
        assert_eq!(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
            halfwidth(source)
        );

        assert_eq!(
            "他说：我们将在16:32分出发去CBD中心。",
            halfwidth("他说：我们将在１６：３２分出发去ＣＢＤ中心。")
        );

        // Fullwidth space
        assert_eq!(
            "ジョイフル－後場売り気配 200 店舗を閉鎖へ 7 月以降、不採算店中心に",
            halfwidth("ジョイフル－後場売り気配　200 店舗を閉鎖へ　7 月以降、不採算店中心に")
        );
    }
}
