// autocorrect: false
use super::*;
use crate::config::toggle;
pub use crate::result::*;
use crate::rule::CJK_RE;
use crate::Config;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use std::result::Result;

trait RuleTypeToString {
    fn to_string(&self) -> String;
}

impl<R: RuleType> RuleTypeToString for R {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}

pub fn format_pairs<R: RuleType, O: Results>(out: O, pairs: Result<Pairs<R>, Error<R>>) -> O {
    // Limit parse stack max depth for avoiding some complex parser will hangs indefinitely.
    pest::set_call_limit(Some(10_000_000usize.try_into().unwrap()));

    let mut out = out;

    match pairs {
        Ok(pairs) => {
            for pair in pairs {
                format_pair(&mut out, pair);
            }
        }
        Err(_err) => {
            out.error(&_err.to_string());
        }
    }

    out
}

fn format_pair<R: RuleType, O: Results>(results: &mut O, pair: Pair<R>) {
    let rule = pair.as_rule();
    let rule_name = rule.to_string();
    let rule_name = rule_name.as_str();

    // println!("rule: {}, {}", rule_name, item.as_str());
    match rule_name {
        "string" | "link_string" | "mark_string" | "text" | "inner_text" | "comment"
        | "COMMENT" => {
            format_or_lint(results, rule_name, pair);
        }
        "inline_style" | "inline_javascript" | "codeblock" => {
            format_or_lint_for_inline_scripts(results, pair, rule_name);
        }
        _ => {
            let mut has_child = false;
            let pair_str = pair.as_str();
            let sub_pairs = pair.into_inner();

            // Special hotfix for Markdown block / paragraph / blockquote
            // If they has CJK chars, disable `halfwidth-punctuation` rule temporary.
            let mut last_toggle = None;
            if rule_name == "block" && CJK_RE.is_match(pair_str) {
                last_toggle = Some(results.get_toggle().clone());
                results.toggle_merge_for_codeblock();
            }

            for child in sub_pairs {
                format_pair(results, child);
                has_child = true;
            }

            // Restore toggle if last_toggle is some
            if let Some(t) = &last_toggle {
                results.toggle(t);
            }

            if !has_child {
                results.ignore(pair_str);
            }
        }
    };
}

/// Format or Lint a matched item
pub fn format_or_lint<R: RuleType, O: Results>(results: &mut O, rule_name: &str, pair: Pair<R>) {
    let part = pair.as_str();
    let (line, col) = pair.line_col();

    // Check AutoCorrect enable/disable toggle marker
    // If disable results.is_enabled() will be false
    if rule_name == "comment" || rule_name == "COMMENT" {
        results.toggle(&toggle::parse(part));
    }

    let disabled_rules = results.get_toggle().disable_rules();
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
            let line_result =
                crate::rule::format_or_lint_with_disable_rules(line_str, true, &disabled_rules);

            // skip, when no difference
            if line_result.severity.is_pass() {
                sub_line += 1;
                continue;
            }

            // trim start whitespace
            let mut trimmed = line_str.trim_start();
            // number of start whitespace in this line
            let leading_spaces = line_str.len() - trimmed.len();
            // trim end whitespace
            trimmed = trimmed.trim_end();
            // println!("{}||{},{}", line_result.out, trimmed, new_line.eq(trimmed));

            let current_line = line + sub_line;
            let current_col = if sub_line > 0 {
                // col will equal numner of removed leading whitespace
                leading_spaces + 1
            } else {
                col
            };

            // Add error lint result, if new_line has get changed result
            results.push(LineResult {
                line: current_line,
                col: current_col,
                old: String::from(trimmed),
                new: line_result.out.trim().to_string(),
                severity: line_result.severity,
            });

            sub_line += 1;
        }
    } else {
        let mut new_part = String::from(part);

        // Skip format if AutoCorrect disabled
        if results.is_enabled() {
            let lines = part.split('\n');

            new_part = lines
                .into_iter()
                .map(|l| {
                    crate::rule::format_or_lint_with_disable_rules(l, false, &disabled_rules).out
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
    pair: Pair<R>,
    rule_name: &str,
) {
    let part = pair.as_str();
    let (base_line, _) = pair.line_col();

    let is_enable_context =
        rule_name != "codeblock" || Config::current().is_enabled_context("codeblock");

    if results.is_lint() {
        // Skip lint if AutoCorrect disabled
        if !results.is_enabled() {
            return;
        }

        if !is_enable_context {
            return;
        }

        let sub_result = match rule_name {
            "inline_style" => Some(lint_for(part, "css")),
            "inline_javascript" => Some(lint_for(part, "js")),
            "codeblock" => {
                let codeblock = Codeblock::from_pair(pair);
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
        if results.is_enabled() && is_enable_context {
            let sub_result = match rule_name {
                "inline_style" => Some(format_for(part, "css")),
                "inline_javascript" => Some(format_for(part, "js")),
                "codeblock" => {
                    // WARNING: nested codeblock, when call format_for again.
                    // Because codeblock.data has wrap chars, this make overflowed its stack.
                    let mut codeblock = Codeblock::from_pair(pair);

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
            line: 1,
            col: 1,
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
            match child.as_rule().to_string().as_str() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

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
        let raw = indoc! { r###"
        Hello world

        ```ts
        // hello世界
        const a = "string字符串";
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
        "###};

        let expected = indoc! { r###"
        {
          "filepath": "md",
          "lines": [
              {
              "l": 4,
              "c": 1,
              "new": "// hello 世界",
              "old": "// hello世界",
              "severity": 1
              },
              {
              "l": 5,
              "c": 11,
              "new": "\"string 字符串\"",
              "old": "\"string字符串\"",
              "severity": 1
              },
              {
              "l": 8,
              "c": 5,
              "new": "外部 test",
              "old": "外部test",
              "severity": 1
              },
              {
              "l": 14,
              "c": 5,
              "new": "# 查找 user",
              "old": "# 查找user",
              "severity": 1
              }
          ],
          "error": ""
        }
        "###};

        let result = lint_for(raw, "md");
        assert_json_eq!(expected, result.to_json_pretty());
    }

    #[test]
    fn test_disable_rules_all() {
        let raw = r#"// autocorrect-disable
        // hello世界
        // autocorrect-enable
        // hello世界
        // autocorrect-disable space-word
        // hello世界.
        // autocorrect-disable fullwidth
        // hello世界.
        // autocorrect-disable space-word,fullwidth
        // hello世界.
        const a = "hello世界."
        “"#;

        let expected = r#"// autocorrect-disable
        // hello世界
        // autocorrect-enable
        // hello 世界
        // autocorrect-disable space-word
        // hello世界。
        // autocorrect-disable fullwidth
        // hello 世界.
        // autocorrect-disable space-word,fullwidth
        // hello世界.
        const a = "hello世界."
        “"#;

        assert_eq!(expected, format_for(raw, "js").out);
        let result = lint_for(raw, "js");
        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.lines[0].new, "// hello 世界");
        assert_eq!(result.lines[1].new, "// hello世界。");
        assert_eq!(result.lines[2].new, "// hello 世界.");
    }
}
