// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/kotlin.pest"]
struct KotlinParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_kotlin() {
        let example = r###"
/** 
 * 第1行注释
 * 第2行注释
 */
fun helloWorld(name: String) {
  // 第3行注释
  var singleLineString = "第1个字符串string"

  var quotation = """
  这是多行string里面包含"双引号"
  "Begin at the beginning," the King said gravely.
  """

  var re0 = Regex("re正则" )
  var re1 = "re正则".toRegex()
}
"###;

        let expect = r###"
/** 
 * 第 1 行注释
 * 第 2 行注释
 */
fun helloWorld(name: String) {
  // 第 3 行注释
  var singleLineString = "第 1 个字符串 string"

  var quotation = """
  这是多行 string 里面包含"双引号"
  "Begin at the beginning," the King said gravely.
  """

  var re0 = Regex("re正则" )
  var re1 = "re正则".toRegex()
}
"###;

        assert_eq!(expect, format_for(example, "kotlin").to_string());
    }
}
