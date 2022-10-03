use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./config/toggle.pest"]
pub struct ToggleParser;

#[derive(PartialEq, Eq, Debug)]
pub enum Toggle {
    None,
    Disable(Vec<String>),
    Enable(Vec<String>),
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
}
