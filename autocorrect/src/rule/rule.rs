use crate::config::SeverityMode;
use crate::result::Severity;

pub(crate) struct Rule {
    #[allow(dead_code)]
    pub name: String,
    pub format_fn: fn(input: &str) -> String,
}

pub(crate) struct RuleResult {
    pub out: String,
    pub severity: Severity,
}

impl RuleResult {
    pub fn new(input: &str) -> Self {
        Self {
            out: input.to_string(),
            severity: Severity::Pass,
        }
    }
}

impl Rule {
    pub fn new(name: &str, format: fn(input: &str) -> String) -> Self {
        Rule {
            name: name.to_string(),
            format_fn: format,
        }
    }

    pub fn format(&self, result: &mut RuleResult) {
        if self.severity() != SeverityMode::Error {
            return;
        }
        let input = result.out.clone();

        result.out = (self.format_fn)(&input);
        if result.out.ne(&input) {
            result.severity = Severity::Error;
        }
    }

    pub fn lint(&self, result: &mut RuleResult) {
        if self.severity() == SeverityMode::Off {
            return;
        }
        let input = result.out.clone();
        result.out = (self.format_fn)(&input);
        if result.out.ne(&input) && result.severity == Severity::Pass {
            if self.severity() == SeverityMode::Warning {
                result.severity = Severity::Warning;
            } else {
                result.severity = Severity::Error;
            }
        }
    }

    fn severity(&self) -> SeverityMode {
        let config = crate::Config::current();

        if let Some(s) = config.rules.get(&self.name) {
            s.clone()
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
        let rule = Rule::new("space-word", |input| format!("{} - foo", input.to_string()));
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
        let rule = Rule::new("spellcheck", |input| input.to_string());

        let mut result = RuleResult::new("test");
        rule.format(&mut result);
        assert_eq!(result.out, "test");
        assert_eq!(result.severity, Severity::Pass);

        rule.lint(&mut result);
        assert_eq!(result.out, "test");
        assert_eq!(result.severity, Severity::Pass);
    }
}
