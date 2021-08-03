// autocorrect: false
use super::*;
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

    return out;
}

fn format_pair<R: RuleType, O: Results>(results: &mut O, item: Pair<R>, scope_rule: &str) {
    let rule = item.as_rule();
    let rule_name = format!("{:?}", rule);

    // println!("rule: {}", rule_name);

    match rule_name.as_str() {
        "string" | "link_string" | "text" | "comment" => {
            format_or_lint(results, rule_name.as_str(), item)
        }
        "inline_style" | "inline_javascript" => {
            format_or_lint_for_inline_scripts(results, item, rule_name.as_str())
        }
        _ => {
            let mut child_count = 0;
            let item_str = item.as_str();
            for child in item.into_inner() {
                format_pair(results, child, scope_rule);
                child_count += 1;
            }

            if child_count == 0 {
                results.ignore(item_str);
            }
        }
    };
}

pub fn format_or_lint<R: RuleType, O: Results>(results: &mut O, rule_name: &str, item: Pair<R>) {
    let (part_line, part_col) = item.as_span().start_pos().line_col();
    let part = item.as_str();

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
        let mut new_part = String::from(part);
        // only for on enable
        if results.is_enabled() {
            new_part = format(part);
        }

        results.push(LineResult {
            line: part_line,
            col: part_col,
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
    } else {
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

    return Toggle::None;
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
}

pub struct FormatResult {
    pub out: String,
    pub error: String,
    pub raw: String,
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
}

impl<'a> FormatResult {
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
        return self.error.len() > 0;
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

    fn toggle(&mut self, enable: bool) {
        self.enable = enable;
    }

    fn is_enabled(&self) -> bool {
        return self.enable;
    }
}

impl LintResult {
    pub fn new(raw: &str) -> Self {
        LintResult {
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
            out.push_str(format!("-> {}:{}:{}\n", self.filepath, line.line, line.col).as_str());

            let changeset = difference::Changeset::new(line.old.as_str(), line.new.as_str(), "\n");
            out.push_str(format!("{}\n", changeset).as_str());
        }

        return out;
    }

    #[allow(dead_code)]
    pub fn has_error(&self) -> bool {
        return self.error.len() > 0;
    }
}

impl Results for LintResult {
    fn push(&mut self, line_result: LineResult) {
        self.lines.push(line_result);
    }

    fn ignore(&mut self, _: &str) {
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

    fn toggle(&mut self, enable: bool) {
        self.enable = enable;
    }

    fn is_enabled(&self) -> bool {
        return self.enable;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
