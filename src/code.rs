// autocorrect: false
use super::*;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use serde::{Deserialize, Serialize};
use std::result::Result;

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

#[derive(Serialize, Deserialize)]
pub struct LineResult {
    #[serde(rename(serialize = "l"))]
    pub line: usize,
    #[serde(rename(serialize = "c"))]
    pub col: usize,
    pub new: String,
    pub old: String,
}

pub trait Results {
    fn push(&mut self, line_result: LineResult);
    fn ignore(&mut self, str: &str);
    fn error(&mut self, err: &str);
    fn to_string(&self) -> String;
    fn is_lint(&self) -> bool;
}

pub struct FormatResult {
    pub out: String,
    pub error: String,
    pub raw: String,
}

#[derive(Serialize, Deserialize)]
pub struct LintResult {
    #[serde(skip)]
    pub raw: String,
    pub filepath: String,
    pub lines: Vec<LineResult>,
    pub error: String,
}

impl<'a> FormatResult {
    pub fn new(raw: &str) -> Self {
        FormatResult {
            raw: String::from(raw),
            out: String::from(""),
            error: String::from(""),
        }
    }
}

impl<'a> Results for FormatResult {
    fn push(&mut self, line_result: LineResult) {
        self.out.push_str(line_result.new.as_str());
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
    pub fn new(raw: &str) -> Self {
        LintResult {
            filepath: String::from(""),
            raw: String::from(raw),
            lines: Vec::new(),
            error: String::from(""),
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            _ => String::from("{}"),
        }
    }

    pub fn to_json_pretty(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Ok(json) => json,
            _ => String::from("{}"),
        }
    }

    pub fn to_diff(&self) -> String {
        let mut out = String::from("");

        if self.lines.len() == 0 {
            return out;
        }

        out.push_str(
            format!(
                "AutoCorrect has found {} issues need to fix.\n\n",
                self.lines.len()
            )
            .as_str(),
        );

        for line in self.lines.iter() {
            let line_info = format!("--> {}:{}:{}", self.filepath, line.line, line.col);
            out.push_str(line_info.as_str());
            out.push_str("\n");

            let changeset = difference::Changeset::new(line.old.as_str(), line.new.as_str(), "\n");
            out.push_str(format!("{}\n\n", changeset).as_str());
        }

        return out;
    }
}

impl Results for LintResult {
    fn push(&mut self, line_result: LineResult) {
        self.lines.push(line_result);
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
