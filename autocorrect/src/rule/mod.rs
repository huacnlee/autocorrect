mod fullwidth;
mod halfwidth;
mod rule;
mod strategery;
mod word;

use regex::Regex;
use rule::Rule;

lazy_static! {
    static ref RULES: Vec<Rule> = vec![
        // Rule: space-word
        Rule::new("space-word", word::format_space_word),
        // Rule: space-punctuation
        Rule::new("space-punctuation", word::format_space_punctuation),
        // Rule: fullwidth
        Rule::new("fullwidth", fullwidth::format),
        // Rule: halfwidth
        Rule::new("halfwidth", halfwidth::format),
    ];

    static ref AFTER_RULES: Vec<Rule> = vec![
        // Rule: no-space-fullwidth
        Rule::new("no-space-fullwidth", word::format_no_space_fullwidth),
    ];
}

lazy_static! {
    static ref FULL_DATE_RE: Regex = regexp!(
        "{}",
        r"[ ]{0,}\d+[ ]{0,}年 [ ]{0,}\d+[ ]{0,}月 [ ]{0,}\d+[ ]{0,}[日号][ ]{0,}"
    );
    static ref CJK_RE: Regex = regexp!("{}", r"\p{CJK}");
    static ref SPACE_RE: Regex = regexp!("{}", r"[ ]");
    // start with Path or URL http://, https://, mailto://, app://, /foo/bar/dar, without //foo/bar/dar
    static ref PATH_RE: Regex = regexp!("{}", r"^(([a-z\d]+)://)|(^/?[\w\d\-]+/)");
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

pub(crate) fn format_or_lint(text: &str, lint: bool) -> String {
    // skip if not has CJK
    if !CJK_RE.is_match(text) {
        return String::from(text);
    }

    let mut out: String = String::new();
    let mut part = String::new();
    for ch in text.chars() {
        part.push(ch);

        // Is next char is newline or space, break part to format
        if ch == ' ' || ch == '\n' || ch == '\r' {
            let new_part = part.clone();
            part.clear();

            out.push_str(&format_part(&new_part, lint));
        }
    }

    if !part.is_empty() {
        out.push_str(&format_part(&part, lint));
    }

    format_after_rules(&out, lint)
}

fn format_part(text: &str, lint: bool) -> String {
    if !CJK_RE.is_match(text) {
        return String::from(text);
    }

    if PATH_RE.is_match(text) {
        return String::from(text);
    }

    format_rules(&text, lint)
}

fn format_rules(input: &str, lint: bool) -> String {
    let mut out = input.to_string();

    for rule in RULES.iter() {
        if lint {
            out = rule.lint(&out);
        } else {
            out = rule.format(&out);
        }
    }

    out
}

fn format_after_rules(input: &str, lint: bool) -> String {
    let mut out = input.to_string();

    for rule in AFTER_RULES.iter() {
        if lint {
            out = rule.lint(&out);
        } else {
            out = rule.format(&out);
        }
    }

    out
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rule_names() {
        let rule_names = default_rule_names();
        let expect = vec![
            "space-word",
            "space-punctuation",
            "fullwidth",
            "halfwidth",
            "no-space-fullwidth",
        ];
        assert_eq!(expect, rule_names);
    }
}
