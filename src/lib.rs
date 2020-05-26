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

pub(crate) fn format(text: &str) -> &str {
    let out = text;
    for rule in STRATEGIES.iter() {
        out = rule.format(out)
    }
    return text;
}

// removeFullDateSpacing
// 发布 2013 年 3 月 10 号公布 -> 发布2013年3月10号公布
fn remove_full_date_spacing(text: &str) {
    &FULL_DATE_RE.replace_all(&text, "");
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

    fn assertCases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let mut actual = format(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_format() {
        let cases = map![
            "部署到heroku有问题网页不能显示"=>                                                           "部署到 heroku 有问题网页不能显示",
            "[北京]美企聘site/web大型应用开发高手-Ruby"=>                                                  "[北京] 美企聘 site/web 大型应用开发高手-Ruby",
            "[成都](团800)招聘Rails工程师"=>                                                          "[成都](团 800) 招聘 Rails 工程师",
            "Teahour.fm第18期发布"=>                                                              "Teahour.fm 第 18 期发布",
            "Yes!升级到了Rails 4"=>                                                               "Yes! 升级到了 Rails 4",
            "记事本,记事本显示阅读次数#149"=>                                                            "记事本,记事本显示阅读次数 #149",
            "里面用@foo符号的话后面的变量名会被替换成userN"=>                                                    "里面用 @foo 符号的话后面的变量名会被替换成 userN",
            "WWDC上讲到的Objective C/LLVM改进"=>                                                    "WWDC 上讲到的 Objective C/LLVM 改进",
            "在Ubuntu11.10 64位系统安装newrelic出错"=>                                                 "在 Ubuntu11.10 64 位系统安装 newrelic 出错",
            "升级了macOS 10.9 附遇到的Bug概率有0.1%或更少"=>                                                "升级了 macOS 10.9 附遇到的 Bug 概率有 0.1% 或更少",
            "在做Rails 3.2 Tutorial第Chapter 9.4.2遇到一个问题求助！"=>                                    "在做 Rails 3.2 Tutorial 第 Chapter 9.4.2 遇到一个问题求助！",
            "发现macOS安装软件新方法：Homebrew"=>                                                       "发现 macOS 安装软件新方法：Homebrew",
            "without looking like it’s been marked up with tags or formatting instructions."=> "without looking like it’s been marked up with tags or formatting instructions.",
            "隔夜SHIBOR报1.5530%，上涨33.80个基点。7天SHIBOR报2.3200%，上涨6.10个基点。3个月SHIBOR报2.8810%，下降1.80个"=> "隔夜 SHIBOR 报 1.5530%，上涨 33.80 个基点。7 天 SHIBOR 报 2.3200%，上涨 6.10 个基点。3 个月 SHIBOR 报 2.8810%，下降 1.80 个",
            "野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元"=>                                              "野村：重申吉利汽车 (00175)“买入” 评级 上调目标价至 17.9 港元",
            "小米集团-W调整目标价为13.5港币"=>                                                              "小米集团-W 调整目标价为 13.5 港币",
            "（路透社）-预计全年净亏损约1.3亿港元*预期因出售汽车"=>                                                     "（路透社）- 预计全年净亏损约 1.3 亿港元 * 预期因出售汽车"
        ];
    }
}
