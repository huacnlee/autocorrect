// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/sql.pest"]
struct SQLParser;

pub fn format_sql(text: &str, lint: bool) -> String {
  let result = SQLParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_sql_pair(&mut out, item, lint);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_sql_pair(text: &mut String, item: Pair<Rule>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  match item.as_rule() {
    Rule::string | Rule::comment => format_or_lint(text, part, true, lint, line, col),
    Rule::item => {
      for sub in item.into_inner() {
        format_sql_pair(text, sub, lint);
      }
    }
    _ => format_or_lint(text, part, true, lint, line, col),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_sql() {
    let example = r#"
SELECT * FROM "topics" WHERE "id" = ?;
COMMENT ON COLUMN "topics"."user_id" IS 'topic创建者';
-- Comment注释信息
COMMENT ON COLUMN "topics"."status" IS '3屏蔽 1审核中 2已发布';
/* 
  多行，且带有换行
  注释comment信息 
*/
COMMENT ON COLUMN "topics"."kind" IS '0普通 1转发';
"#;

    let expect = r#"
SELECT * FROM "topics" WHERE "id" = ?;
COMMENT ON COLUMN "topics"."user_id" IS 'topic 创建者';
-- Comment 注释信息
COMMENT ON COLUMN "topics"."status" IS '3 屏蔽 1 审核中 2 已发布';
/* 
  多行，且带有换行
  注释 comment 信息 
*/
COMMENT ON COLUMN "topics"."kind" IS '0 普通 1 转发';
"#;

    assert_eq!(expect, format_sql(example, false));
  }
}
