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
      return String::from(text);
    }
  }
}

fn format_pair<R: RuleType>(text: &mut String, item: Pair<R>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();
  let rule = item.as_rule();
  let rule_name = format!("{:?}", rule);

  // println!("rule: {}", rule_name);

  match rule_name.as_str() {
    "item" => {
      for sub in item.into_inner() {
        format_pair(text, sub, lint);
      }
    }
    "string" | "comment" => format_or_lint(text, part, true, lint, line, col),
    _ => format_or_lint(text, part, false, lint, line, col),
  };
}

fn format_or_lint(
  text: &mut String,
  part: &str,
  correct: bool,
  lint: bool,
  line: usize,
  col: usize,
) {
  if lint {
    if correct {
      let new_part = format(part);
      if new_part == part {
        return;
      }

      let message = json!({"l": line,"c": col, "old": part, "new": new_part });

      text.push_str(message.to_string().as_str());
      text.push_str("\n")
    }
  } else {
    if correct {
      text.push_str(format(part).as_str());
    } else {
      text.push_str(part);
    }
  }
}
