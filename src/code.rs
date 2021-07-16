// autocorrect: false
use super::*;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use std::fmt;
use std::result::Result;

use serde_json::json;

pub fn format_pairs<R: RuleType, O: Results>(out: O, pairs: Result<Pairs<R>, Error<R>>) -> O {
    let mut out = out;

    match pairs {
        Ok(items) => {
            for item in items {
                format_pair(&mut out, item);
            }
        }
        Err(_err) => {
            out.error(format!("{}", _err).as_str());
        }
    }

    return out;
}

fn format_pair<R: RuleType, O: Results>(results: &mut O, item: Pair<R>) {
    let rule = item.as_rule();
    let rule_name = format!("{:?}", rule);

    // println!("rule: {}", rule_name);

    match rule_name.as_str() {
        "string" | "link_string" | "text" | "comment" => format_or_lint(results, item),
        _ => {
            let mut child_count = 0;
            let item_str = item.as_str();
            for child in item.into_inner() {
                format_pair(results, child);
                child_count += 1;
            }

            if child_count == 0 {
                results.ignore(item_str);
            }
        }
    };
}

fn format_or_lint<R: RuleType, O: Results>(results: &mut O, item: Pair<R>) {
    let (part_line, part_col) = item.as_span().start_pos().line_col();
    let part = item.as_str();

    if results.is_lint() {
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

            results.push(LineResult {
                line: current_line,
                col: current_col,
                old: String::from(trimmed),
                new: new_line,
            });
            sub_line += 1;
        }
    } else {
        results.push(LineResult {
            line: part_line,
            col: part_col,
            old: String::from(part),
            new: format(part),
        });
    }
}

pub struct LineResult {
    pub line: usize,
    pub col: usize,
    pub new: String,
    pub old: String,
}

pub trait Results {
    fn push(&mut self, lineResult: LineResult);
    fn ignore(&mut self, str: &str);
    fn error(&mut self, err: &str);
    fn to_string(&self) -> String;
    fn is_lint(&self) -> bool;
}

pub struct FormatResult {
    out: String,
    error: String,
    filename: String,
    raw: String,
}

pub struct LintResult {
    filename: String,
    raw: String,
    lines: Vec<LineResult>,
    error: String,
}

impl<'a> FormatResult {
    pub fn new(filename: &str, raw: &str) -> Self {
        FormatResult {
            filename: filename.to_string(),
            raw: String::from(raw),
            out: String::from(""),
            error: String::from(""),
        }
    }
}

impl<'a> Results for FormatResult {
    fn push(&mut self, lineResult: LineResult) {
        self.out.push_str(lineResult.new.as_str());
    }

    fn ignore(&mut self, str: &str) {
        self.out.push_str(str)
    }

    fn error(&mut self, err: &str) {
        self.error = String::from(err);
    }

    fn to_string(&self) -> String {
        self.out.to_string()
    }

    fn is_lint(&self) -> bool {
        false
    }
}

impl LintResult {
    pub fn new(filename: &str, raw: &str) -> Self {
        LintResult {
            filename: filename.to_string(),
            raw: String::from(raw),
            lines: Vec::new(),
            error: String::from(""),
        }
    }
}

impl Results for LintResult {
    fn push(&mut self, lineResult: LineResult) {
        self.lines.push(lineResult);
    }

    fn ignore(&mut self, str: &str) {
        // do nothing
    }

    fn error(&mut self, err: &str) {
        self.error = String::from(err);
    }

    fn to_string(&self) -> String {
        return String::from("");
    }

    fn is_lint(&self) -> bool {
        true
    }
}
