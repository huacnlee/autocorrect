// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/go.pest"]
struct GoParser;

#[allow(dead_code)]
pub fn format_go(text: &str, lint: bool) -> String {
  let pairs = GoParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
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
  fmt.Println(a + b + "go语言")
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
  fmt.Println(a + b + "go 语言")
  db = d.DB.WithContext(ctx)
  return
}
"###;

    assert_eq!(expect, format_go(example, false));
  }
}
