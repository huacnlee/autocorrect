mod fullwidth;
mod halfwidth;
mod rule;
mod strategery;
mod word;

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

pub fn format_rules(input: &str) -> String {
    let mut out = input.to_string();

    for rule in RULES.iter() {
        out = rule.format(&out);
    }

    out
}

pub fn format_after_rules(input: &str) -> String {
    let mut out = input.to_string();

    for rule in AFTER_RULES.iter() {
        out = rule.format(&out);
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
