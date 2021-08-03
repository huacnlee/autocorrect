// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/sql.pest"]
struct SQLParser;

#[allow(dead_code)]
pub fn format_sql(text: &str) -> code::FormatResult {
  let pairs = SQLParser::parse(Rule::item, text);
  let text = code::FormatResult::new(text);
  return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_sql(text: &str) -> code::LintResult {
  let pairs = SQLParser::parse(Rule::item, text);
  let text = code::LintResult::new(text);
  return code::format_pairs(text, pairs);
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

    assert_eq!(expect, format_sql(example).to_string());
  }
}
