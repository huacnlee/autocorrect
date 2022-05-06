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
    ---
    title: IPAD 和 Ios 接入的不同点
    id: h
    slug: /appstore/ipad_and_ios
    ---

    # 这是Heading 1大标题

    https://google.com/foo/__ios__/**ios**

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
        
    ```json
    { "json的key处理": "你好hello" }
    ```

    ```foobardar
    这种非String类型的不会格式化
    ```

    ```md
    ![流程图片](../../static/xxxx.png)
    ```

    | 字段       | 长度(bit) | 长度（字节）| 说明                                                                       |
    | ---------- | ---------- | ------------ | -------------------------------------------------------------------------- |
    | request_id | 32(uint32) | 4            | 请求id，同一个连接的id需要唯一，从1开始，到达4294967295后从新开始。 |
    | timeout    | 16(uint16) | 2            | `timeout` 单位毫秒，最大60000（60s）                                       |
    
    - ![img图片](https://google.com/a/b/url不处理)
    - [link链接](https://google.com/a/b/url不处理)
    "###;

        let expected = r###"
    ---
    title: iPad 和 iOS 接入的不同点
    id: h
    slug: /appstore/ipad_and_ios
    ---

    # 这是 Heading 1 大标题

    https://google.com/foo/__ios__/**ios**

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
        
    ```json
    { "json的key处理": "你好 hello" }
    ```

    ```foobardar
    这种非String类型的不会格式化
    ```

    ```md
    ![流程图片](../../static/xxxx.png)
    ```

    | 字段       | 长度 (bit) | 长度（字节）| 说明                                                                       |
    | ---------- | ---------- | ------------ | -------------------------------------------------------------------------- |
    | request_id | 32(uint32) | 4            | 请求 id，同一个连接的 id 需要唯一，从 1 开始，到达 4294967295 后从新开始。 |
    | timeout    | 16(uint16) | 2            | `timeout` 单位毫秒，最大 60000（60s）                                       |
    
    - ![img 图片](https://google.com/a/b/url不处理)
    - [link 链接](https://google.com/a/b/url不处理)
    "###;

        assert_eq!(expected, format_markdown(example).to_string());

        let lint_result = lint_markdown(expected);
        assert_eq!(false, lint_result.has_error());
        if !lint_result.lines.is_empty() {
            panic!("{}", lint_result.to_string());
        }
    }
}
