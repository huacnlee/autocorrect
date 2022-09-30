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
        (self.format_fn)(input)
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
