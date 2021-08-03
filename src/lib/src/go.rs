// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/go.pest"]
struct GoParser;

#[allow(dead_code)]
pub fn format_go(text: &str) -> code::FormatResult {
  let pairs = GoParser::parse(Rule::item, text);
  let text = code::FormatResult::new(text);
  return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_go(text: &str) -> code::LintResult {
  let pairs = GoParser::parse(Rule::item, text);
  let text = code::LintResult::new(text);
  return code::format_pairs(text, pairs);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_go() {
    let example = r###"
// WithContext创建基于ctx的db
// 第2行注释
func (d *Dao) WithContext(ctx context.Context) (db *gorm.DB) {
  a := "第1个"
  b := `
多行string
第2行
`
  re := regexp.MustCompile(`regexp不处理`)
  re1 := regexp.Compile("regexp不处理")
  t := time.Parse("2006年01月02日 15:04", t)

  fmt.Println(a + b + "go语言")
  fmt.Println("%s链接的内容不会空格%d也不处理，保守", "格式", 100)
  db = d.DB.WithContext(ctx)
  return
}
"###;

    let expect = r###"
// WithContext 创建基于 ctx 的 db
// 第 2 行注释
func (d *Dao) WithContext(ctx context.Context) (db *gorm.DB) {
  a := "第 1 个"
  b := `
多行 string
第 2 行
`
  re := regexp.MustCompile(`regexp不处理`)
  re1 := regexp.Compile("regexp不处理")
  t := time.Parse("2006年01月02日 15:04", t)

  fmt.Println(a + b + "go 语言")
  fmt.Println("%s链接的内容不会空格%d也不处理，保守", "格式", 100)
  db = d.DB.WithContext(ctx)
  return
}
"###;

    assert_eq!(expect, format_go(example).to_string());
  }
}
