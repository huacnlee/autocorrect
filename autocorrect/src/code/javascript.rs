// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/javascript.pest"]
struct JavaScriptParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_javascript() {
        let example = r###"
// 第1行注释
// 第2行注释
function helloWorld(a) {
  const a = '第1个';
  const b = "第2个" + "第3个";
  const raw = `Hello你好`;
  const re = /包含#regexp测试/;
  const re1 = new RegExp("RegExp不处理");
  const re2 = new RegExp('不处理RegExp');
  const str_literal = `这个${foo}不会处理`;

  /**
   * Hello你好
   * 这是第2行
   */
  const c = `这是string第1行
  这是string第2行`;

  // autocorrect-disable
  const disable_1 = "这行将会disable掉";
  const disable_2 = "这行将也会disable掉";
  // autocorrect-enable

  return <>
    <div className="react-name">
        <List renderItem={(item) => (
          <Item className="list-item">
            <span>nested项</span>
            <span>{item}</span>
          </Item>
        )} />
        <h1>Hello你好<strong>你好foo世界</strong></h1>
        外部HTML结果
        <div>{ a && t("这里string也要处理")}</div>
    </div>
  </>
}

const map = {
  "在Map中key不转换": "在Map中value要转换",
  children: {
    "Children中的key也不转换": "Children中的value要转换",
  }
}
"###;

        let expect = r###"
// 第 1 行注释
// 第 2 行注释
function helloWorld(a) {
  const a = '第 1 个';
  const b = "第 2 个" + "第 3 个";
  const raw = `Hello 你好`;
  const re = /包含#regexp测试/;
  const re1 = new RegExp("RegExp不处理");
  const re2 = new RegExp('不处理RegExp');
  const str_literal = `这个${foo}不会处理`;

  /**
   * Hello 你好
   * 这是第 2 行
   */
  const c = `这是 string 第 1 行
  这是 string 第 2 行`;

  // autocorrect-disable
  const disable_1 = "这行将会disable掉";
  const disable_2 = "这行将也会disable掉";
  // autocorrect-enable

  return <>
    <div className="react-name">
        <List renderItem={(item) => (
          <Item className="list-item">
            <span>nested 项</span>
            <span>{item}</span>
          </Item>
        )} />
        <h1>Hello 你好<strong>你好 foo 世界</strong></h1>
        外部 HTML 结果
        <div>{ a && t("这里 string 也要处理")}</div>
    </div>
  </>
}

const map = {
  "在Map中key不转换": "在 Map 中 value 要转换",
  children: {
    "Children中的key也不转换": "Children 中的 value 要转换",
  }
}
"###;

        assert_eq!(expect, format_for(example, "javascript").to_string());
    }

    macro_rules! assert_json_eq {
        ($expected:expr, $actual:expr) => {{
            let expected = $expected;
            let actual = $actual;

            let expect_json =
                serde_json::from_str(expected).unwrap_or(serde_json::Value::default());
            let result =
                serde_json::from_str(actual.as_str()).unwrap_or(serde_json::Value::default());
            assert_eq!(expect_json, result);
        }};
    }

    #[test]
    fn it_format_javascript_without_any_string() {
        let example = r###"
        function helloWorld(a) {
            const a = "";
            return <div className="tags">
               {tags.map(tag => <Tag color="orange"><Icon name="label" /> {tag.name}</Tag>)}
            </div>;
        }
        "###;

        let expect = r###"
        function helloWorld(a) {
            const a = "";
            return <div className="tags">
               {tags.map(tag => <Tag color="orange"><Icon name="label" /> {tag.name}</Tag>)}
            </div>;
        }
        "###;

        assert_eq!(expect, format_for(example, "javascript").to_string());
    }

    #[test]
    fn it_lint_javascript() {
        let example = r###"
    /**
    * Hello你好IOS应用
    * 好的 IOS 应用
    * 这是第2行
    */
    function application() {
      let example = "这是single line单行注释";
      console.log(`这是string第1行
      这是string第2行
      `)

      // autocorrect-disable
      const disable_1 = "这行将会disable掉";
      const disable_2 = "这行将也会disable掉";
      // autocorrect-enable

      const c = "这是string第3行";
    }
    "###;

        let expect = r###"
        {
          "filepath": "test.js",
          "lines": [
            { "l": 3, "c": 5, "new": "* Hello 你好 iOS 应用", "old": "* Hello你好IOS应用", "severity": 1 },
            { "l": 4, "c": 5, "new": "* 好的 iOS 应用", "old": "* 好的 IOS 应用", "severity": 2 },
            { "l": 5, "c": 5, "new": "* 这是第 2 行", "old": "* 这是第2行", "severity": 1 },
            { "l": 8, "c": 21, "new": "\"这是 single line 单行注释\"", "old": "\"这是single line单行注释\"", "severity": 1 },
            { "l": 9, "c": 19, "new": "`这是 string 第 1 行", "old": "`这是string第1行", "severity": 1 },
            { "l": 10, "c": 7, "new": "这是 string 第 2 行", "old": "这是string第2行", "severity": 1 },
            { "l": 18, "c": 17, "new": "\"这是 string 第 3 行\"", "old": "\"这是string第3行\"", "severity": 1 }
          ],
          "error": ""
        }
    "###;

        let lint_result = lint_for(example, "test.js").to_json();

        assert_json_eq!(expect, lint_result);
        assert_json_eq!(expect, lint_for(example, "test.js").to_json_pretty());
    }
}
