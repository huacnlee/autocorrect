use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./config/toggle.pest"]
pub struct ToggleParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Toggle {
    None,
    // Empty to disable all
    Disable(Vec<String>),
    // Empty to enable all
    Enable(Vec<String>),
}

impl Default for Toggle {
    fn default() -> Self {
        Toggle::Enable(vec![])
    }
}

impl Toggle {
    pub fn match_rule(&self, rule_name: &str) -> Option<bool> {
        match self {
            Toggle::None => None,
            Toggle::Disable(rules) => {
                if rules.is_empty() {
                    Some(false)
                } else {
                    Some(!rules.contains(&rule_name.to_string()))
                }
            }
            Toggle::Enable(rules) => {
                if rules.is_empty() {
                    Some(true)
                } else {
                    Some(rules.contains(&rule_name.to_string()))
                }
            }
        }
    }

    pub fn disable_rules(&self) -> HashMap<String, bool> {
        match self {
            Toggle::Disable(rules) => {
                let mut map = HashMap::new();
                for rule in rules {
                    map.insert(rule.to_string(), true);
                }
                map
            }
            _ => HashMap::new(),
        }
    }

    // Merge two toggle if it posible, otherwise override
    pub fn merge(&mut self, new_toggle: Self) {
        match new_toggle {
            Toggle::Disable(rules) => {
                if let Toggle::Disable(old_rules) = self {
                    if !old_rules.is_empty() {
                        old_rules.extend(rules.clone());
                    }

                    if rules.is_empty() {
                        old_rules.clear();
                    }
                } else {
                    *self = Toggle::Disable(rules);
                }
            }
            Toggle::Enable(rules) => {
                if let Toggle::Enable(old_rules) = self {
                    if !old_rules.is_empty() {
                        old_rules.extend(rules.clone());
                    }

                    if rules.is_empty() {
                        old_rules.clear();
                    }
                } else {
                    *self = Toggle::Enable(rules);
                }
            }
            _ => *self = new_toggle,
        }
    }
}

pub fn parse(input: &str) -> Toggle {
    if let Ok(pairs) = ToggleParser::parse(Rule::item, input) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::disable => {
                    let mut rules = vec![];
                    for pair in pair.into_inner() {
                        if pair.as_rule() == Rule::rule_name {
                            rules.push(pair.as_str().to_lowercase().to_owned());
                        }
                    }

                    return Toggle::Disable(rules);
                }
                Rule::enable => {
                    let mut rules = vec![];
                    for pair in pair.into_inner() {
                        if pair.as_rule() == Rule::rule_name {
                            rules.push(pair.as_str().to_lowercase().to_owned());
                        }
                    }
                    return Toggle::Enable(rules);
                }
                _ => {}
            }
        }
    }

    Toggle::None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_match_rule() {
        assert_eq!(Toggle::Enable(vec![]).match_rule("rule"), Some(true));
        assert_eq!(Toggle::Enable(vec![]).match_rule("foo"), Some(true));
        assert_eq!(Toggle::Enable(vec![]).match_rule(""), Some(true));

        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned()]).match_rule("foo"),
            Some(true)
        );
        assert_eq!(
            Toggle::Enable(vec!["bar".to_owned()]).match_rule("foo"),
            Some(false)
        );
        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]).match_rule("foo"),
            Some(true)
        );
        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]).match_rule("bar"),
            Some(true)
        );
        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]).match_rule("dar"),
            Some(false)
        );
        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]).match_rule(""),
            Some(false)
        );
    }

    #[test]
    fn it_parse() {
        assert_eq!(Toggle::Enable(vec![]), parse("autocorrect-enable"));
        assert_eq!(Toggle::Enable(vec![]), parse("// autocorrect-enable"));
        assert_eq!(Toggle::Enable(vec![]), parse("# autocorrect-enable"));
        assert_eq!(Toggle::Enable(vec![]), parse("# autocorrect: true"));
        assert_eq!(Toggle::Enable(vec![]), parse("# autocorrect:true"));
        assert_eq!(Toggle::Disable(vec![]), parse("# autocorrect: false"));
        assert_eq!(Toggle::Disable(vec![]), parse("# autocorrect:false"));
        assert_eq!(Toggle::Disable(vec![]), parse("# autocorrect-disable"));
        assert_eq!(Toggle::Disable(vec![]), parse("// autocorrect-disable"));
        assert_eq!(Toggle::None, parse("// hello world"));
    }

    #[test]
    fn it_parse_with_rules() {
        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned()]),
            parse("autocorrect-enable foo")
        );

        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]),
            parse("// autocorrect-enable foo, bar")
        );
        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]),
            parse("// autocorrect-enable foo,bar")
        );

        assert_eq!(
            Toggle::Disable(vec!["foo".to_owned()]),
            parse("# autocorrect-disable foo")
        );
        assert_eq!(
            Toggle::Disable(vec!["foo".to_owned(), "bar".to_owned()]),
            parse("// autocorrect-disable foo,bar")
        );
        assert_eq!(
            Toggle::Disable(vec![
                "foo".to_owned(),
                "bar".to_owned(),
                "foo-bar_dar".to_owned()
            ]),
            parse("// autocorrect-disable foo,Bar, Foo-bAr_dar")
        );
    }

    #[test]
    fn test_disable_rules() {
        // disable_rules
        assert_eq!(
            Some(&true),
            parse("// autocorrect-disable foo,Bar, Foo-bAr_dar")
                .disable_rules()
                .get("foo")
        );
        assert_eq!(
            Some(&true),
            parse("// autocorrect-disable foo,Bar, Foo-bAr_dar")
                .disable_rules()
                .get("bar")
        );
        assert_eq!(
            Some(&true),
            parse("// autocorrect-disable foo,Bar, Foo-bAr_dar")
                .disable_rules()
                .get("foo-bar_dar")
        );
        assert_eq!(
            None,
            parse("// autocorrect-disable foo,Bar, Foo-bAr_dar")
                .disable_rules()
                .get("foo-bar")
        );
    }

    #[test]
    fn test_merge() {
        let mut toggle = Toggle::Enable(vec!["foo".to_owned()]);
        toggle.merge(Toggle::Enable(vec!["bar".to_owned()]));
        assert_eq!(
            Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]),
            toggle
        );
        toggle.merge(Toggle::Enable(vec![]));
        assert_eq!(Toggle::Enable(vec![]), toggle);
        toggle.merge(Toggle::Enable(vec!["foo".to_owned()]));
        assert_eq!(Toggle::Enable(vec![]), toggle);

        let mut toggle = Toggle::Disable(vec!["foo".to_owned(), "bar".to_owned()]);
        toggle.merge(Toggle::Disable(vec!["dar".to_owned()]));
        assert_eq!(
            Toggle::Disable(vec!["foo".to_owned(), "bar".to_owned(), "dar".to_owned()]),
            toggle
        );
        toggle.merge(Toggle::Disable(vec![]));
        assert_eq!(Toggle::Disable(vec![]), toggle);
        toggle.merge(Toggle::Disable(vec!["foo".to_owned()]));
        assert_eq!(Toggle::Disable(vec![]), toggle);

        // Merge with disable enum value, override
        let mut toggle = Toggle::Enable(vec!["foo".to_owned(), "bar".to_owned()]);
        toggle.merge(Toggle::Disable(vec!["dar".to_owned()]));
        assert_eq!(Toggle::Disable(vec!["dar".to_owned()]), toggle);
        toggle.merge(Toggle::None);
        assert_eq!(Toggle::None, toggle);

        let mut toggle = Toggle::Disable(vec!["foo".to_owned(), "bar".to_owned()]);
        toggle.merge(Toggle::Enable(vec!["dar".to_owned()]));
        assert_eq!(Toggle::Enable(vec!["dar".to_owned()]), toggle);
        toggle.merge(Toggle::None);
        assert_eq!(Toggle::None, toggle);
    }
}
