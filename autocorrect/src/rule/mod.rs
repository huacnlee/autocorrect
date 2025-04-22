// autocorrect: false
mod fullwidth;
#[allow(clippy::module_inception)]
mod rule;
mod strategery;
mod word;

pub mod halfwidth;
pub mod spellcheck;

use std::{borrow::Cow, collections::HashMap};

use regex::Regex;
use rule::{Rule, RuleResult};

use crate::result::Severity;

lazy_static! {
    static ref RULES: Vec<Rule> = vec![
        // Rule: space-word
        Rule::new("space-word", word::format_space_word),
        // Rule: space-punctuation
        Rule::new("space-punctuation", word::format_space_punctuation),
        // Rule: space-bracket
        Rule::new("space-bracket", word::format_space_bracket),
        // Rule: space-dash
        Rule::new("space-dash", word::format_space_dash),
        // Rule: space-backticks
        Rule::new("space-backticks", word::format_space_backticks),
        // Rule: space-dollar
        Rule::new("space-dollar", word::format_space_dollar),
        // Rule: fullwidth
        Rule::new("fullwidth", fullwidth::format),
    ];

    static ref AFTER_RULES: Vec<Rule> = vec![
        // Rule: halfwidth-word
        Rule::new("halfwidth-word", halfwidth::format_word),
        // Rule: halfwidth punctuations
        Rule::new("halfwidth-punctuation", halfwidth::format_punctuation),
        // Rule: no-space-fullwidth
        Rule::new("no-space-fullwidth", word::format_no_space_fullwidth),
        // Rule: no-space-fullwidth-quote
        Rule::new("no-space-fullwidth-quote", word::format_no_space_fullwidth_quote),
        Rule::new("spellcheck", spellcheck::format),
    ];
}

lazy_static! {
    static ref FULL_DATE_RE: Regex = regexp!(
        r"[ ]{0,}\d+[ ]{0,}年 [ ]{0,}\d+[ ]{0,}月 [ ]{0,}\d+[ ]{0,}[日号][ ]{0,}"
    );
    pub static ref CJK_RE: Regex = regexp!(r"\p{CJK}");
    static ref SPACE_RE: Regex = regexp!(r"[ ]");
    /// Match string start with Path or URL:
    ///
    /// ```ignore
    /// http://
    /// https://
    /// mailto://
    /// app://
    /// /foo/bar/dar
    /// ignore //foo/bar/dar
    /// ```
    static ref PATH_RE: Regex = regexp!(r"(^[a-zA-Z\d]+://)|(^/?[a-zA-Z\d\-_\.]{2,}/)");
    /// Match string is path with hash, e.g.: `foo-a_01.html#测试test`
    static ref PATH_HASH_RE: Regex = regexp!(r"[a-zA-Z0-9\-_.]+#[\w\-_.]*[\p{Han}]+[a-zA-Z0-9\-_.]*");
}

/// Get all rule names for default enable
#[allow(dead_code)]
pub fn default_rule_names() -> Vec<String> {
    let mut rule_names = vec![];
    RULES.iter().for_each(|r| rule_names.push(r.name.clone()));
    AFTER_RULES
        .iter()
        .for_each(|r| rule_names.push(r.name.clone()));

    rule_names
}

pub(crate) fn format_or_lint(text: &str, lint: bool) -> RuleResult {
    format_or_lint_with_disable_rules(text, lint, &map![])
}

pub(crate) fn format_or_lint_with_disable_rules<'a>(
    text: &'a str,
    lint: bool,
    disable_rules: &HashMap<String, bool>,
) -> RuleResult<'a> {
    let mut result = RuleResult::default();

    // skip if not has CJK
    if CJK_RE.is_match(text) {
        let mut part = String::new();
        for ch in text.chars() {
            part.push(ch);

            // Is next char is newline or space, break part to format
            if matches!(ch, ' ' | '\n' | '\r') {
                let mut sub_result = RuleResult::new(&part);
                sub_result.severity = result.severity;
                format_part(&mut sub_result, lint, disable_rules);

                let mut out = result.out.into_owned();
                out.push_str(&sub_result.out);
                result.out = Cow::Owned(out);
                result.severity = sub_result.severity;
                part.clear();
            }
        }

        if !part.is_empty() {
            let mut sub_result = RuleResult::new(&part);
            sub_result.severity = result.severity;

            format_part(&mut sub_result, lint, disable_rules);

            let mut out = result.out.into_owned();
            out.push_str(&sub_result.out);
            result.out = Cow::Owned(out);
            result.severity = sub_result.severity;
        }
    } else {
        result.out = Cow::Borrowed(text);
    }

    format_after_rules(&mut result, lint, disable_rules);

    result
}

fn format_part(result: &mut RuleResult, lint: bool, disable_rules: &HashMap<String, bool>) {
    if is_match_path(&result.out) || is_match_path_hash(&result.out) {
        return;
    }

    let raw = result.out.clone();

    for rule in RULES
        .iter()
        .filter(|r| !disable_rules.get(r.name.as_str()).unwrap_or(&false))
    {
        if lint {
            rule.lint(result);
        } else {
            rule.format(result);
        }
    }

    // Check textRules to change result
    for (text, mode) in crate::Config::current().text_rules.iter() {
        if raw.contains(text) {
            match mode {
                crate::config::SeverityMode::Off => {
                    result.severity = Severity::Pass;
                    result.out = raw;
                    return;
                }
                crate::config::SeverityMode::Warning => {
                    if lint {
                        result.severity = Severity::Warning;
                    } else {
                        result.severity = Severity::Pass;
                        result.out = raw;
                    }
                    return;
                }
                _ => {}
            }
        }
    }
}

fn format_after_rules(result: &mut RuleResult, lint: bool, disable_rules: &HashMap<String, bool>) {
    for rule in AFTER_RULES
        .iter()
        .filter(|r| !disable_rules.get(r.name.as_str()).unwrap_or(&false))
    {
        if lint {
            rule.lint(result);
        } else {
            rule.format(result);
        }
    }
}

fn is_match_path_hash(text: &str) -> bool {
    PATH_HASH_RE.is_match(text.trim())
}

fn is_match_path(text: &str) -> bool {
    PATH_RE.is_match(text)
}

#[cfg(test)]
mod tests {
    use crate::result::Severity;

    use super::*;

    #[test]
    fn test_default_rule_names() {
        let rule_names = default_rule_names();
        let expect = vec![
            "space-word",
            "space-punctuation",
            "space-bracket",
            "space-dash",
            "space-backticks",
            "space-dollar",
            "fullwidth",
            "halfwidth-word",
            "halfwidth-punctuation",
            "no-space-fullwidth",
            "no-space-fullwidth-quote",
            "spellcheck",
        ];
        assert_eq!(expect, rule_names);
    }

    #[test]
    fn test_format_part() {
        let mut result = RuleResult::new("Hello世界.");
        format_part(&mut result, false, &map!());
        assert_eq!("Hello 世界。", result.out);
        assert_eq!(Severity::Error, result.severity);

        let mut result = RuleResult::new("Hello世界.");
        format_part(&mut result, true, &map!());
        assert_eq!("Hello 世界。", result.out);
        assert_eq!(Severity::Error, result.severity);

        let mut result = RuleResult::new("Hello 世界。");
        format_part(&mut result, true, &map!());
        assert_eq!("Hello 世界。", result.out);
        assert_eq!(Severity::Pass, result.severity);
    }

    #[test]
    fn test_format_after_rules() {
        crate::config::setup_test();

        let mut result = RuleResult::new("测试 ios 应用， 与技术");
        format_after_rules(&mut result, false, &map!());
        assert_eq!("测试 ios 应用，与技术", result.out);
        assert_eq!(Severity::Error, result.severity);

        let mut result = RuleResult::new("测试 ios 应用， 与技术");
        format_after_rules(&mut result, true, &map!());
        assert_eq!("测试 iOS 应用，与技术", result.out);
        assert_eq!(Severity::Error, result.severity);
    }

    #[test]
    fn test_format_or_lint() {
        crate::config::setup_test();

        let result = format_or_lint("测试ios应用， 与技术", false);
        assert_eq!("测试 ios 应用，与技术", result.out);
        assert_eq!(Severity::Error, result.severity);

        let result = format_or_lint("测试ios应用， 与技术", true);
        assert_eq!("测试 iOS 应用，与技术", result.out);
        assert_eq!(Severity::Error, result.severity);

        // Pass case
        let result = format_or_lint("测试 iOS 应用，与技术", false);
        assert_eq!("测试 iOS 应用，与技术", result.out);
        assert_eq!(Severity::Pass, result.severity);

        let result = format_or_lint("测试 iOS 应用，与技术", true);
        assert_eq!("测试 iOS 应用，与技术", result.out);
        assert_eq!(Severity::Pass, result.severity);
    }

    #[test]
    fn test_rules() {
        crate::config::setup_test();

        let cases = map! {
            "hello你好" => (map!{}, "hello 你好"),
            "hello你好 “Quote” 和 ‘Single Quote’ 测试0" => (map!{
                "no-space-fullwidth-quote" => true,
            }, "hello 你好 “Quote” 和 ‘Single Quote’ 测试 0"),
            "hello你好 “Quote” 和 ‘Single Quote’ 测试1" => (map!{}, "hello 你好“Quote”和‘Single Quote’测试 1"),
            "你好-世界" => (map!{}, "你好 - 世界"),
            "世界-你好" => (map!{"space-dash" => true}, "世界-你好"),
            "1你好[世界]" => (map!{ }, "1 你好 [世界]"),
            "2你好[世界]" => (map!{ "space-bracket" => true }, "2 你好[世界]"),
            "代码`code`例子1" => (map!{}, "代码 `code` 例子 1"),
            "代码`code`例子2" => (map!{ "space-backticks" => true }, "代码`code`例子 2"),
            "变量$x$表示了" => (map!{}, "变量 $x$ 表示了"),
            "变量$x$表示了" => (map!{ "space-dollar" => true}, "变量$x$表示了"),
            "测试 ，" => (map!{}, "测试，"),
            "测试 ，" => (map!{"no-space-fullwidth"=>true}, "测试 ，"),
        };

        for (input, (disable_rules, expect)) in cases {
            let out = format_or_lint_with_disable_rules(
                input,
                false,
                &disable_rules
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            );
            assert_eq!(expect, out.out);
        }
    }

    #[test]
    fn test_is_match_path() {
        let no_match_cases = cases! {r#"
            //foo/bar/foo_bar
            foo /foo/bar/foo_bar
            hello world
            你好啊
            逐步改善你的C/C++/Zig代码库
            测试 C/C++代码
        "#};
        for case in no_match_cases {
            assert!(
                !is_match_path(case),
                "expected `{}` to not match, but matched.",
                case
            );
        }

        let match_cases = cases! {r#"
            http://google.com/foo-bar_01.htm
            http://google.com/foo/bar_01?a=1&b=2#foo
            app://foo.com/bar.1
            /foo/bar/foo_bar
            /.foo/bar/foo_bar
            /foo/bar/foo_bar/
            foo/bar/dar
        "#};
        for case in match_cases {
            assert!(
                is_match_path(case),
                "expected `{}` to be match, but not.",
                case
            );
        }
    }

    #[test]
    fn test_is_match_path_hash() {
        let no_match_cases = cases! {r#"
            演示#标签
            HashTag的演示#标签1
            foo bar #符号
            记事本,记事本1显示阅读次数#149号
            逐步改善你的C/C++/Zig代码库
        "#};
        for case in no_match_cases {
            assert!(
                !is_match_path_hash(case),
                "expected `{}` to not match, but matched.",
                case
            );
        }

        let match_cases = cases! {r#"
            foo-bar_01.htm#测试copy
            foo-bar_01#copy测试
             foo-bar#测试
            Foo_bar#a测试
            foo.Bar#测A试1
        "#};
        for case in match_cases {
            assert!(
                is_match_path_hash(case),
                "expected `{}` to be match, but not.",
                case
            );
        }
    }
}
