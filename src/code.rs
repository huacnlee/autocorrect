// autocorrect: false
use super::*;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use std::result::Result;

use serde_json::json;

pub fn format_pairs<R: RuleType>(
    text: &str,
    pairs: Result<Pairs<R>, Error<R>>,
    lint: bool,
) -> String {
    match pairs {
        Ok(items) => {
            let mut out = String::new();
            for item in items {
                format_pair(&mut out, item, lint);
            }
            return out;
        }
        Err(_err) => {
            // return raw or empty(lint) when err
            println!("error: {}", _err);
            if lint {
                return String::from("");
            } else {
                return String::from(text);
            }
        }
    }
}

fn format_pair<R: RuleType>(results: &mut StringOrLintResult, item: Pair<R>, lint: bool) {
    let rule = item.as_rule();
    let rule_name = format!("{:?}", rule);

    // println!("rule: {}", rule_name);

    match rule_name.as_str() {
        "string" | "link_string" | "text" | "comment" => format_or_lint(results, item, lint),
        _ => {
            let mut child_count = 0;
            let item_str = item.as_str();
            for child in item.into_inner() {
                format_pair(results, child, lint);
                child_count += 1;
            }

            if child_count == 0 {
                results.push_ignore_str(item_str);
            }
        }
    };
}

fn format_or_lint<R: RuleType>(results: &mut StringOrLintResult, item: Pair<R>, lint: bool) {
    let (part_line, part_col) = item.as_span().start_pos().line_col();
    let part = item.as_str();

    match results {
        StringOrLintResult::String(str) => {}
    }

    if lint {
        let lines = part.split("\n");

        // sub line in a part
        let mut sub_line = 0;
        for line in lines {
            // trim start whitespace
            let mut trimmed = line.trim_start();
            // number of start whitespace in this line
            let leading_spaces = line.len() - trimmed.len();
            // trim end whitespace
            trimmed = trimmed.trim_end();

            // format trimmed string
            let new_line = format(trimmed);

            // println!("{}||{},{}", new_line, trimmed, new_line.eq(trimmed));

            if new_line.eq(trimmed) {
                sub_line += 1;
                continue;
            }

            let current_line = part_line + sub_line;
            let current_col = if sub_line > 0 {
                // col will equal numner of removed leading whitespace
                leading_spaces + 1
            } else {
                part_col
            };

            results.push_result(LineResult {
                line: current_line,
                col: current_col,
                old: trimmed,
                new: new_line,
            });
            sub_line += 1;
        }
    } else {
        results.push_result(LineResult {
            line: part_line,
            col: part_col,
            old: String::from(part),
            new: format(part),
        });
    }
}

pub struct LineResult {
    line: usize,
    col: usize,
    new: String,
    old: String,
}

pub trait Results {
    fn push_result(&mut self, lineResult: LineResult);
    fn push_ignore_str(&mut self, str: &str);
    fn push_error(&self, err: &str);
}

pub struct FormatResult<'a> {
    out: &'a mut String,
    error: String,
}

pub struct LintResult<'a> {
    lines: &'a mut Vec<LineResult>,
    error: String,
}

impl<'a> Results for FormatResult<'a> {
    fn push_result(&mut self, lineResult: LineResult) {
        self.out.push_str(lineResult.new.as_str());
    }

    fn push_ignore_str(&mut self, str: &str) {
        self.out.push_str(str)
    }

    fn push_error(&self, err: &str) {
        self.error = String::from(err);
    }
}

impl<'a> Results for LintResult<'a> {
    fn push_result(&mut self, lineResult: LineResult) {
        self.lines.push(lineResult);
    }

    fn push_ignore_str(&mut self, str: &str) {
        // do nothing
    }

    fn push_error(&self, err: &str) {
        self.error = String::from(err);
    }
}
