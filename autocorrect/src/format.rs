// autocorrect: false
use crate::code::{self, Results};

/// Automatically add spaces between Chinese and English words.
///
/// This method only work for plain text.
///
/// # Example
///
/// ```
/// extern crate autocorrect;
///
/// println!("{}", autocorrect::format("学习如何用 Rust 构建 Application"));
/// // => "学习如何用 Rust 构建 Application"
///
/// println!("{}", autocorrect::format("于 3 月 10 日开始"));
/// // => "于 3 月 10 日开始"
///
/// println!("{}", autocorrect::format("既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"));
/// // => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"
/// ```
pub fn format(text: &str) -> String {
    let result = crate::rule::format_or_lint(text, false);
    result.out
}

/// Format a html content.
///
///
/// Example:
///
/// ```
//  extern crate autocorrect;
//
/// let html = r#"
/// <article>
///   <h1>这是 Heading 标题</h1>
///   <div class="content">
///     <p>你好 Rust 世界<strong>Bold 文本</strong></p>
///     <p>这是第二行 p 标签</p>
///   </div>
/// </article>
/// "#;
/// autocorrect::format_html(html);
/// ```
#[deprecated(since = "2.0.0", note = "Please use `format_for` instead")]
pub fn format_html(html_str: &str) -> String {
    code::format_html(html_str).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{format_for, lint_for};

    use super::*;
    use std::collections::HashMap;

    #[track_caller]
    fn assert_cases(cases: HashMap<&str, &str>) {
        let mut fails: Vec<(String, String)> = Vec::new();
        for (source, exptected) in cases.into_iter() {
            let actual = format(source);

            if exptected != actual {
                fails.push((exptected.to_string(), actual));
            }
        }

        for (expected, actual) in fails.clone() {
            eprintln!("{}", crate::diff::diff_lines(&expected, &actual));
        }

        if !fails.is_empty() {
            panic!("Failed: {} cases.", fails.len());
        }
    }

    #[test]
    fn it_format() {
        let cases = map![
            "!sm" => "!sm",
            "Hello world!" => "Hello world!",
            "部署到heroku有问题网页不能显示" => "部署到 heroku 有问题网页不能显示",
            "[北京]美企聘web大型应用开发高手-Ruby" => "[北京] 美企聘 web 大型应用开发高手-Ruby",
            "[成都](团800)招聘Rails工程师" => "[成都](团 800) 招聘 Rails 工程师",
            "Teahour.fm第18期发布" => "Teahour.fm 第 18 期发布",
            "Yes!升级到了Rails 4" => "Yes! 升级到了 Rails 4",
            "WWDC上讲到的Objective C/LLVM 改进" => "WWDC 上讲到的 Objective C/LLVM 改进",
            "在Ubuntu11.10 64位系统安装newrelic出错" => "在 Ubuntu11.10 64 位系统安装 newrelic 出错",
            "升级了macOS 10.9 附遇到的Bug概率有0.1%或更少" => "升级了 macOS 10.9 附遇到的 Bug 概率有 0.1% 或更少",
            "在做Rails 3.2 Tutorial第Chapter 9.4.2遇到一个问题求助！" => "在做 Rails 3.2 Tutorial 第 Chapter 9.4.2 遇到一个问题求助！",
            "发现macOS安装软件新方法：Homebrew" => "发现 macOS 安装软件新方法：Homebrew",
            "without looking like it’s been marked up with tags or formatting instructions." => "without looking like it's been marked up with tags or formatting instructions.",
            "隔夜SHIBOR报1.5530%，上涨33.80%个基点。7天SHIBOR报2.3200%，上涨6.10个基点。3个月SHIBOR报2.8810%，下降1.80个" => "隔夜 SHIBOR 报 1.5530%，上涨 33.80% 个基点。7 天 SHIBOR 报 2.3200%，上涨 6.10 个基点。3 个月 SHIBOR 报 2.8810%，下降 1.80 个",
            // https://support.apple.com/zh-cn/iphone-12-and-iphone-12-pro-service-program-for-no-sound-issues
            "适用于“无声音”问题的iPhone 12和iPhone 12 Pro服务计划" => "适用于“无声音”问题的 iPhone 12 和 iPhone 12 Pro 服务计划",
            "野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元" => "野村：重申吉利汽车 (00175)“买入”评级 上调目标价至 17.9 港元",
            "小米集团-W调整目标价为13.5港币" => "小米集团-W 调整目标价为 13.5 港币",
            "（路透社）-预计全年净亏损约1.3亿港元*预期因出售汽车" => "（路透社）- 预计全年净亏损约 1.3 亿港元*预期因出售汽车",
            "（路透社）-预计全年净亏损约1.3亿\n\n港元*预期因出售汽车" => "（路透社）- 预计全年净亏损约 1.3 亿\n\n港元*预期因出售汽车",
            "Cell或RefCell类型使用某种形式的*内部" => "Cell 或 RefCell 类型使用某种形式的*内部",
            "Cell或RefCell类型使用某种形式的*内部可变性*" => "Cell 或 RefCell 类型使用某种形式的*内部可变性*",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_specials() {
        let cases = map![
            "记事本,记事本显示阅读次数#149" => "记事本，记事本显示阅读次数#149",
            "HashTag的演示#标签" => "HashTag 的演示#标签",
            "HashTag 的演示#标签#演示" =>         "HashTag 的演示#标签#演示",
            "Mention里面有关于中文的@某某人" =>        "Mention 里面有关于中文的@某某人",
            "Mention里面有关于中文的 @huacnlee 测试" =>        "Mention 里面有关于中文的 @huacnlee 测试",
            "Dollar的演示$阿里巴巴.US$股票标签" =>    "Dollar 的演示$阿里巴巴.US$股票标签",
            "测试英文,逗号Comma转换." =>    "测试英文，逗号 Comma 转换。",
            "测试英文,Comma逗号转换." =>    "测试英文，Comma 逗号转换。",
            "英文,逗号后面.阿里巴巴.US有空格?的情况!测试" =>    "英文，逗号后面。阿里巴巴.US 有空格？的情况！测试",
            "你好hello?world!" =>    "你好 hello?world!",
            "search by%关键词%" => "search by%关键词%",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_programming() {
        let cases = map![
            "A开头的case测试" => "A 开头的 case 测试",
            "内容带有\n不会处理" => "内容带有\n不会处理",
            "内容带有%s或%d或%v特殊字符，或者%S或%D或%V这些特殊format字符" => "内容带有%s或%d或%v特殊字符，或者%S或%D或%V这些特殊 format 字符",
            "内容带有$1或$2或$3特殊字符" => "内容带有$1或$2或$3特殊字符",
            "来自Yahoo!的文档" => "来自 Yahoo! 的文档",
            "规则后面是否跟随者!import以及规则的来源" => "规则后面是否跟随者!import 以及规则的来源",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_date() {
        let cases = map![
            "于3月10日开始" => "于 3 月 10 日开始",
            "于3月开始" =>    "于 3 月开始",
            "于2009年开始" => "于 2009 年开始",
            "正式发布2013年3月10日-Ruby Saturday活动召集" => "正式发布 2013 年 3 月 10 日-Ruby Saturday 活动召集",
            "正式发布2013年3月10号发布" =>                 "正式发布 2013 年 3 月 10 号发布",
            "2013年12月22号开始出发" =>                  "2013 年 12 月 22 号开始出发",
            "12月22号开始出发" =>                       "12 月 22 号开始出发",
            "22号开始出发" =>                          "22 号开始出发",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_with_markdown_td() {
        // By LEFT_QUOTE_RE, RIGHT_QUOTE_RE
        let cases = map![
            "| 8 位有符号整数（补码）   |" => "| 8 位有符号整数（补码）   |",
            "| 8 位有符号整数（补码） |" => "| 8 位有符号整数（补码） |",
            "| 8 位有符号整数   |" => "| 8 位有符号整数   |",
            "| 8 位有符号整数。   |" => "| 8 位有符号整数。   |",
            "| 包括 8 位有符号整数。 |" => "| 包括 8 位有符号整数。 |",
            "|   包括 8 位有符号整数！   |" => "|   包括 8 位有符号整数！   |",
            "| 64 位浮点数（例如：10.90）| `double` |" => "| 64 位浮点数（例如：10.90）| `double` |",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_remove_spaces_with_punctuation() {
        let cases = map![
            "以达到快速、 跨平台 、 低资源占用的目的 。 很多著名且受欢迎的软件，例如 Firefox 、 Dropbox 和 Cloudflare 都在使用" => "以达到快速、跨平台、低资源占用的目的。很多著名且受欢迎的软件，例如 Firefox、Dropbox 和 Cloudflare 都在使用",
            "注意： 引进给变量， 转换为机器代码。 这意味着任何变量、 常量； 命名的概念都会被删除" => "注意：引进给变量，转换为机器代码。这意味着任何变量、常量；命名的概念都会被删除",
            "注意 ： 引进给变量 ， 转换为机器代码 。 这意味着任何变量 、 常量 ； 命名的概念都会被删除" => "注意：引进给变量，转换为机器代码。这意味着任何变量、常量；命名的概念都会被删除",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_number() {
        let cases = map![
            "在Ubuntu 11.10 64位系统安装Go出错" => "在 Ubuntu 11.10 64 位系统安装 Go 出错",
            "喜欢暗黑2却对 D3不满意的可以看看这个。" =>     "喜欢暗黑 2 却对 D3 不满意的可以看看这个。",
            "Ruby 2.7版本第3次发布"=>          "Ruby 2.7 版本第 3 次发布",
            "值范围-255或+255之间" => "值范围 -255 或 +255 之间",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_special_symbols() {
        let cases = map![
            "公告:(美股)阿里巴巴[BABA.US]发布2019下半年财报!" => "公告:(美股) 阿里巴巴 [BABA.US] 发布 2019 下半年财报！",
            "消息github.com解禁了" => "消息 github.com 解禁了",
            "美股异动|阿帕奇石油(APA.US)盘前涨超15% 在苏里南近海发现大量石油" => "美股异动 | 阿帕奇石油 (APA.US) 盘前涨超 15% 在苏里南近海发现大量石油",
            "美国统计局：美国11月原油出口下降至302.3万桶/日，10月为338.3万桶/日。" => "美国统计局：美国 11 月原油出口下降至 302.3 万桶/日，10 月为 338.3 万桶/日。",
            "[b]Foo bar dar[/b]" => "[b]Foo bar dar[/b]",
            // r#"{标签内的"a"元素上的'target'属性在}"# => r#"{标签内的 "a" 元素上的 'target' 属性在}"#,
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_fullwidth_symbols() {
        let cases = map![
            "（美股）市场：发布「最新」100消息【BABA.US】“大涨”50%；同比上涨20%！" => "（美股）市场：发布「最新」100 消息【BABA.US】“大涨”50%；同比上涨 20%！",
            "第3季度财报发布看涨看跌？敬请期待。" =>                         "第 3 季度财报发布看涨看跌？敬请期待。",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_space_dash_with_hans() {
        let cases = map![
            "范围包含A-Z等字符" => "范围包含 A-Z 等字符",
            "第3季度-财报发布看涨看跌？敬请期待。" => "第 3 季度 - 财报发布看涨看跌？敬请期待。",
            "腾讯-ADR-已发行" =>     "腾讯-ADR-已发行",
            "（腾讯）-发布-（新版）本微信" => "（腾讯）- 发布 -（新版）本微信",
            "【腾讯】-发布-【新版】本微信" => "【腾讯】- 发布 -【新版】本微信",
            "「腾讯」-发布-「新版」本微信" => "「腾讯」- 发布 -「新版」本微信",
            "《腾讯》-发布-《新版》本微信" => "《腾讯》- 发布 -《新版》本微信",
            "“腾讯”-发布-“新版”本微信" => "“腾讯” - 发布 - “新版”本微信",
            "‘腾讯’-发布-‘新版’本微信" => "‘腾讯’ - 发布 - ‘新版’本微信",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_url() {
        // URL or Path need keep original format
        let cases = map![
            "wiki/网页浏览器列表#基於WebKit排版引擎" => "wiki/网页浏览器列表#基於WebKit排版引擎",
            "wiki-/_网页浏基於WebKit排版" => "wiki-/_网页浏基於WebKit排版",
            "Web/CSS/网格-模板-列" => "Web/CSS/网格-模板-列",
            "wiki/hello网页浏览器列表#基於WebKit排版引擎" => "wiki/hello网页浏览器列表#基於WebKit排版引擎",
            "URL地址 /wiki/hello网页浏览器 访问" => "URL 地址 /wiki/hello网页浏览器 访问",
            "请打开URL地址 https://google.com/这是URL文件名.html 访问" => "请打开 URL 地址 https://google.com/这是URL文件名.html 访问",
            "https://google.com/这是URL文件名.html" => "https://google.com/这是URL文件名.html",
            "https://zh.wikipedia.org/wiki/网页浏览器列表#基於WebKit排版引擎" => "https://zh.wikipedia.org/wiki/网页浏览器列表#基於WebKit排版引擎",
            "//this is注释" => "//this is 注释",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_cjk() {
        let cases = map![
            "全世界已有数百家公司在生产环境中使用Rust，以达到快速、跨平台、低资源占用的目的。很多著名且受欢迎的软件，例如Firefox、 Dropbox和Cloudflare都在使用Rust。" => "全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。很多著名且受欢迎的软件，例如 Firefox、Dropbox 和 Cloudflare 都在使用 Rust。",
            "現今全世界上百家公司企業為了尋求快速、節約資源而且能跨平台的解決辦法，都已在正式環境中使用Rust。許多耳熟能詳且受歡迎的軟體，諸如Firefox、Dropbox以及Cloudflare都在使用Rust。" => "現今全世界上百家公司企業為了尋求快速、節約資源而且能跨平台的解決辦法，都已在正式環境中使用 Rust。許多耳熟能詳且受歡迎的軟體，諸如 Firefox、Dropbox 以及 Cloudflare 都在使用 Rust。",
            "既に、世界中の数百という企業がRustを採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。皆さんがご存じで愛用しているソフトウェア、例えばFirefox、DropboxやCloudflareも、Rustを採用しています。" => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。皆さんがご存じで愛用しているソフトウェア、例えば Firefox、Dropbox や Cloudflare も、Rust を採用しています。",
            "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다. Firefox, Dropbox 및 Cloudflare와 같이 잘 알려져 있고 널리 사용되는 많은 소프트웨어가 Rust를 사용하고 있습니다." => "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다. Firefox, Dropbox 및 Cloudflare 와 같이 잘 알려져 있고 널리 사용되는 많은 소프트웨어가 Rust 를 사용하고 있습니다.",
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_lint_for() {
        let raw = "<p>Hello你好ios版本</p>";
        let result = lint_for(raw, "foo.bar.html");
        let expect_json = r#"{"filepath":"foo.bar.html","lines":[{"l":1,"c":4,"new":"Hello 你好 iOS 版本","old":"Hello你好ios版本","severity":1}],"error":""}"#;
        assert!(!result.has_error());
        assert_eq!(1, result.lines.len());
        assert_eq!(expect_json, result.to_json());

        let result1 = lint_for("const a = 'hello世界'", "js");
        assert!(!result1.has_error());
        assert_eq!(1, result1.lines.len());
    }

    #[test]
    fn it_format_for() {
        let raw = "<p>Hello你好</p>";
        let result = format_for(raw, "foo.bar.html");
        assert!(!result.has_error());
        assert_eq!("<p>Hello 你好</p>", result.out);

        let result1 = format_for("const a = 'hello世界'", "js");
        assert!(!result1.has_error());
        assert_eq!("const a = 'hello 世界'", result1.out);
    }

    #[test]
    fn test_format_with_text_rules() {
        crate::config::setup_test();

        let raw = "textRule忽略测试，这是一个文本。\n这行是textRule警告\n这行word应该改变.";
        let expected = "textRule忽略测试，这是一个文本。\n这行是textRule警告\n这行 word 应该改变。";
        let lint_result = r#"{"filepath":"text","lines":[{"l":2,"c":1,"new":"这行是 textRule 警告","old":"这行是textRule警告","severity":2},{"l":3,"c":1,"new":"这行 word 应该改变。","old":"这行word应该改变.","severity":1}],"error":""}"#;
        assert_eq!(expected, format(raw));

        assert_eq!(expected, format_for(raw, "text").out);
        assert_eq!(lint_result, lint_for(raw, "text").to_json());
    }

    #[test]
    fn test_format_halfwidth() {
        let cases = map! {
            "你好\nWelcome all say hello，world。\n你好world" => "你好\nWelcome all say hello, world.\n你好 world",
            "Jetbrains请访问：https://www.jetbrains.com/help/idea/using-git-integration.html。" => "Jetbrains 请访问：https://www.jetbrains.com/help/idea/using-git-integration.html。",
        };

        assert_cases(cases);
    }
}
