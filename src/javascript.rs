// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/javascript.pest"]
struct JavaScriptParser;

#[allow(dead_code)]
pub fn format_javascript(text: &str) -> code::FormatResult {
    let pairs = JavaScriptParser::parse(Rule::item, text);
    let text = code::FormatResult::new(text);
    return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_javascript(text: &str) -> code::LintResult {
    let pairs = JavaScriptParser::parse(Rule::item, text);
    let text = code::LintResult::new(text);
    return code::format_pairs(text, pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_format_javascript() {
        let example = r###"
// 第 1 行注释
// 第 2 行注释
function helloWorld(a) {
  const a = '第 1 个';
  const b = "第 2 个" + "第 3 个";
  const re = /包含#regexp测试/;
  const str_literal = `这个${foo}不会处理`;

  /**
   * Hello你好
   * 这是第2行
   */
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
  const re = /包含#regexp测试/;
  const str_literal = `这个${foo}不会处理`;

  /**
   * Hello 你好
   * 这是第 2 行
   */
  const c = `这是 string 第 1 行
  这是 string 第 2 行`;
}
"###;

        assert_eq!(expect, format_javascript(example).to_string());
    }

    macro_rules! assert_json_eq {
        ($expected:expr, $actual:expr) => {{
            let expected = $expected;
            let actual = $actual;

            let expect_json =
                serde_json::from_str(expected).unwrap_or(serde_json::Value::default());
            let result =
                serde_json::from_str(actual.as_str()).unwrap_or(serde_json::Value::default());
            pretty_assertions::assert_eq!(expect_json, result);
        }};
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
        {
            "filepath":"",
            "lines":[
                {"c":5,"l":3,"new":"* Hello 你好","old":"* Hello你好"},
                {"c":5,"l":4,"new":"* 这是第 2 行","old":"* 这是第2行"},
                {"c":21,"l":7,"new":"\"这是 single line 单行注释\"","old":"\"这是single line单行注释\""},
                {"c":19,"l":8,"new":"`这是 string 第 1 行","old":"`这是string第1行"},
                {"c":7,"l":9,"new":"这是 string 第 2 行","old":"这是string第2行"}
            ],
            "error": ""
        }
    "###;

        assert_json_eq!(expect, lint_javascript(example).to_json());
        assert_json_eq!(expect, lint_javascript(example).to_json_pretty());
    }
}
