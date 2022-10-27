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
        crate::config::setup_test();

        let example = r###"
---
title: iPad 和 iOS 接入的不同点
id: h
slug: /appstore/ipad_and_ios
original_slug: Web/CSS/网格-模板-列
---

page-tags: 美国, 中国,德国 , 法国

# 这是Heading 1大标题[示例](#示例)，代码内部：`minmax(最小值,最大值10)`不应该改变。

它指向一个[示例](#示例)

## This is list

1. List有序列表1
    1. 有序列表1.1
      2. 有序列表1.1.1
    1. 有序列表1.2
3. List有序列表2
  - 无序列表2.1
    无序列表paragraph

    无序列表paragraph，这里是第2段。

    * 无序列表2.1.1
    * 无序列表2.1.2

      无序列表paragraph，以及里面有`code代码`的例子，还有**加粗bold**。

  - 无序列表2.2
4. List有序列表3
  [ ] TODO列表1
  [x] TODO列表checked
  [X] TODO列表checked

## Test multiple code will not deadlock

`svh`, `lvh`, `dvh`, `svw`, `lvw`, `dvw`, `svmax`, `lvmax`, `dvmax`, `svmin`, `lvmin`, `dvmin` 

> **备注：** 你可以[添加新的条目](#Glossary)或完善条目。

它无需[握手](<https://zh.wikipedia.org/wiki/握手_(技术)>)会话。

- 你可以[添加新的条目](#Glossary)或完善条目。

<!-- 这里是comment文字 -->

你可以[添加新的条目](/zh-CN/docs/MDN/Contribute/Howto/Write_a_new_entry_in_the_Glossary)或改进、完善现有条目。

在paragraph中，这行用于测试在有中文的段落中，忽略halfwidth处理（测试：`{{jsxref("Object.setPrototypeOf")}}` World Wide Web）。

> blockquote中，忽略 [halfwidth](#halfwidth) World Wide Web。

- list中，忽略 [halfwidth](#halfwidth) World Wide Web。

This line is all in english，to test convert “full-width” into ‘half-width’！

https://google.com/foo/__ios__

**加粗** 
*倾斜*
~~删除线~~
这是**Bold加粗**在1个段落中，这端会correct掉，如果是inline code，例如`Rust语言`，也可以应该处理。

## （一）测试Heading处理,应该忽略#号后再处理.
###测试Heading处理，应该忽略#号后再处理.
####   测试Heading处理,应该忽略#号后再处理!

    // 这行不应该处理,因为无法识别codeblock的语言,test html code.
    $ echo linux ios

- 【括号】测试中文符号在List里面

> 引用文本：Quote也是可以的。
> （括号）在Quote里面

```rust
// Codeblock里面也会处理
let a = "你好hello";
```
    
```json
{ "json的key处理": "你好hello" }
```

<!-- autocorrect: false -->
这段应该ignore掉，不应该处理。
```rust
//这段应该ignore掉
```
<!-- autocorrect: true -->

```foobardar
这种非String类型的不会格式化
```

是则保持挂起直到被唤醒或超时。返回值为 "`ok`"、"**~~not-equal~~**" 或 `timeout` var。

## HTML标签里面的不处理，只处理文本

<div title="HTML标签里面都不处理"><h2>这是H2标题</h2><p>这里是p段落。</p></div>
<a href="#NTSC制式" />
<a href="https://zh.wikipedia.org/wiki/NTSC制式">NTSC制式</a>

```md
![流程图片](../../static/xxxx.png)
```

| 字段       | 长度(bit) | 长度（字节）| 说明                                                                       |
| ---------- | ---------- | ------------ | -------------------------------------------------------------------------- |
| request_id | 32(uint32) | 4            | 请求id，同一个连接的id需要唯一，从1开始，到达4294967295后从新开始。 |
| timeout    | 16(uint16) | 2            | `timeout` 单位毫秒，最大60000（60s）                                       |

- ![img图片](https://google.com/a/b/url不处理)
    - Sub list item
    - Third indent item.
- [link链接](https://google.com/a/b/url不处理)
- Escher puzzle（[链接](https://google.com)）
- 一个[[Wikilinks测试]]示例
"###;

        let expected = r###"
---
title: iPad 和 iOS 接入的不同点
id: h
slug: /appstore/ipad_and_ios
original_slug: Web/CSS/网格-模板-列
---

page-tags: 美国, 中国,德国 , 法国

# 这是 Heading 1 大标题[示例](#示例)，代码内部：`minmax(最小值,最大值10)`不应该改变。

它指向一个[示例](#示例)

## This is list

1. List 有序列表 1
    1. 有序列表 1.1
      2. 有序列表 1.1.1
    1. 有序列表 1.2
3. List 有序列表 2
  - 无序列表 2.1
    无序列表 paragraph

    无序列表 paragraph，这里是第 2 段。

    * 无序列表 2.1.1
    * 无序列表 2.1.2

      无序列表 paragraph，以及里面有`code代码`的例子，还有**加粗 bold**。

  - 无序列表 2.2
4. List 有序列表 3
  [ ] TODO 列表 1
  [x] TODO 列表 checked
  [X] TODO 列表 checked

## Test multiple code will not deadlock

`svh`, `lvh`, `dvh`, `svw`, `lvw`, `dvw`, `svmax`, `lvmax`, `dvmax`, `svmin`, `lvmin`, `dvmin` 

> **备注：** 你可以[添加新的条目](#Glossary)或完善条目。

它无需[握手](<https://zh.wikipedia.org/wiki/握手_(技术)>)会话。

- 你可以[添加新的条目](#Glossary)或完善条目。

<!-- 这里是 comment 文字 -->

你可以[添加新的条目](/zh-CN/docs/MDN/Contribute/Howto/Write_a_new_entry_in_the_Glossary)或改进、完善现有条目。

在 paragraph 中，这行用于测试在有中文的段落中，忽略 halfwidth 处理（测试：`{{jsxref("Object.setPrototypeOf")}}` World Wide Web）。

> blockquote 中，忽略 [halfwidth](#halfwidth) World Wide Web。

- list 中，忽略 [halfwidth](#halfwidth) World Wide Web。

This line is all in english, to test convert "full-width" into 'half-width'!

https://google.com/foo/__ios__

**加粗** 
*倾斜*
~~删除线~~
这是**Bold 加粗**在 1 个段落中，这端会 correct 掉，如果是 inline code，例如`Rust语言`，也可以应该处理。

## （一）测试 Heading 处理，应该忽略#号后再处理。
###测试 Heading 处理，应该忽略#号后再处理。
####   测试 Heading 处理，应该忽略#号后再处理！

    // 这行不应该处理,因为无法识别codeblock的语言,test html code.
    $ echo linux ios

- 【括号】测试中文符号在 List 里面

> 引用文本：Quote 也是可以的。
> （括号）在 Quote 里面

```rust
// Codeblock 里面也会处理
let a = "你好 hello";
```
    
```json
{ "json的key处理": "你好 hello" }
```

<!-- autocorrect: false -->
这段应该ignore掉，不应该处理。
```rust
//这段应该ignore掉
```
<!-- autocorrect: true -->

```foobardar
这种非String类型的不会格式化
```

是则保持挂起直到被唤醒或超时。返回值为 "`ok`"、"**~~not-equal~~**" 或 `timeout` var。

## HTML 标签里面的不处理，只处理文本

<div title="HTML标签里面都不处理"><h2>这是 H2 标题</h2><p>这里是 p 段落。</p></div>
<a href="#NTSC制式" />
<a href="https://zh.wikipedia.org/wiki/NTSC制式">NTSC 制式</a>

```md
![流程图片](../../static/xxxx.png)
```

| 字段       | 长度 (bit) | 长度（字节）| 说明                                                                       |
| ---------- | ---------- | ------------ | -------------------------------------------------------------------------- |
| request_id | 32(uint32) | 4            | 请求 id，同一个连接的 id 需要唯一，从 1 开始，到达 4294967295 后从新开始。 |
| timeout    | 16(uint16) | 2            | `timeout` 单位毫秒，最大 60000（60s）                                       |

- ![img 图片](https://google.com/a/b/url不处理)
    - Sub list item
    - Third indent item.
- [link 链接](https://google.com/a/b/url不处理)
- Escher puzzle（[链接](https://google.com)）
- 一个[[Wikilinks测试]]示例
"###;

        assert_eq!(expected, format_for(example, "markdown").to_string());

        let expected_json_result = r#"{
  "filepath": "markdown",
  "lines": [],
  "error": ""
}"#;

        let lint_result = lint_for(expected, "markdown");
        assert_eq!(false, lint_result.has_error());
        assert_eq!(expected_json_result, lint_result.to_json_pretty());
    }

    #[test]
    fn test_lint_for_inline_code() {
        crate::config::setup_test();

        let raw = r###"
## Spellcheck测试ios和html和WIFI

```rb
# 这里是markdown缩进的codeblock
wifi = "ios"
```

    // 这行不应该处理，因为无法识别codeblock的语言
    $ echo ios
    wifi = "ios"
    $ echo html

这里是普通的段落。
"###;

        let json_result = r##"{"filepath":"markdown","lines":[{"l":2,"c":4,"new":"Spellcheck 测试 iOS 和 HTML 和 Wi-Fi","old":"Spellcheck测试ios和html和WIFI","severity":1},{"l":5,"c":1,"new":"# 这里是 markdown 缩进的 codeblock","old":"# 这里是markdown缩进的codeblock","severity":1}],"error":""}"##;

        let lint_result = lint_for(raw, "markdown");
        assert_eq!(json_result, lint_result.to_json());
        assert_eq!(false, lint_result.has_error());
        assert_eq!(2, lint_result.lines.len());
        assert_eq!(
            "Spellcheck 测试 iOS 和 HTML 和 Wi-Fi",
            lint_result.lines[0].new
        );
        assert_eq!(
            "# 这里是 markdown 缩进的 codeblock",
            lint_result.lines[1].new
        );
    }
}
