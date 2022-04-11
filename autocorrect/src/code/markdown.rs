// autocorrect: false
use super::*;

use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/markdown.pest"]
struct MarkdownParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_format_markdown() {
        let example = r###"
    # 这是Heading 1大标题

    **加粗** 
    *倾斜*
    ~~删除线~~
    这是**Bold加粗**在1个段落中，这端会correct掉，如果是inline code，例如`Rust语言`，也可以应该处理。

    ## （一）测试Heading处理,应该忽略#号后再处理.
    ###测试Heading处理，应该忽略#号后再处理.
    ####   测试Heading处理,应该忽略#号后再处理!

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

    **加粗** 
    *倾斜*
    ~~删除线~~
    这是**Bold 加粗**在 1 个段落中，这端会 correct 掉，如果是 inline code，例如`Rust 语言`，也可以应该处理。

    ## （一）测试 Heading 处理，应该忽略#号后再处理。
    ###测试 Heading 处理，应该忽略#号后再处理。
    ####   测试 Heading 处理，应该忽略#号后再处理！

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
