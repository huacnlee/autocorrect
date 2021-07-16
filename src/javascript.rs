// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/javascript.pest"]
struct JavaScriptParser;

#[allow(dead_code)]
pub fn format_javascript(text: &str, lint: bool) -> String {
    let pairs = JavaScriptParser::parse(Rule::item, text);
    return code::format_pairs(text, pairs, lint);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_javascript() {
        let example = r###"
// 第 1 行注释
// 第 2 行注释
function helloWorld(a) {
  const a = '第 1 个';
  const b = "第 2 个" + "第 3 个";
  const c = `这是string第1行
  这是string第2行`;
}
"###;

        let expect = r###"
// 第 1 行注释
// 第 2 行注释
function helloWorld(a) {
  const a = '第 1 个';
  const b = "第 2 个" + "第 3 个";
  const c = `这是 string 第 1 行
  这是 string 第 2 行`;
}
"###;

        assert_eq!(expect, format_javascript(example, false));
    }

    #[test]
    fn it_lint_javascript() {
        let example = r###"
    /**
    * Hello你好
    * 这是第2行
    */
    function application() {
      let example = "这是single line单行注释";
      console.log(`这是string第1行
      这是string第2行
      `)
    }
    "###;

        let expect = r###"
{"c":4,"l":2,"new":"* Hello 你好","old":"* Hello你好"}
{"c":1,"l":3,"new":"* 这是第 2 行","old":"* 这是第2行"}
{"c":0,"l":7,"new":"\"这是 single line 单行注释\"","old":"\"这是single line单行注释\""}
{"c":0,"l":8,"new":"`这是 string 第 1 行","old":"`这是string第1行"}
{"c":13,"l":9,"new":"这是 string 第 2 行","old":"这是string第2行"}
    "###;

        assert_eq!(expect.trim(), format_javascript(example, true).trim());
    }
}
