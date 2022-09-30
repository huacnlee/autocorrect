use crate::config::SeverityMode;

pub(crate) struct Rule {
    #[allow(dead_code)]
    pub name: String,
    pub format_fn: fn(input: &str) -> String,
}

impl Rule {
    pub fn new(name: &str, format: fn(input: &str) -> String) -> Self {
        Rule {
            name: name.to_string(),
            format_fn: format,
        }
    }

    pub fn format(&self, input: &str) -> String {
        if self.severity() != SeverityMode::Error {
            return String::from(input);
        }

        (self.format_fn)(input)
    }

    pub fn lint(&self, input: &str) -> String {
        if self.severity() == SeverityMode::Off {
            return String::from(input);
        }

        (self.format_fn)(input)
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
    fn test_rule() {
        let rule = Rule::new("test", |input| input.to_string());
        assert_eq!(rule.name, "test");
        assert_eq!(rule.format("test"), "test");
    }
}
