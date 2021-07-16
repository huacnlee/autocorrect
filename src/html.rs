// autocorrect: false
use super::*;

use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/html.pest"]
struct JavaParser;

#[allow(dead_code)]
pub fn format_html(text: &str) -> code::FormatResult {
    let pairs = JavaParser::parse(Rule::item, text);

    let out = code::FormatResult::new(text);
    return code::format_pairs(out, pairs);
}

#[allow(dead_code)]
pub fn lint_html(text: &str) -> code::LintResult {
    let pairs = JavaParser::parse(Rule::item, text);

    let out = code::LintResult::new(text);
    return code::format_pairs(out, pairs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    macro_rules! assert_html_eq {
        ($expected:expr, $actual:expr) => {{
            let re = Regex::new(">\\s+<").unwrap();
            let expected = $expected;
            let actual = $actual;
            let expected_clean = &re.replace_all(expected.trim(), "><");
            let actual_clean = &re.replace_all(actual.trim(), "><");

            if expected_clean != actual_clean {
                panic!(
                    "\nexpected:\n{}\n----------------------------------------\nactual:\n{}",
                    expected, actual
                )
            }
        }};
    }

    #[test]
    fn test_format_html() {
        let html = r###"
        <!DOCTYPE html>
        <html xmlns=http://www.w3.org/1999/xhtml>
        <article>
        <h1>编译Rust为WebAssembly</h1>
        <style type="text/css" nofollow>
        .body { font-size: 14px; }
        </style>
        <script type="text/javascript">
        // 这个不能script里面不能转换
        window['__abbaidu_2036_subidgetf'] = function () {var subid = 'feed_landing_super';return subid;};window['__abbaidu_2036_cb'] = function (responseData) {};
        </script>
        <script async src=https://dlswbr.baidu.com/heicha/mw/abclite-2036-s.js></script>
        <div class="content">
            <p>如果你写了一些Rust代码，你可以把它编译成WebAssembly！这份教程将带你编译Rust项目为wasm并在一个现存的web应用中使用它。</p>
            <a href="#rust_和_webassembly_用例" title="Permalink to Rust 和 WebAssembly 用例">Rust和WebAssembly用例</a>
            <h2>Rust和WebAssembly用例</h2>
            <div><p>Rust 和 WebAssembly 有两大主要用例：</p>
            <ul>
            <li>构建完整应用——整个Web应用都基于Rust开发！</li>
            <li>构建应用的组成部分——在现存的JavaScript前端中使用Rust。</li>
            </ul>
            <p>目前，Rust团队正专注于第二种用例，因此我们也将着重介绍它。对于第一种用例，可以参阅&nbsp;<code><a href="https://github.com/DenisKolodin/yew" class="external" rel=" noopener">yew</a></code>&nbsp;这类项目。</p>
            <p>在本教程中，我们将使用Rust的npm包构建工具<code>wasm-pack</code>来构建一个npm包。这个包只包含WebAssembly和JavaScript代码，以便包的用户无需安装Rust就能使用。他们甚至不需要知道这里包含WebAssembly！</p></div>
        </div>
        </article>
        </html>
        "###;

        let expected = r###"
        <!DOCTYPE html>
        <html xmlns=http://www.w3.org/1999/xhtml>
        <article>
        <h1>编译 Rust 为 WebAssembly</h1>
        <style type="text/css" nofollow>
        .body { font-size: 14px; }
        </style>
        <script type="text/javascript">
        // 这个不能script里面不能转换
        window['__abbaidu_2036_subidgetf'] = function () {var subid = 'feed_landing_super';return subid;};window['__abbaidu_2036_cb'] = function (responseData) {};
        </script>
        <script async src=https://dlswbr.baidu.com/heicha/mw/abclite-2036-s.js></script>
        <div class="content">
            <p>如果你写了一些 Rust 代码，你可以把它编译成 WebAssembly！这份教程将带你编译 Rust 项目为 wasm 并在一个现存的 web 应用中使用它。</p>
            <a href="#rust_和_webassembly_用例" title="Permalink to Rust 和 WebAssembly 用例">Rust 和 WebAssembly 用例</a>
            <h2>Rust 和 WebAssembly 用例</h2>
            <div><p>Rust 和 WebAssembly 有两大主要用例：</p>
            <ul>
            <li>构建完整应用——整个 Web 应用都基于 Rust 开发！</li>
            <li>构建应用的组成部分——在现存的 JavaScript 前端中使用 Rust。</li>
            </ul>
            <p>目前，Rust 团队正专注于第二种用例，因此我们也将着重介绍它。对于第一种用例，可以参阅&nbsp;<code><a href="https://github.com/DenisKolodin/yew" class="external" rel=" noopener">yew</a></code>&nbsp;这类项目。</p>
            <p>在本教程中，我们将使用 Rust 的 npm 包构建工具<code>wasm-pack</code>来构建一个 npm 包。这个包只包含 WebAssembly 和 JavaScript 代码，以便包的用户无需安装 Rust 就能使用。他们甚至不需要知道这里包含 WebAssembly！</p></div>
        </div>
        </article>
        </html>
        "###;

        assert_html_eq!(expected, format_html(html).to_string())
    }

    #[test]
    fn test_format_html_with_fullpage() {
        let html = r#"
        <html><head><title>Hello</title></head>
        <body>
        <article>
        <h1>这是Heading标题</h1>
        <div class="content">
            <p>你好Rust世界<strong>Bold文本</strong></p>
            <p>这是第二行p标签</p>
        </div>
        </article>
        </body>
        </html>"#;

        let expected = r#"
        <html><head><title>Hello</title></head>
        <body>
        <article>
        <h1>这是 Heading 标题</h1>
        <div class="content">
            <p>你好 Rust 世界<strong>Bold 文本</strong></p>
            <p>这是第二行 p 标签</p>
        </div>
        </article>
        </body>
        </html>"#;

        assert_html_eq!(expected, format_html(html).to_string())
    }

    #[test]
    fn test_lint_html() {
        let html = r#"
        <html><head><title>Hello</title></head>
        <body>
        <article>
        <h1>这是Heading标题</h1>
        <div class="content">
            <p>你好Rust世界<strong>Bold文本</strong></p>
            <p>这是第二行p标签</p>
        </div>
        </article>
        </body>
        </html>"#;

        let json = r#"""
        {
            "filepath": "",
            "lines": [
              { "l": 5, "c": 13, "new": "这是 Heading 标题", "old": "这是Heading标题" },
              { "l": 7, "c": 16, "new": "你好 Rust 世界", "old": "你好Rust世界" },
              { "l": 7, "c": 32, "new": "Bold 文本", "old": "Bold文本" },
              { "l": 8, "c": 16, "new": "这是第二行 p 标签", "old": "这是第二行p标签" }
            ],
            "error": ""
        }
        """#;

        let lint_result = lint_html(html);
        assert_eq!("", lint_result.error);
        assert_eq!(4, lint_result.lines.len());
    }
}
