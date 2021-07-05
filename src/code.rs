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

fn format_pair<R: RuleType>(text: &mut String, item: Pair<R>, lint: bool) {
  let rule = item.as_rule();
  let rule_name = format!("{:?}", rule);

  // println!("rule: {}", rule_name);

  match rule_name.as_str() {
    "string" | "link_string" | "text" | "comment" => format_or_lint(text, item, lint),
    _ => {
      let mut child_count = 0;
      let item_str = item.as_str();
      for child in item.into_inner() {
        format_pair(text, child, lint);
        child_count += 1;
      }

      if child_count == 0 {
        if !lint {
          text.push_str(item_str);
        }
      }
    }
  };
}

fn format_or_lint<R: RuleType>(text: &mut String, item: Pair<R>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  if lint {
    let new_part = format(part);
    if new_part == part {
      return;
    }

    let message = json!({"l": line,"c": col, "old": part, "new": new_part });

    text.push_str(message.to_string().as_str());
    text.push_str("\n")
  } else {
    text.push_str(format(part).as_str());
  }
}
