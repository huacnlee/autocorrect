// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar//strings.pest"]
struct StringsParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_format_javascript() {
        let example = r###"
/* 
    InfoPlist.strings测试
    Created by某某
*/

"CFBundleDisplayName" = "App名称";//app中文名称
"CFBundleIdentifier" = "huacnlee.autocorrect";

"NSCameraUsageDescription" = "开启Wi-Fi后继续使用";
// 单行comment
"中文key测试" = "开启定位权限";
"###;

        let expect = r###"
/* 
    InfoPlist.strings 测试
    Created by 某某
*/

"CFBundleDisplayName" = "App 名称";//app 中文名称
"CFBundleIdentifier" = "huacnlee.autocorrect";

"NSCameraUsageDescription" = "开启 Wi-Fi 后继续使用";
// 单行 comment
"中文 key 测试" = "开启定位权限";
"###;

        assert_eq!(expect, format_strings(example).to_string());
    }
}
