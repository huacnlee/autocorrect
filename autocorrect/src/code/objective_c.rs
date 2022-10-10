// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/objective_c.pest"]
struct ObjectiveCParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_objective_c() {
        let example = r###"
// 第1行注释
// 第2行注释
- (void) helloWorld {
  // 第3行注释
  NSString *geotestUrl = @"第1个字符串string";

  NSLocalizedString(@"hello世界不会修改", nil);

  NSLocalizedString(
    @"hello世界不会修改", nil);

  NSRegularExpression* re0 = [NSRegularExpression regularExpressionWithPattern:  @"re正则" options:0 error:&err];
}
"###;

        let expect = r###"
// 第 1 行注释
// 第 2 行注释
- (void) helloWorld {
  // 第 3 行注释
  NSString *geotestUrl = @"第 1 个字符串 string";

  NSLocalizedString(@"hello世界不会修改", nil);

  NSLocalizedString(
    @"hello世界不会修改", nil);

  NSRegularExpression* re0 = [NSRegularExpression regularExpressionWithPattern:  @"re正则" options:0 error:&err];
}
"###;

        assert_eq!(expect, format_for(example, "objective_c").to_string());
    }
}
