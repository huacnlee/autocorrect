// autocorrect: false
use super::*;

use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/markdown.pest"]
struct MarkdownParser;

#[allow(dead_code)]
pub fn format_markdown(text: &str) -> code::FormatResult {
    let pairs = MarkdownParser::parse(Rule::item, text);
    let text = code::FormatResult::new(text);
    return code::format_pairs(text, pairs);
}

#[allow(dead_code)]
pub fn lint_markdown(text: &str) -> code::LintResult {
    let pairs = MarkdownParser::parse(Rule::item, text);
    let text = code::LintResult::new(text);
    return code::format_pairs(text, pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_markdown() {
        let example = r###"
    # 这是Heading 1大标题

    这是**Bold加粗**在1个段落中，这端会correct掉，如果是inline code，例如`Rust语言`，也可以应该处理。
    
    > 引用文本：Quote也是可以的。
    
    ```rust
    // Codeblock里面也会处理
    let a = "你好hello";
    ```
    
    - ![img图片](https://google.com/a/b/url不处理)
    - [link链接](https://google.com/a/b/url不处理)
    "###;

        let expected = r###"
    # 这是 Heading 1 大标题

    这是**Bold 加粗**在 1 个段落中，这端会 correct 掉，如果是 inline code，例如`Rust 语言`，也可以应该处理。
    
    > 引用文本：Quote 也是可以的。
    
    ```rust
    // Codeblock 里面也会处理
    let a = "你好 hello";
    ```
    
    - ![img 图片](https://google.com/a/b/url不处理)
    - [link 链接](https://google.com/a/b/url不处理)
    "###;

        assert_eq!(expected, format_markdown(example).to_string())
    }
}
