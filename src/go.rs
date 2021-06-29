use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/go.pest"]
struct GoParser;

pub fn format_go(text: &str) -> String {
  let result = GoParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_go_pair(&mut out, item);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_go_pair(text: &mut String, item: Pair<Rule>) {
  match item.as_rule() {
    Rule::string | Rule::comment => text.push_str(format(item.as_str()).as_str()),
    Rule::item => {
      for sub in item.into_inner() {
        format_go_pair(text, sub);
      }
    }
    _ => text.push_str(item.as_str()),
  }
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

    assert_eq!(expect, format_go(example));
  }
}
