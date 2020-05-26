mod strategery;
use crate::strategery::*;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref FULL_DATE_RE: Regex =
        Regex::new(r"[\s]{0,}\d+[\s]{0,}年[\s]{0,}\d+[\s]{0,}月[\s]{0,}\d+[\s]{0,}[日号][\s]{0,}")
            .unwrap();
    static ref SPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref DASH_HANS_RE: Regex =
        Regex::new(r"([\p{Han}）】」》”’])([\-]+)([\p{Han}（【「《“‘])").unwrap();
    static ref LEFT_QUOTE_RE: Regex = Regex::new(r"\s([（【「《])").unwrap();
    static ref RIGHT_QUOTE_RE: Regex = Regex::new(r"([）】」》])\s").unwrap();

    // Strategies all rules
    static ref STRATEGIES: Vec<Strategery> = vec![
        // EnglishLetter
        Strategery::new(r"\p{Han}", r"[a-zA-Z]", true, true),
        // Number
        Strategery::new(r"\p{Han}", r"[0-9]", true, true),
        // SpecialSymbol
        Strategery::new(r"\p{Han}", r"[\|+$@#*]", true, true),
        Strategery::new(r"\p{Han}", r"[\[\(‘“]", true, false),
        Strategery::new(r"[’”\]\)!%]", r"\p{Han}", true, false),
        Strategery::new(r"[”\]\)!]", r"[a-zA-Z0-9]+", true, false),
        // FullwidthPunctuation
        Strategery::new(r"[\w\p{Han}]", r"[，。！？：；）」》】”’]", false, true),
        Strategery::new(r"[‘“【「《（]", r"[\w\p{Han}]", false, true),
    ];
}

pub(crate) fn format(text: &str) -> String {
    let mut out = String::from(text);
    for rule in STRATEGIES.iter() {
        out = rule.format(&out)
    }

    out = remove_full_date_spacing(&out);
    out = space_dash_with_hans(&out);

    return out;
}

// removeFullDateSpacing
// 发布 2013 年 3 月 10 号公布 -> 发布2013年3月10号公布
fn remove_full_date_spacing(text: &str) -> String {
    let mut out = String::from(text);
    for ma in FULL_DATE_RE.find_iter(&text) {
        let new_val = ma.as_str().replace(" ", "");
        out = out.replace(ma.as_str(), &new_val);
    }

    return out;
}

fn space_dash_with_hans(text: &str) -> String {
    let mut out = String::from(text);

    // 自由-开放
    out = (&DASH_HANS_RE.replace_all(&out, "$1 $2 $3")).to_string();
    out = (&LEFT_QUOTE_RE.replace_all(&out, "$1")).to_string();
    out = (&RIGHT_QUOTE_RE.replace_all(&out, "$1")).to_string();
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

    fn assert_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = format(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn it_format() {
        let cases = map![
            "部署到heroku有问题网页不能显示" => "部署到 heroku 有问题网页不能显示",
            "[北京]美企聘site/web大型应用开发高手-Ruby" => "[北京] 美企聘 site/web 大型应用开发高手-Ruby",
            "[成都](团800)招聘Rails工程师" => "[成都](团 800) 招聘 Rails 工程师",
            "Teahour.fm第18期发布" => "Teahour.fm 第 18 期发布",
            "Yes!升级到了Rails 4" => "Yes! 升级到了 Rails 4",
            "记事本,记事本显示阅读次数#149" => "记事本,记事本显示阅读次数 #149",
            "里面用@foo符号的话后面的变量名会被替换成userN" => "里面用 @foo 符号的话后面的变量名会被替换成 userN",
            "WWDC上讲到的Objective C/LLVM改进" => "WWDC 上讲到的 Objective C/LLVM 改进",
            "在Ubuntu11.10 64位系统安装newrelic出错" => "在 Ubuntu11.10 64 位系统安装 newrelic 出错",
            "升级了macOS 10.9 附遇到的Bug概率有0.1%或更少" => "升级了 macOS 10.9 附遇到的 Bug 概率有 0.1% 或更少",
            "在做Rails 3.2 Tutorial第Chapter 9.4.2遇到一个问题求助！" => "在做 Rails 3.2 Tutorial 第 Chapter 9.4.2 遇到一个问题求助！",
            "发现macOS安装软件新方法：Homebrew" => "发现 macOS 安装软件新方法：Homebrew",
            "without looking like it’s been marked up with tags or formatting instructions." => "without looking like it’s been marked up with tags or formatting instructions.",
            "隔夜SHIBOR报1.5530%，上涨33.80个基点。7天SHIBOR报2.3200%，上涨6.10个基点。3个月SHIBOR报2.8810%，下降1.80个" => "隔夜 SHIBOR 报 1.5530%，上涨 33.80 个基点。7 天 SHIBOR 报 2.3200%，上涨 6.10 个基点。3 个月 SHIBOR 报 2.8810%，下降 1.80 个",
            "野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元" => "野村：重申吉利汽车 (00175)“买入” 评级 上调目标价至 17.9 港元",
            "小米集团-W调整目标价为13.5港币" => "小米集团-W 调整目标价为 13.5 港币",
            "（路透社）-预计全年净亏损约1.3亿港元*预期因出售汽车" => "（路透社）- 预计全年净亏损约 1.3 亿港元 * 预期因出售汽车"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_date() {
        let cases = map![
            "于3月10日开始" => "于 3 月 10 日开始",
            "于3月开始" =>    "于 3 月开始",
            "于2009年开始" => "于 2009 年开始",
            "正式发布2013年3月10日-Ruby Saturday活动召集" => "正式发布2013年3月10日-Ruby Saturday 活动召集",
            "正式发布2013年3月10号发布" =>                 "正式发布2013年3月10号发布",
            "2013年12月22号开始出发" =>                  "2013年12月22号开始出发",
            "12月22号开始出发" =>                       "12 月 22 号开始出发",
            "22号开始出发" =>                          "22 号开始出发"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_english_letter() {
        let cases = map![
            "长桥LongBridge App下载" => "长桥 LongBridge App 下载"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_number() {
        let cases = map![
            "在Ubuntu 11.10 64位系统安装Go出错" => "在 Ubuntu 11.10 64 位系统安装 Go 出错",
            "喜欢暗黑2却对 D3不满意的可以看看这个。" =>     "喜欢暗黑 2 却对 D3 不满意的可以看看这个。",
            "Ruby 2.7版本第3次发布"=>          "Ruby 2.7 版本第 3 次发布"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_special_symbols() {
        let cases = map![
            "公告:(美股)阿里巴巴[BABA.US]发布2019下半年财报!" =>          "公告:(美股) 阿里巴巴 [BABA.US] 发布 2019 下半年财报!",
            "消息http://github.com解禁了" =>                     "消息 http://github.com 解禁了",
            "美股异动|阿帕奇石油(APA.US)盘前涨超15% 在苏里南近海发现大量石油" =>     "美股异动 | 阿帕奇石油 (APA.US) 盘前涨超 15% 在苏里南近海发现大量石油",
            "美国统计局：美国11月原油出口下降至302.3万桶/日，10月为338.3万桶/日。" => "美国统计局：美国 11 月原油出口下降至 302.3 万桶/日，10 月为 338.3 万桶/日。"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_fullwidth_symbols() {
        let cases = map![
            "（美股）市场：发布「最新」100消息【BABA.US】“大涨”50%；同比上涨20%！" => "（美股）市场：发布「最新」100 消息【BABA.US】“大涨” 50%；同比上涨 20%！",
            "第3季度财报发布看涨看跌？敬请期待。" =>                         "第 3 季度财报发布看涨看跌？敬请期待。"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_space_dash_with_hans() {
        let cases = map![
            "第3季度-财报发布看涨看跌？敬请期待。" => "第 3 季度 - 财报发布看涨看跌？敬请期待。",
            "腾讯-ADR-已发行" =>     "腾讯-ADR-已发行",
            "（腾讯）-发布-（新版）本微信" => "（腾讯）- 发布 -（新版）本微信",
            "【腾讯】-发布-【新版】本微信" => "【腾讯】- 发布 -【新版】本微信",
            "「腾讯」-发布-「新版」本微信" => "「腾讯」- 发布 -「新版」本微信",
            "《腾讯》-发布-《新版》本微信" => "《腾讯》- 发布 -《新版》本微信",
            "“腾讯”-发布-“新版”本微信" => "“腾讯” - 发布 - “新版” 本微信",
            "‘腾讯’-发布-‘新版’本微信" => "‘腾讯’ - 发布 - ‘新版’ 本微信"
        ];

        assert_cases(cases);
    }
}
