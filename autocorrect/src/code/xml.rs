// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/xml.pest"]
struct XMLParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_xml() {
        let example = r#"
<?xml version="1.0" encoding="UTF-8"?>
<resources>
    <item name="foo" />
    <item><string name="record_item_amount">金额100美元</string></item>
    <string name="description">香港证监会第1、4、9类牌照.</string>
    <string name="cdata"><![CDATA[100美元]]></string>
</resources>    
"#;

        let expect = r#"
<?xml version="1.0" encoding="UTF-8"?>
<resources>
    <item name="foo" />
    <item><string name="record_item_amount">金额 100 美元</string></item>
    <string name="description">香港证监会第 1、4、9 类牌照。</string>
    <string name="cdata"><![CDATA[100 美元]]></string>
</resources>    
"#;

        assert_eq!(expect, format_for(example, "xml").to_string())
    }
}
