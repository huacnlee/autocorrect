// autocorrect: false
/*!
Automatically add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).

## Other implements

- Ruby - [auto-correct](https://github.com/huacnlee/auto-correct).
- Go - [go-auto-correct](https://github.com/huacnlee/go-auto-correct).
- Rust - [autocorrect](https://github.com/huacnlee/autocorrect).

## Features

- Auto add spacings between CJK (Chinese, Japanese, Korean) and English words.
- HTML content support.

## Example

Use `autocorrect::format` to format plain text.

```rust
extern crate autocorrect;

fn main() {
    println!("{}", autocorrect::format("长桥 LongBridge App 下载"));
    // => "长桥 LongBridge App 下载"

    println!("{}", autocorrect::format("Ruby 2.7 版本第 1 次发布"));
    // => "Ruby 2.7 版本第 1 次发布"

    println!("{}", autocorrect::format("于 3 月 10 日开始"));
    // => "于 3 月 10 日开始"

    println!("{}", autocorrect::format("包装日期为2013年3月10日"));
    // => "包装日期为2013年3月10日"

    println!("{}", autocorrect::format("全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。"));
    // => "全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。"

    println!("{}", autocorrect::format("既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"));
    // => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"

    println!("{}", autocorrect::format("전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고， 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."));
    // => "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고， 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."
}
```
*/
#[macro_use]
extern crate lazy_static;

macro_rules! regexp {
    ($($arg:tt)*) => {{
        let reg_str = format!($($arg)*);

        let mut rule_str = String::from(reg_str).replace(
            r"\p{CJK}",
            r"\p{Han}|\p{Hangul}|\p{Hanunoo}|\p{Katakana}|\p{Hiragana}|\p{Bopomofo}",
        );
        rule_str = String::from(rule_str).replace(
            r"\p{CJ}",
            r"\p{Han}|\p{Katakana}|\p{Hiragana}|\p{Bopomofo}",
        );
        // println!("{}", rule_str);
        let res = regex::Regex::new(&rule_str).unwrap();
        res
    }};
}

#[macro_export]
macro_rules! map {
    {$($key:expr => $value:expr),+} => {{
        let mut m = HashMap::new();
        $(
            m.insert($key, $value);
        )+
        m
    }};
}

mod code;
mod fullwidth;
mod halfwidth;
pub mod ignorer;
mod strategery;
pub mod types;

mod csharp;
mod css;
mod dart;
mod elixir;
mod go;
mod html;
mod java;
mod javascript;
mod json;
mod kotlin;
mod markdown;
mod objective_c;
mod php;
mod python;
mod ruby;
mod rust;
mod scala;
mod sql;
mod strings;
mod swift;
mod yaml;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::strategery::Strategery;
pub use code::{FormatResult, LintResult, Results};
use regex::Regex;

lazy_static! {
    static ref FULL_DATE_RE: Regex = regexp!(
        "{}",
        r"[ ]{0,}\d+[ ]{0,}年 [ ]{0,}\d+[ ]{0,}月 [ ]{0,}\d+[ ]{0,}[日号][ ]{0,}"
    );
    static ref CJK_RE: Regex = regexp!("{}", r"\p{CJK}");
    static ref SPACE_RE: Regex = regexp!("{}", r"[ ]");
    static ref DASH_HANS_RE: Regex = regexp!("{}", r"([\p{CJK}）】」》”’])([\-]+)([\p{CJK}}}（【「《“‘])");
    // （）【】「」《》
    static ref LEFT_QUOTE_RE: Regex = regexp!("{}", r" ([（【「《])");
    static ref RIGHT_QUOTE_RE: Regex = regexp!("{}", r"([）】」》]) ");
    // Strategies all rules
    static ref STRATEGIES: Vec<Strategery> = vec![
        // EnglishLetter, Number
        // But not start with %, $, \ for avoid change %s, %d, $1, $2, \d, \r, \p ... in source code
        Strategery::new(r"\p{CJK}[^%\$\\]", r"[a-zA-Z0-9]", true, false),
        Strategery::new(r"[^%\$\\][a-zA-Z0-9]", r"\p{CJK}", true, false),
        // 10%中文
        Strategery::new(r"[0-9][%]", r"\p{CJK}", true, false),
        // SpecialSymbol
        Strategery::new(r"\p{CJK}", r"[\|+]", true, true),
        // @ after CJK, not not before, 你好 @某某
        Strategery::new(r"\p{CJK}", r"[@]", true, false),
        Strategery::new(r"\p{CJK}", r"[\[\(‘“]", true, false),
        Strategery::new(r"[’”\]\)!]", r"\p{CJK}", true, false),

        // FullwidthPunctuation remove space case, Fullwidth can safe to remove spaces
        Strategery::new(r"[\w\p{CJK}]", r"[，。！？：；）」》】”’]", false, true),
        Strategery::new(r"[‘“【「《（]", r"[\w\p{CJK}]", false, true),
    ];
}

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
#[wasm_bindgen]
pub fn format(text: &str) -> String {
    let mut out = String::from(text);

    // skip if not has CJK
    if !CJK_RE.is_match(text) {
        return out;
    }

    out = fullwidth::fullwidth(&out);
    out = halfwidth::halfwidth(&out);

    for rule in STRATEGIES.iter() {
        out = rule.format(&out)
    }

    out = space_dash_with_hans(&out);

    out
}

/// Format a html content.
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
#[wasm_bindgen(js_name = "formatHTML")]
pub fn format_html(html_str: &str) -> String {
    html::format_html(html_str).to_string()
}

/// Lint a file content with filetype.
///
/// Example:
///
/// ```
//  extern crate autocorrect;
//
/// let raw = r#"
/// <article>
///   <h1>这是 Heading 标题</h1>
///   <div class="content">
///     <p>你好 Rust 世界<strong>Bold 文本</strong></p>
///     <p>这是第二行 p 标签</p>
///   </div>
/// </article>
/// "#;
/// autocorrect::lint_for(raw, "html");
/// autocorrect::lint_for(raw, "index.html");
/// ```
pub fn lint_for(raw: &str, filename_or_ext: &str) -> code::LintResult {
    let mut result = match types::match_filename(filename_or_ext) {
        "html" => html::lint_html(raw),
        "yaml" => yaml::lint_yaml(raw),
        "sql" => sql::lint_sql(raw),
        "rust" => rust::lint_rust(raw),
        "ruby" => ruby::lint_ruby(raw),
        "elixir" => elixir::lint_elixir(raw),
        "go" => go::lint_go(raw),
        "javascript" => javascript::lint_javascript(raw),
        "css" => css::lint_css(raw),
        "json" => json::lint_json(raw),
        "python" => python::lint_python(raw),
        "objective_c" => objective_c::lint_objective_c(raw),
        "strings" => strings::lint_strings(raw),
        "csharp" => csharp::lint_csharp(raw),
        "swift" => swift::lint_swift(raw),
        "java" => java::lint_java(raw),
        "scala" => scala::lint_scala(raw),
        "kotlin" => kotlin::lint_kotlin(raw),
        "php" => php::lint_php(raw),
        "dart" => dart::lint_dart(raw),
        "markdown" => markdown::lint_markdown(raw),
        "text" => markdown::lint_markdown(raw),
        _ => LintResult::new(raw),
    };

    result.filepath = String::from(filename_or_ext);

    result
}

/// Format a file content with filetype.
///
/// Example:
///
/// ```
//  extern crate autocorrect;
//
/// let raw = r#"
/// <article>
///   <h1>这是 Heading 标题</h1>
///   <div class="content">
///     <p>你好 Rust 世界<strong>Bold 文本</strong></p>
///     <p>这是第二行 p 标签</p>
///   </div>
/// </article>
/// "#;
/// autocorrect::format_for(raw, "html");
/// autocorrect::format_for(raw, "index.html");
/// ```
pub fn format_for(raw: &str, filename_or_ext: &str) -> code::FormatResult {
    let result = match types::match_filename(filename_or_ext) {
        "html" => html::format_html(raw),
        "yaml" => yaml::format_yaml(raw),
        "sql" => sql::format_sql(raw),
        "rust" => rust::format_rust(raw),
        "ruby" => ruby::format_ruby(raw),
        "elixir" => elixir::format_elixir(raw),
        "go" => go::format_go(raw),
        "javascript" => javascript::format_javascript(raw),
        "css" => css::format_css(raw),
        "json" => json::format_json(raw),
        "python" => python::format_python(raw),
        "objective_c" => objective_c::format_objective_c(raw),
        "strings" => strings::format_strings(raw),
        "csharp" => csharp::format_csharp(raw),
        "swift" => swift::format_swift(raw),
        "java" => java::format_java(raw),
        "scala" => scala::format_scala(raw),
        "kotlin" => kotlin::format_kotlin(raw),
        "php" => php::format_php(raw),
        "dart" => dart::format_dart(raw),
        "markdown" => markdown::format_markdown(raw),
        "text" => markdown::format_markdown(raw),
        _ => FormatResult::new(raw),
    };

    result
}

/// Format content with filetype, and return a json result.
#[wasm_bindgen(js_name = "formatFor")]
pub fn format_for_json_out(raw: &str, filename_or_ext: &str) -> wasm_bindgen::JsValue {
    let result = format_for(raw, filename_or_ext);
    wasm_bindgen::JsValue::from_serde(&result).unwrap()
}

/// Lint content with filetype, and return a json result.
#[wasm_bindgen(js_name = "lintFor")]
pub fn lint_for_json_out(raw: &str, filename_or_ext: &str) -> wasm_bindgen::JsValue {
    let result = lint_for(raw, filename_or_ext);
    wasm_bindgen::JsValue::from_serde(&result).unwrap()
}

fn space_dash_with_hans(text: &str) -> String {
    let mut out = String::from(text);

    // 自由 - 开放
    out = (&DASH_HANS_RE.replace_all(&out, "$1 $2 $3")).to_string();
    out = (&LEFT_QUOTE_RE.replace_all(&out, "$1")).to_string();
    out = (&RIGHT_QUOTE_RE.replace_all(&out, "$1")).to_string();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn assert_cases(cases: HashMap<&str, &str>) {
        for (source, exptected) in cases.into_iter() {
            let actual = format(source);
            assert_eq!(exptected, actual);
        }
    }

    #[test]
    fn it_format() {
        let cases = map![
            "!sm" => "!sm",
            "Hello world!" => "Hello world!",
            "部署到heroku有问题网页不能显示" => "部署到 heroku 有问题网页不能显示",
            "[北京]美企聘site/web大型应用开发高手-Ruby" => "[北京] 美企聘 site/web 大型应用开发高手-Ruby",
            "[成都](团800)招聘Rails工程师" => "[成都](团 800) 招聘 Rails 工程师",
            "Teahour.fm第18期发布" => "Teahour.fm 第 18 期发布",
            "Yes!升级到了Rails 4" => "Yes！升级到了 Rails 4",
            "WWDC上讲到的Objective C/LLVM改进" => "WWDC 上讲到的 Objective C/LLVM 改进",
            "在Ubuntu11.10 64位系统安装newrelic出错" => "在 Ubuntu11.10 64 位系统安装 newrelic 出错",
            "升级了macOS 10.9 附遇到的Bug概率有0.1%或更少" => "升级了 macOS 10.9 附遇到的 Bug 概率有 0.1% 或更少",
            "在做Rails 3.2 Tutorial第Chapter 9.4.2遇到一个问题求助！" => "在做 Rails 3.2 Tutorial 第 Chapter 9.4.2 遇到一个问题求助！",
            "发现macOS安装软件新方法：Homebrew" => "发现 macOS 安装软件新方法：Homebrew",
            "without looking like it’s been marked up with tags or formatting instructions." => "without looking like it’s been marked up with tags or formatting instructions.",
            "隔夜SHIBOR报1.5530%，上涨33.80%个基点。7天SHIBOR报2.3200%，上涨6.10个基点。3个月SHIBOR报2.8810%，下降1.80个" => "隔夜 SHIBOR 报 1.5530%，上涨 33.80% 个基点。7 天 SHIBOR 报 2.3200%，上涨 6.10 个基点。3 个月 SHIBOR 报 2.8810%，下降 1.80 个",
            "野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元" => "野村：重申吉利汽车 (00175)“买入” 评级 上调目标价至 17.9 港元",
            "小米集团-W调整目标价为13.5港币" => "小米集团-W 调整目标价为 13.5 港币",
            "（路透社）-预计全年净亏损约1.3亿港元*预期因出售汽车" => "（路透社）- 预计全年净亏损约 1.3 亿港元*预期因出售汽车",
            "（路透社）-预计全年净亏损约1.3亿\n\n港元*预期因出售汽车" => "（路透社）- 预计全年净亏损约 1.3 亿\n\n港元*预期因出售汽车",
            "Cell或RefCell类型使用某种形式的*内部" => "Cell 或 RefCell 类型使用某种形式的*内部",
            "Cell或RefCell类型使用某种形式的*内部可变性*" => "Cell 或 RefCell 类型使用某种形式的*内部可变性*"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_specials() {
        let cases = map![
            "记事本,记事本显示阅读次数#149" => "记事本，记事本显示阅读次数#149",
            "HashTag的演示#标签" => "HashTag 的演示#标签",
            "HashTag 的演示#标签#演示" =>         "HashTag 的演示#标签#演示",
            "Mention里面有关于中文的@某某人" =>        "Mention 里面有关于中文的 @某某人",
            "里面用@foo符号的话后面的变量名会被替换成userN" => "里面用 @foo 符号的话后面的变量名会被替换成 userN",
            "Dollar的演示$阿里巴巴.US$股票标签" =>    "Dollar 的演示$阿里巴巴.US$股票标签",
            "测试英文,逗号Comma转换." =>    "测试英文，逗号 Comma 转换。",
            "测试英文,Comma逗号转换." =>    "测试英文，Comma 逗号转换。",
            "英文,逗号后面.阿里巴巴.US有空格?的情况!测试" =>    "英文，逗号后面。阿里巴巴.US 有空格？的情况！测试",
            "你好hello?world!" =>    "你好 hello?world!",
            "search by%关键词%" => "search by%关键词%"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_programming() {
        let cases = map![
            "内容带有\n不会处理" => "内容带有\n不会处理",
            "内容带有%s或%d或%v特殊字符，或者%S或%D或%V这些特殊format字符" => "内容带有%s或%d或%v特殊字符，或者%S或%D或%V这些特殊 format 字符",
            "内容带有$1或$2或$3特殊字符" => "内容带有$1或$2或$3特殊字符"
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
            "22号开始出发" =>                          "22 号开始出发"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_english_letter() {
        let cases = map![
            "长桥 LongBridge App 下载" => "长桥 LongBridge App 下载"
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
            "公告:(美股)阿里巴巴[BABA.US]发布2019下半年财报!" =>          "公告:(美股) 阿里巴巴 [BABA.US] 发布 2019 下半年财报！",
            "消息http://github.com解禁了" =>                     "消息 http://github.com 解禁了",
            "美股异动|阿帕奇石油(APA.US)盘前涨超15% 在苏里南近海发现大量石油" =>     "美股异动 | 阿帕奇石油 (APA.US) 盘前涨超 15% 在苏里南近海发现大量石油",
            "美国统计局：美国11月原油出口下降至302.3万桶/日，10月为338.3万桶/日。" => "美国统计局：美国 11 月原油出口下降至 302.3 万桶/日，10 月为 338.3 万桶/日。",
            "[b]Foo bar dar[/b]" => "[b]Foo bar dar[/b]"
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_format_for_fullwidth_symbols() {
        let cases = map![
            "（美股）市场：发布「最新」100消息【BABA.US】“大涨”50%；同比上涨20%！" => "（美股）市场：发布「最新」100 消息【BABA.US】“大涨”50%；同比上涨 20%！",
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

    #[test]
    fn it_format_for_cjk() {
        let cases = map![
            "全世界已有数百家公司在生产环境中使用Rust，以达到快速、跨平台、低资源占用的目的。很多著名且受欢迎的软件，例如Firefox、 Dropbox和Cloudflare都在使用Rust。" => "全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。很多著名且受欢迎的软件，例如 Firefox、 Dropbox 和 Cloudflare 都在使用 Rust。",
            "現今全世界上百家公司企業為了尋求快速、節約資源而且能跨平台的解決辦法，都已在正式環境中使用Rust。許多耳熟能詳且受歡迎的軟體，諸如Firefox、Dropbox以及Cloudflare都在使用Rust。" => "現今全世界上百家公司企業為了尋求快速、節約資源而且能跨平台的解決辦法，都已在正式環境中使用 Rust。許多耳熟能詳且受歡迎的軟體，諸如 Firefox、Dropbox 以及 Cloudflare 都在使用 Rust。",
            "既に、世界中の数百という企業がRustを採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。皆さんがご存じで愛用しているソフトウェア、例えばFirefox、DropboxやCloudflareも、Rustを採用しています。" => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。皆さんがご存じで愛用しているソフトウェア、例えば Firefox、Dropbox や Cloudflare も、Rust を採用しています。",
            "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다. Firefox, Dropbox 및 Cloudflare와 같이 잘 알려져 있고 널리 사용되는 많은 소프트웨어가 Rust를 사용하고 있습니다." => "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다. Firefox, Dropbox 및 Cloudflare 와 같이 잘 알려져 있고 널리 사용되는 많은 소프트웨어가 Rust 를 사용하고 있습니다."
        ];

        assert_cases(cases);
    }

    #[test]
    fn it_lint_for() {
        let raw = "<p>Hello你好</p>";
        let result = lint_for(raw, "foo.bar.html");
        let expect_json = r#"{"filepath":"foo.bar.html","lines":[{"l":1,"c":4,"new":"Hello 你好","old":"Hello你好"}],"error":""}"#;
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
}
