use std::borrow::Cow;

use crate::config::SeverityMode;
use crate::result::Severity;

pub(crate) struct Rule {
    #[allow(dead_code)]
    pub name: String,
    pub format_fn: for<'a> fn(input: &'a str) -> Cow<'a, str>,
}

#[derive(Default)]
pub(crate) struct RuleResult<'a> {
    pub out: Cow<'a, str>,
    pub severity: Severity,
}

impl<'a> RuleResult<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            out: Cow::Borrowed(input),
            ..Default::default()
        }
    }
}

impl Rule {
    pub fn new(name: &str, format: for<'a> fn(input: &'a str) -> Cow<'a, str>) -> Self {
        Rule {
            name: name.to_string(),
            format_fn: format,
        }
    }

    pub fn format(&self, result: &mut RuleResult) {
        if self.severity() != SeverityMode::Error {
            return;
        }

        if let Cow::Owned(new) = (self.format_fn)(&result.out) {
            result.severity = Severity::Error;
            result.out = Cow::Owned(new);
        }
    }

    pub fn lint(&self, result: &mut RuleResult) {
        if self.severity() == SeverityMode::Off {
            return;
        }

        if let Cow::Owned(new) = (self.format_fn)(&result.out) {
            if result.severity == Severity::Pass {
                if self.severity() == SeverityMode::Warning {
                    result.severity = Severity::Warning;
                } else {
                    result.severity = Severity::Error;
                }
            }
            result.out = Cow::Owned(new);
        }
    }

    fn severity(&self) -> SeverityMode {
        let config = crate::Config::current();

        if let Some(s) = config.rules.get(&self.name) {
            *s
        } else {
            SeverityMode::Off
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_not_pass() {
        let rule = Rule::new("space-word", |input| Cow::Owned(format!("{input} - foo")));
        assert_eq!(rule.severity(), SeverityMode::Error);
        assert_eq!(rule.name, "space-word");

        let mut result = RuleResult::new("test");
        rule.format(&mut result);
        assert_eq!(result.out, "test - foo");
        assert_eq!(result.severity, Severity::Error);

        let mut result = RuleResult::new("test");
        rule.lint(&mut result);
        assert_eq!(result.out, "test - foo");
        assert_eq!(result.severity, Severity::Error);
    }

    #[test]
    fn test_rule_pass() {
        let rule = Rule::new("spellcheck", |input| Cow::Borrowed(input));

        let mut result = RuleResult::new("test");
        rule.format(&mut result);
        assert_eq!(result.out, "test");
        assert_eq!(result.severity, Severity::Pass);

        rule.lint(&mut result);
        assert_eq!(result.out, "test");
        assert_eq!(result.severity, Severity::Pass);
    }
}
