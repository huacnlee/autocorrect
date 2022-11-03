// autocorrect: false
use regex::Regex;
use std::collections::HashMap;

const SPCIAL_PUNCTUATIONS: &str = "[.:!]([ ]*)";
const NORMAL_PUNCTUATIONS: &str = "[,?]([ ]*)";

lazy_static! {
    static ref FULLWIDTH_MAPS: HashMap<&'static str, &'static str> = map!(
      "," => "，",
      "." => "。",
      ";" => "；",
      ":" => "：",
      "!" => "！",
      "?" => "？"
    );
    static ref PUNCTUATION_WITH_LEFT_CJK_RE: Regex = regexp!(
        "{}{}{}",
        r"[\p{CJ}\w\d]+",
        NORMAL_PUNCTUATIONS,
        r"[\p{CJ}]+"
    );
    static ref PUNCTUATION_WITH_RIGHT_CJK_RE: Regex =
        regexp!("{}{}", r"[\p{CJ}]+", NORMAL_PUNCTUATIONS);
    static ref PUNCTUATION_WITH_SPEICAL_CJK_RE: Regex =
        regexp!("{}{}{}", r"[\p{CJ}]+", SPCIAL_PUNCTUATIONS, r"[\p{CJ}]+");
    static ref PUNCTUATION_WITH_SPEICAL_LAST_CJK_RE: Regex =
        regexp!("{}{}{}", r"[\p{CJ}]+", SPCIAL_PUNCTUATIONS, r#"["']?$"#);
    static ref PUNCTUATIONS_RE: Regex =
        regexp!("({}|{})", SPCIAL_PUNCTUATIONS, NORMAL_PUNCTUATIONS);
}

// fullwidth correct punctuations near the CJK chars
pub fn format(text: &str) -> String {
    let mut out = String::from(text);

    out = PUNCTUATION_WITH_LEFT_CJK_RE
        .replace_all(&out, |cap: &regex::Captures| {
            fullwidth_replace_part(&cap[0])
        })
        .to_string();

    out = PUNCTUATION_WITH_RIGHT_CJK_RE
        .replace_all(&out, |cap: &regex::Captures| {
            fullwidth_replace_part(&cap[0])
        })
        .to_string();

    out = PUNCTUATION_WITH_SPEICAL_CJK_RE
        .replace_all(&out, |cap: &regex::Captures| {
            fullwidth_replace_part(&cap[0])
        })
        .to_string();

    out = PUNCTUATION_WITH_SPEICAL_LAST_CJK_RE
        .replace_all(&out, |cap: &regex::Captures| {
            fullwidth_replace_part(&cap[0])
        })
        .to_string();

    out
}

fn fullwidth_replace_part(part: &str) -> String {
    let out = PUNCTUATIONS_RE.replace_all(part, |cap: &regex::Captures| {
        let str = &cap[0];
        return FULLWIDTH_MAPS[String::from(str).trim()];
    });

    out.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn assert_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = format(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn test_fullwidth() {
        let cases = map!(
          "你好,这是一个句子." => "你好，这是一个句子。",
          // For Programming String with quote: "Hello你好." => "Hello 你好。"
          "\"请求参数错误.\"" => "\"请求参数错误。\"",
          "'请求参数错误.'" => "'请求参数错误。'",
          "!开头不处理." => "!开头不处理。",
          "刚刚买了一部 iPhone,好开心!" => "刚刚买了一部 iPhone，好开心！",
          "蚂蚁集团上市后有多大的上涨空间?" =>  "蚂蚁集团上市后有多大的上涨空间？",
          "我们需要一位熟悉 JavaScript、HTML5,至少理解一种框架 (如 Backbone.js、AngularJS、React 等) 的前端开发者." => "我们需要一位熟悉 JavaScript、HTML5，至少理解一种框架 (如 Backbone.js、AngularJS、React 等) 的前端开发者。",
          "蚂蚁疾奔:蚂蚁集团两地上市~全速推进!" =>                                                        "蚂蚁疾奔：蚂蚁集团两地上市~全速推进！",
          "蚂蚁集团是阿里巴巴 (BABA.N) 旗下金融科技子公司" =>                                                "蚂蚁集团是阿里巴巴 (BABA.N) 旗下金融科技子公司",
          "Dollar 的演示 $阿里巴巴.US$ 股票标签" =>                                                   "Dollar 的演示 $阿里巴巴.US$ 股票标签",
          "确保&quot;&gt;HTML Entity&lt;&quot;的字符&#34;不会被处理&#34; Ruby&amp;Go" => "确保&quot;&gt;HTML Entity&lt;&quot;的字符&#34;不会被处理&#34; Ruby&amp;Go"
        );

        assert_cases(cases);
    }

    #[test]
    fn test_fullwidth_with_cjk() {
        let cases = map!(
            "你好,这是一个句子." => "你好，这是一个句子。",
            "你好,這是一個句子." => "你好，這是一個句子。",
            "でもっと多くのことができるようになります.そんな新機能の数々をさっそく体験してみましょう." => "でもっと多くのことができるようになります。そんな新機能の数々をさっそく体験してみましょう。",
            "근면, 검소, 협동은 우리 겨레의 미덕이다." => "근면, 검소, 협동은 우리 겨레의 미덕이다."
        );

        assert_cases(cases);
    }
}
