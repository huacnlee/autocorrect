use pest::Parser;
use pest_derive::Parser;

use super::RuleResult;

#[derive(Parser)]
#[grammar = "./rule/text.pest"]
struct TextParser;

pub(crate) fn format(input: &str) -> RuleResult {
    let mut out = String::new();
    let mut pairs = TextParser::parse(Rule::expr, input)
        .unwrap()
        .flatten()
        .peekable();

    while let Some(pair) = pairs.next() {
        let next = pairs.peek();
        match pair.as_rule() {
            Rule::word => {
                out.push_str(pair.as_str());
                if let Some(next) = next {
                    match next.as_rule() {
                        Rule::cjk => {
                            out.push(' ');
                        }
                        _ => {}
                    }
                }
            }
            Rule::cjk => {
                out.push_str(pair.as_str());
                if let Some(next) = next {
                    match next.as_rule() {
                        Rule::word => {
                            out.push(' ');
                        }
                        _ => {}
                    }
                }
            }
            _ => out.push_str(pair.as_str()),
        }
    }

    RuleResult {
        out,
        severity: super::Severity::Pass,
    }
}
