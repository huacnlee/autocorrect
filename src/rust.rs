// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/rust.pest"]
struct RustParser;

#[allow(dead_code)]
pub fn format_rust(text: &str, lint: bool) -> String {
  let pairs = RustParser::parse(Rule::item, text);
  return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_rust() {
    let example = r###"
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let mut largest = number_list[0];

  // 1第一行Single line注释
  // 2第二行注释
  for number in number_list {
      if number > largest {
          largest = number;
      }
  }

  let a = r#"
这是第1行
这是第2行
"#;

  let b = r##"
这是第 3 行
这是第 4 行
"##;

  /**
   * 多行Rust注释
   * 第二行Rust注释
  */
  println!("最大的数字number是{}", largest);
}
"###;

    let expect = r###"
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let mut largest = number_list[0];

  // 1 第一行 Single line 注释
  // 2 第二行注释
  for number in number_list {
      if number > largest {
          largest = number;
      }
  }

  let a = r#"
这是第 1 行
这是第 2 行
"#;

  let b = r##"
这是第 3 行
这是第 4 行
"##;

  /**
   * 多行 Rust 注释
   * 第二行 Rust 注释
  */
  println!("最大的数字 number 是{}", largest);
}
"###;

    assert_eq!(expect, format_rust(example, false));
  }
}
