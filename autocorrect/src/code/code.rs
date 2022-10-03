// autocorrect: false
use super::*;
pub use crate::result::*;
use crate::spellcheck::spellcheck;
use crate::{config, format, Config};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use regex::Regex;
use std::result::Result;

pub fn format_pairs<R: RuleType, O: Results>(out: O, pairs: Result<Pairs<R>, Error<R>>) -> O {
    // Limit parse stack max depth for avoiding some complex parser will hangs indefinitely.
    pest::set_call_limit(Some(10_000_000usize.try_into().unwrap()));

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

fn get_rule_name<R: RuleType>(item: &Pair<R>) -> String {
    let rule = item.as_rule();
    format!("{:?}", rule)
}

fn format_pair<R: RuleType, O: Results>(results: &mut O, item: Pair<R>, scope_rule: &str) {
    let rule_name = get_rule_name(&item);

    // println!("rule: {}, {}", rule_name, item.as_str());

    match rule_name.as_str() {
        "string" | "link_string" | "mark_string" | "text" | "comment" => {
            format_or_lint(results, &rule_name, item);
        }
        "inline_style" | "inline_javascript" | "codeblock" => {
            format_or_lint_for_inline_scripts(results, item, &rule_name);
        }
        _ => {
            let mut has_child = false;
            let item_str = item.as_str();
            let sub_items = item.into_inner();

            for child in sub_items {
                format_pair(results, child, scope_rule);
                has_child = true;
            }

            if !has_child {
                results.ignore(item_str);
            }
        }
    };
}

/// Format or Lint a matched item
pub fn format_or_lint<R: RuleType, O: Results>(results: &mut O, rule_name: &str, item: Pair<R>) {
    let part = item.as_str();
    let (line, col) = results.move_cursor(part);

    // Check AutoCorrect enable/disable toggle marker
    // If disable results.is_enabled() will be false
    if rule_name == "comment" {
        match match_autocorrect_toggle(part) {
            Toggle::Disable => results.toggle(false),
            Toggle::Enable => results.toggle(true),
            _ => {}
        }
    }

    if results.is_lint() {
        // Skip lint if AutoCorrect disabled
        if !results.is_enabled() {
            return;
        }

        let lines = part.split('\n');

        // sub line in a part
        let mut sub_line = 0;
        for line_str in lines {
            // format trimmed string
            let new_line = format(line_str);
            let spell_new_line = spellcheck(&new_line);

            // skip, when no difference
            if new_line.eq(line_str) && spell_new_line.eq(&new_line) {
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

            // Add error lint result, if new_line has get changed result
            if new_line.ne(line_str) {
                results.push(LineResult {
                    line: current_line,
                    col: current_col,
                    old: String::from(trimmed),
                    new: new_line.trim().to_string(),
                    severity: Severity::Error,
                });
            }

            // If has spelling issues, add more lint result
            if spell_new_line.ne(&new_line) {
                results.push(LineResult {
                    line: current_line,
                    col: current_col,
                    old: String::from(trimmed),
                    new: spell_new_line.trim().to_string(),
                    severity: Severity::Warning,
                });
            }

            sub_line += 1;
        }
    } else {
        let mut new_part = String::from(part);

        // Skip format if AutoCorrect disabled
        if results.is_enabled() {
            let lines = part.split('\n');

            new_part = lines
                .into_iter()
                .map(format)
                .map(|l| {
                    if Config::current().spellcheck.mode == Some(config::SpellcheckMode::Enabled) {
                        spellcheck(&l)
                    } else {
                        l
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
        }

        results.push(LineResult {
            line,
            col,
            old: String::from(part),
            new: new_part,
            severity: Severity::Pass,
        });
    }
}

/// Format / Lint for the inline scripts.
///
/// For example, The script / style in HTML or Codeblock in Markdown.
fn format_or_lint_for_inline_scripts<R: RuleType, O: Results>(
    results: &mut O,
    item: Pair<R>,
    rule_name: &str,
) {
    let part = item.as_str();

    let (base_line, _) = results.move_cursor(part);

    if results.is_lint() {
        // Skip lint if AutoCorrect disabled
        if !results.is_enabled() {
            return;
        }

        let sub_result = match rule_name {
            "inline_style" => Some(lint_for(part, "css")),
            "inline_javascript" => Some(lint_for(part, "js")),
            "codeblock" => {
                let codeblock = Codeblock::from_pair(item);
                Some(lint_for(&codeblock.code, &codeblock.lang))
            }
            _ => None,
        };

        if let Some(result) = sub_result {
            if result.has_error() {
                results.error(&result.error);
            }

            for mut line in result.lines {
                // Inline script's lines need add base_line - 1 offset.
                line.line += base_line - 1;
                results.push(line);
            }
        }
    } else {
        let mut new_part = String::from(part);

        // Skip format if AutoCorrect disabled
        if results.is_enabled() {
            let sub_result = match rule_name {
                "inline_style" => Some(format_for(part, "css")),
                "inline_javascript" => Some(format_for(part, "js")),
                "codeblock" => {
                    // WARNING: nested codeblock, when call format_for again.
                    // Because codeblock.data has wrap chars, this make overflowed its stack.
                    let mut codeblock = Codeblock::from_pair(item);
                    let mut result = format_for(&codeblock.code, &codeblock.lang);
                    codeblock.update_data(&result.out);
                    result.out = codeblock.data;
                    Some(result)
                }
                _ => None,
            };

            if let Some(result) = sub_result {
                if result.has_error() {
                    results.error(&result.error);
                }

                new_part = result.out;
            }
        }

        results.push(LineResult {
            line: 0,
            col: 0,
            old: String::from(part),
            new: new_part,
            severity: Severity::Pass,
        });
    }
}

struct Codeblock {
    pub lang: String,
    // All string of codeblock
    pub data: String,
    // Code string of codeblock
    pub code: String,
}

impl Codeblock {
    // Update codeblock data replace code as new code.
    pub fn update_data(&mut self, new_code: &str) {
        self.data = self.data.replace(&self.code, new_code);
        self.code = new_code.to_string();
    }

    pub fn from_pair<R: RuleType>(item: Pair<R>) -> Codeblock {
        let mut codeblock = Codeblock {
            lang: String::new(),
            data: String::new(),
            code: String::new(),
        };

        codeblock.data = item.as_str().to_string();

        for child in item.into_inner() {
            match get_rule_name(&child).as_str() {
                "codeblock_lang" => {
                    codeblock.lang = child.as_str().to_string();
                }
                "codeblock_code" => {
                    codeblock.code = child.as_str().to_string();
                }
                _ => {}
            }
        }

        codeblock
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
    fn test_format_for() {
        let mut raw = "// Hello你好";
        let mut result = format_for(raw, "rust");
        assert_eq!(result.out, "// Hello 你好");

        result = format_for(raw, "js");
        assert_eq!(result.out, "// Hello 你好");

        result = format_for(raw, "ruby");
        assert_eq!(result.out, "// Hello你好");

        raw = "// Hello你好";
        result = format_for(raw, "not-exist-type");
        assert_eq!(result.out, raw);
    }

    #[test]
    fn test_lint_for() {
        let mut raw = "// Hello你好";
        let mut result = lint_for(raw, "rust");
        assert_eq!(result.lines.len(), 1);

        result = lint_for(raw, "js");
        assert_eq!(result.lines.len(), 1);

        result = lint_for(raw, "ruby");
        assert_eq!(result.lines.len(), 0);

        raw = "// Hello你好";
        result = lint_for(raw, "not-exist-type");
        assert_eq!(result.lines.len(), 0);
    }

    #[test]
    fn test_codeblock() {
        let mut codeblock = Codeblock {
            data: "```rb\nhello\n```".to_string(),
            code: "\nhello\n".to_string(),
            lang: "rb".to_string(),
        };

        codeblock.update_data("\nhello world\n");
        assert_eq!(codeblock.data, "```rb\nhello world\n```".to_string());
        assert_eq!(codeblock.code, "\nhello world\n".to_string());
    }

    #[test]
    fn test_inline_script_line_number() {
        let raw = r#""Hello world
        ```js
        // hello世界
        ```

        ### 外部test

        Second line

        ```rb
        class User
          # 查找user
          def find
          end
        end
        ```
        “"#;

        let result = lint_for(raw, "markdown");
        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.lines[0].line, 3);
        assert_eq!(result.lines[0].col, 9);
        assert_eq!(result.lines[0].new, "// hello 世界");
        assert_eq!(result.lines[1].line, 6);
        assert_eq!(result.lines[1].col, 13);
        assert_eq!(result.lines[1].new, "外部 test");
        assert_eq!(result.lines[2].line, 12);
        assert_eq!(result.lines[2].col, 11);
        assert_eq!(result.lines[2].new, "# 查找 user");
    }
}
