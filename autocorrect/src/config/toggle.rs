use std::{collections::HashMap, sync::Arc};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./config/toggle.pest"]
pub struct ToggleParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Toggle {
    None,
    // Empty to disable all
    Disable(Arc<HashMap<String, bool>>),
    // Empty to enable all
    Enable(Arc<HashMap<String, bool>>),
}

impl Default for Toggle {
    fn default() -> Self {
        Toggle::enable(vec![])
    }
}

impl Toggle {
    pub fn none() -> Self {
        Toggle::None
    }

    pub fn enable(rules: Vec<&str>) -> Self {
        let rules = rules
            .into_iter()
            .map(|r| (r.to_lowercase().to_string(), true))
            .collect();
        Toggle::Enable(Arc::new(rules))
    }

    pub fn disable(rules: Vec<&str>) -> Self {
        let rules = rules
            .into_iter()
            .map(|r| (r.to_lowercase().to_string(), true))
            .collect();
        Toggle::Disable(Arc::new(rules))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Toggle::None)
    }

    pub fn match_rule(&self, rule_name: &str) -> Option<bool> {
        match self {
            Toggle::None => None,
            Toggle::Disable(rules) => {
                if rules.is_empty() {
                    Some(false)
                } else {
                    Some(!rules.contains_key(rule_name))
                }
            }
            Toggle::Enable(rules) => {
                if rules.is_empty() {
                    Some(true)
                } else {
                    Some(rules.contains_key(rule_name))
                }
            }
        }
    }

    pub fn disable_rules(&self) -> Arc<HashMap<String, bool>> {
        match self {
            Toggle::Disable(rules) => rules.clone(),
            _ => Arc::new(HashMap::new()),
        }
    }

    // Merge two toggle if it posible, otherwise override
    pub fn merge(&mut self, new_toggle: Self) {
        match new_toggle {
            Toggle::Disable(rules) => {
                if let Toggle::Disable(old_rules) = self {
                    let mut old_rules = old_rules
                        .iter()
                        .map(|(k, v)| (k.clone(), *v))
                        .collect::<HashMap<String, bool>>();
                    if !old_rules.is_empty() {
                        for (k, v) in rules.iter() {
                            old_rules.insert(k.clone(), *v);
                        }
                    }

                    if rules.is_empty() {
                        old_rules.clear();
                    }
                    *self = Toggle::Disable(Arc::new(old_rules));
                } else {
                    *self = Toggle::Disable(rules);
                }
            }
            Toggle::Enable(rules) => {
                if let Toggle::Enable(old_rules) = self {
                    let mut old_rules = old_rules
                        .iter()
                        .map(|(k, v)| (k.clone(), *v))
                        .collect::<HashMap<String, bool>>();

                    if !old_rules.is_empty() {
                        for (k, v) in rules.iter() {
                            old_rules.insert(k.clone(), *v);
                        }
                    }

                    if rules.is_empty() {
                        old_rules.clear();
                    }
                    *self = Toggle::Enable(Arc::new(old_rules));
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
                            rules.push(pair.as_str());
                        }
                    }

                    return Toggle::disable(rules);
                }
                Rule::enable => {
                    let mut rules = vec![];
                    for pair in pair.into_inner() {
                        if pair.as_rule() == Rule::rule_name {
                            rules.push(pair.as_str());
                        }
                    }
                    return Toggle::enable(rules);
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
        assert_eq!(Toggle::enable(vec![]).match_rule("rule"), Some(true));
        assert_eq!(Toggle::enable(vec![]).match_rule("foo"), Some(true));
        assert_eq!(Toggle::enable(vec![]).match_rule(""), Some(true));

        assert_eq!(Toggle::enable(vec!["foo"]).match_rule("foo"), Some(true));
        assert_eq!(Toggle::enable(vec!["bar"]).match_rule("foo"), Some(false));
        assert_eq!(
            Toggle::enable(vec!["foo", "bar"]).match_rule("foo"),
            Some(true)
        );
        assert_eq!(
            Toggle::enable(vec!["foo", "bar"]).match_rule("bar"),
            Some(true)
        );
        assert_eq!(
            Toggle::enable(vec!["foo", "bar"]).match_rule("dar"),
            Some(false)
        );
        assert_eq!(
            Toggle::enable(vec!["foo", "bar"]).match_rule(""),
            Some(false)
        );
    }

    #[test]
    fn it_parse() {
        assert_eq!(Toggle::enable(vec![]), parse("autocorrect-enable"));
        assert_eq!(Toggle::enable(vec![]), parse("// autocorrect-enable"));
        assert_eq!(Toggle::enable(vec![]), parse("# autocorrect-enable"));
        assert_eq!(Toggle::enable(vec![]), parse("# autocorrect: true"));
        assert_eq!(Toggle::enable(vec![]), parse("# autocorrect:true"));
        assert_eq!(Toggle::disable(vec![]), parse("# autocorrect: false"));
        assert_eq!(Toggle::disable(vec![]), parse("# autocorrect:false"));
        assert_eq!(Toggle::disable(vec![]), parse("# autocorrect-disable"));
        assert_eq!(Toggle::disable(vec![]), parse("// autocorrect-disable"));
        assert_eq!(Toggle::none(), parse("// hello world"));
    }

    #[test]
    fn it_parse_with_rules() {
        assert_eq!(Toggle::enable(vec!["foo"]), parse("autocorrect-enable foo"));

        assert_eq!(
            Toggle::enable(vec!["foo", "bar"]),
            parse("// autocorrect-enable foo, bar")
        );
        assert_eq!(
            Toggle::enable(vec!["foo", "bar"]),
            parse("// autocorrect-enable foo,bar")
        );

        assert_eq!(
            Toggle::disable(vec!["foo"]),
            parse("# autocorrect-disable foo")
        );
        assert_eq!(
            Toggle::disable(vec!["foo", "bar"]),
            parse("// autocorrect-disable foo,bar")
        );
        assert_eq!(
            Toggle::disable(vec!["foo", "bar", "foo-bar_dar"]),
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
        let mut toggle = Toggle::enable(vec!["foo"]);
        toggle.merge(Toggle::enable(vec!["bar"]));
        assert_eq!(Toggle::enable(vec!["foo", "bar"]), toggle);
        toggle.merge(Toggle::enable(vec![]));
        assert_eq!(Toggle::enable(vec![]), toggle);
        toggle.merge(Toggle::enable(vec!["foo"]));
        assert_eq!(Toggle::enable(vec![]), toggle);

        let mut toggle = Toggle::disable(vec!["foo", "bar"]);
        toggle.merge(Toggle::disable(vec!["dar"]));
        assert_eq!(Toggle::disable(vec!["foo", "bar", "dar"]), toggle);
        toggle.merge(Toggle::disable(vec![]));
        assert_eq!(Toggle::disable(vec![]), toggle);
        toggle.merge(Toggle::disable(vec!["foo"]));
        assert_eq!(Toggle::disable(vec![]), toggle);

        // Merge with disable enum value, override
        let mut toggle = Toggle::enable(vec!["foo", "bar"]);
        toggle.merge(Toggle::disable(vec!["dar"]));
        assert_eq!(Toggle::disable(vec!["dar"]), toggle);
        toggle.merge(Toggle::none());
        assert_eq!(Toggle::none(), toggle);

        let mut toggle = Toggle::disable(vec!["foo", "bar"]);
        toggle.merge(Toggle::enable(vec!["dar"]));
        assert_eq!(Toggle::enable(vec!["dar"]), toggle);
        toggle.merge(Toggle::none());
        assert_eq!(Toggle::none(), toggle);
    }
}
