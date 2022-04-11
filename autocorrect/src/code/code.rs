// autocorrect: false
use super::*;
use crate::format;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::result::Result;

pub fn format_pairs<R: RuleType, O: Results>(out: O, pairs: Result<Pairs<R>, Error<R>>) -> O {
    let mut out = out;

    match pairs {
        Ok(items) => {
            for item in items {
                format_pair(&mut out, item, "");
            }
        }
        Err(_err) => {
            out.error(format!("{}", _err).as_str());
        }
    }

    out
}

fn format_pair<R: RuleType, O: Results>(results: &mut O, item: Pair<R>, scope_rule: &str) {
    let rule = item.as_rule();
    let rule_name = format!("{:?}", rule);
    let rule_name = rule_name.as_str();

    // println!("rule: {}", rule_name);

    match rule_name {
        "string" | "link_string" | "text" | "comment" => {
            format_or_lint(results, rule_name, item);
        }
        "inline_style" | "inline_javascript" => {
            format_or_lint_for_inline_scripts(results, item, rule_name);
        }
        _ => {
            let mut has_child = false;
            let item_str = item.as_str();
            for child in item.into_inner() {
                format_pair(results, child, scope_rule);
                has_child = true;
            }

            if !has_child {
                results.ignore(item_str);
            }
        }
    };
}

pub fn format_or_lint<R: RuleType, O: Results>(results: &mut O, rule_name: &str, item: Pair<R>) {
    let part = item.as_str();
    let (line, col) = results.move_cursor(part);

    // check autocorrect toggle
    if rule_name == "comment" {
        match match_autocorrect_toggle(part) {
            Toggle::Disable => results.toggle(false),
            Toggle::Enable => results.toggle(true),
            _ => {}
        }
    }

    if results.is_lint() {
        // skip if not enable
        if !results.is_enabled() {
            return;
        }

        let lines = part.split('\n');

        // sub line in a part
        let mut sub_line = 0;
        for line_str in lines {
            // format trimmed string
            let new_line = format(line_str);

            // nothing changed, skip
            if new_line.eq(line_str) {
                sub_line += 1;
                continue;
            }

            // trim start whitespace
            let mut trimmed = line_str.trim_start();
            // number of start whitespace in this line
            let leading_spaces = line_str.len() - trimmed.len();
            // trim end whitespace
            trimmed = trimmed.trim_end();
            // println!("{}||{},{}", new_line, trimmed, new_line.eq(trimmed));

            let current_line = line + sub_line;
            let current_col = if sub_line > 0 {
                // col will equal numner of removed leading whitespace
                leading_spaces + 1
            } else {
                col
            };

            results.push(LineResult {
                line: current_line,
                col: current_col,
                old: String::from(trimmed),
                new: new_line.trim().to_string(),
            });
            sub_line += 1;
        }
    } else {
        let mut new_part = String::from(part);

        // only for on enable
        if results.is_enabled() {
            let lines = part.split('\n');

            new_part = lines
                .into_iter()
                .map(|line_str| format(line_str))
                .collect::<Vec<_>>()
                .join("\n");
        }

        results.push(LineResult {
            line,
            col,
            old: String::from(part),
            new: new_part,
        });
    }
}

// format_or_lint for inline scripts, for example, script/css in html
fn format_or_lint_for_inline_scripts<R: RuleType, O: Results>(
    results: &mut O,
    item: Pair<R>,
    rule_name: &str,
) {
    let part = item.as_str();

    results.move_cursor(part);

    if results.is_lint() {
        if rule_name == "inline_style" {
            let sub_reuslts = css::lint_css(part);
            for line in sub_reuslts.lines {
                results.push(line);
            }
            results.error(sub_reuslts.error.as_str());

            return;
        } else if rule_name == "inline_javascript" {
            let sub_reuslts = javascript::lint_javascript(part);
            for line in sub_reuslts.lines {
                results.push(line);
            }
            results.error(sub_reuslts.error.as_str());

            return;
        }
    }

    if rule_name == "inline_style" {
        let sub_reuslts = css::format_css(part);
        results.push(LineResult {
            line: 0,
            col: 0,
            old: String::from(part),
            new: sub_reuslts.out,
        });
        results.error(sub_reuslts.error.as_str());

        return;
    } else if rule_name == "inline_javascript" {
        let sub_reuslts = javascript::format_javascript(part);
        results.push(LineResult {
            line: 0,
            col: 0,
            old: String::from(part),
            new: sub_reuslts.out,
        });
        results.error(sub_reuslts.error.as_str());

        return;
    }
}

#[derive(PartialEq, Debug)]
enum Toggle {
    None,
    Disable,
    Enable,
}

lazy_static! {
    static ref DISABLE_RE: Regex = Regex::new(r"autocorrect(:[ ]*|\-)(false|disable)").unwrap();
    static ref ENABLE_RE: Regex = Regex::new(r"autocorrect(:[ ]*|\-)(true|enable)").unwrap();
}

fn match_autocorrect_toggle(part: &str) -> Toggle {
    if DISABLE_RE.is_match(part) {
        return Toggle::Disable;
    }

    if ENABLE_RE.is_match(part) {
        return Toggle::Enable;
    }

    Toggle::None
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
    // toggle autocorrect template enable or disable
    fn toggle(&mut self, enable: bool);
    fn is_enabled(&self) -> bool;
    // Move and save current line,col return the previus line number
    fn move_cursor(&mut self, part: &str) -> (usize, usize);
}

#[derive(Serialize, Deserialize)]
pub struct FormatResult {
    pub out: String,
    pub error: String,
    #[serde(skip)]
    pub raw: String,
    #[serde(skip)]
    pub enable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LintResult {
    #[serde(skip)]
    pub raw: String,
    pub filepath: String,
    pub lines: Vec<LineResult>,
    pub error: String,
    #[serde(skip)]
    pub enable: bool,
    // For store line number in loop
    #[serde(skip)]
    line: usize,
    // For store col number in loop
    #[serde(skip)]
    col: usize,
}

impl FormatResult {
    pub fn new(raw: &str) -> Self {
        FormatResult {
            raw: String::from(raw),
            out: String::from(""),
            error: String::from(""),
            enable: true,
        }
    }

    #[allow(dead_code)]
    pub fn has_error(&self) -> bool {
        !self.error.is_empty()
    }
}

impl<'a> Results for FormatResult {
    fn push(&mut self, line_result: LineResult) {
        self.out.push_str(line_result.new.as_str());
    }

    fn ignore(&mut self, part: &str) {
        self.out.push_str(part);
        self.move_cursor(part);
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

    fn toggle(&mut self, enable: bool) {
        self.enable = enable;
    }

    fn is_enabled(&self) -> bool {
        self.enable
    }

    fn move_cursor(&mut self, _part: &str) -> (usize, usize) {
        (0, 0)
    }
}

impl LintResult {
    pub fn new(raw: &str) -> Self {
        LintResult {
            line: 1,
            col: 1,
            filepath: String::from(""),
            raw: String::from(raw),
            lines: Vec::new(),
            error: String::from(""),
            enable: true,
        }
    }

    #[allow(dead_code)]
    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            _ => String::from("{}"),
        }
    }

    #[allow(dead_code)]
    pub fn to_json_pretty(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Ok(json) => json,
            _ => String::from("{}"),
        }
    }

    #[allow(dead_code)]
    pub fn to_diff(&self) -> String {
        let mut out = String::from("");

        for line in self.lines.iter() {
            out.push_str(
                format!(
                    "{}:{}:{}\n",
                    self.filepath.replace("./", ""),
                    line.line,
                    line.col
                )
                .as_str(),
            );

            let changeset = difference::Changeset::new(line.old.as_str(), line.new.as_str(), "\n");
            out.push_str(format!("{}\n", changeset).as_str());
        }

        out
    }

    #[allow(dead_code)]
    pub fn has_error(&self) -> bool {
        !self.error.is_empty()
    }
}

impl Results for LintResult {
    fn push(&mut self, line_result: LineResult) {
        self.lines.push(line_result);
    }

    fn ignore(&mut self, part: &str) {
        // do nothing
        self.move_cursor(part);
    }

    fn error(&mut self, err: &str) {
        self.error = String::from(err);
    }

    fn to_string(&self) -> String {
        String::from("")
    }

    fn is_lint(&self) -> bool {
        true
    }

    fn toggle(&mut self, enable: bool) {
        self.enable = enable;
    }

    fn is_enabled(&self) -> bool {
        self.enable
    }

    /// Move the (line, col) with string part
    fn move_cursor(&mut self, part: &str) -> (usize, usize) {
        let (l, c, has_new_line) = line_col(part);

        let prev_line = self.line;
        let prev_col = self.col;

        self.line += l;
        if has_new_line {
            self.col = c;
        } else {
            self.col += c;
        }
        (prev_line, prev_col)
    }
}

/// Calculate line and col number of a string part
/// Fork from Pest for just count the part.
///
/// https://github.com/pest-parser/pest/blob/85b18aae23cc7b266c0b5252f9f74b7ab0000795/pest/src/position.rs#L135
fn line_col(part: &str) -> (usize, usize, bool) {
    let mut chars = part.chars().peekable();

    let mut line_col = (0, 0);
    let mut has_new_line = false;

    loop {
        match chars.next() {
            Some('\r') => {
                if let Some(&'\n') = chars.peek() {
                    chars.next();

                    line_col = (line_col.0 + 1, 1);
                    has_new_line = true;
                } else {
                    line_col = (line_col.0, line_col.1 + 1);
                }
            }
            Some('\n') => {
                line_col = (line_col.0 + 1, 1);
                has_new_line = true;
            }
            Some(_c) => {
                line_col = (line_col.0, line_col.1 + 1);
            }
            None => {
                break;
            }
        }
    }

    (line_col.0, line_col.1, has_new_line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_match_autocorrect_toggle() {
        assert_eq!(
            Toggle::Enable,
            match_autocorrect_toggle("autocorrect-enable")
        );
        assert_eq!(
            Toggle::Enable,
            match_autocorrect_toggle("// autocorrect-enable")
        );
        assert_eq!(
            Toggle::Enable,
            match_autocorrect_toggle("# autocorrect-enable")
        );
        assert_eq!(
            Toggle::Enable,
            match_autocorrect_toggle("# autocorrect: true")
        );
        assert_eq!(
            Toggle::Enable,
            match_autocorrect_toggle("# autocorrect:true")
        );
        assert_eq!(
            Toggle::Disable,
            match_autocorrect_toggle("# autocorrect: false")
        );
        assert_eq!(
            Toggle::Disable,
            match_autocorrect_toggle("# autocorrect:false")
        );
        assert_eq!(
            Toggle::Disable,
            match_autocorrect_toggle("# autocorrect-disable")
        );
        assert_eq!(
            Toggle::Disable,
            match_autocorrect_toggle("// autocorrect-disable")
        );
        assert_eq!(Toggle::None, match_autocorrect_toggle("// hello world"));
    }

    #[test]
    fn test_move_cursor() {
        let mut out = LintResult::new("");
        assert_eq!((out.line, out.col), (1, 1));

        assert_eq!(out.move_cursor(""), (1, 1));
        assert_eq!((out.line, out.col), (1, 1));

        let raw = r#"Foo
Hello world
This is "#;
        assert_eq!(out.move_cursor(raw), (1, 1));
        assert_eq!((out.line, out.col), (3, 9));

        let raw = "Hello\nworld\r\nHello world\nHello world";
        assert_eq!(out.move_cursor(raw), (3, 9));
        assert_eq!((out.line, out.col), (6, 12));

        let raw = "Hello";
        assert_eq!(out.move_cursor(raw), (6, 12));
        assert_eq!((out.line, out.col), (6, 17));

        let raw = "\nHello\n\naaa\n";
        assert_eq!(out.move_cursor(raw), (6, 17));
        assert_eq!((out.line, out.col), (10, 1));
    }
}
