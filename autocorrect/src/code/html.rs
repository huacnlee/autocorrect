// autocorrect: false
use super::*;

use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar//html.pest"]
struct HTMLParser;

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

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
        bad html
        <% a = 1 %>
        {% hello = a %}
        <!DOCTYPE html>
        <!-- html的注释 -->
        <html xmlns=http://www.w3.org/1999/xhtml>
        <title><%= title %>或者{{ title }}</title>
        <article>
        <h1>编译Rust为WebAssembly</h1>
        <style type="text/css" nofollow>
        /* 在css里面的注释会转换 */
        .body { font-size: 14px; } /* 后面个comment注释 */
        </style>
        <script type="text/javascript">
        // 这个script也会转换
        // 按照javascript的方式来处理
        const a = "hello你好";
        /**
         * 多行comment测试
         * 多行第2行
         */
        </script>
        <script async src=https://dlswbr.baidu.com/heicha/mw/abclite-2036-s.js></script>
        <div class="content">
            <p>如果你写了一些Rust代码，你可以把它编译成WebAssembly！这份教程将带你编译Rust项目为wasm并在一个现存的web应用中使用它。</p>
            <a href="#rust_和_webassembly_用例" title="Permalink to Rust 和 WebAssembly 用例">Rust和WebAssembly用例</a>
            <h2>Rust和WebAssembly用例</h2>
            <div @click.prevent="hello" :name="foo" #bar="dar"><p>Rust 和 WebAssembly 有两大主要用例：</p>
            <ul>
            <li>构建完整应用——整个Web应用都基于Rust开发！</li>
            <li>构建应用的组成部分——在现存的JavaScript前端中使用Rust。</li>
            <%= link_to "FTP管理", "/", class: "subnav-item #{(params[:title_tab].blank? || params[:title_tab] == 'sftp_index') ? 'active' : ''}" %>
            </ul>
            <p>目前，Rust团队正专注于第二种用例，因此我们也将着重介绍它。对于第一种用例，可以参阅&nbsp;<code><a href="https://github.com/DenisKolodin/yew" class="external" rel=" noopener">yew</a></code>&nbsp;这类项目。</p>
            <p>在本教程中，我们将使用Rust的npm包构建工具<code>wasm-pack</code>来构建一个npm包。这个包只包含WebAssembly和JavaScript代码，以便包的用户无需安装Rust就能使用。他们甚至不需要知道这里包含WebAssembly！</p></div>
        </div>
        </article>
        </html>
        "###;

        let expected = r###"
        bad html
        <% a = 1 %>
        {% hello = a %}
        <!DOCTYPE html>
        <!-- html 的注释 -->
        <html xmlns=http://www.w3.org/1999/xhtml>
        <title><%= title %>或者{{ title }}</title>
        <article>
        <h1>编译 Rust 为 WebAssembly</h1>
        <style type="text/css" nofollow>
        /* 在 css 里面的注释会转换 */
        .body { font-size: 14px; } /* 后面个 comment 注释 */
        </style>
        <script type="text/javascript">
        // 这个 script 也会转换
        // 按照 javascript 的方式来处理
        const a = "hello 你好";
        /**
         * 多行 comment 测试
         * 多行第 2 行
         */
        </script>
        <script async src=https://dlswbr.baidu.com/heicha/mw/abclite-2036-s.js></script>
        <div class="content">
            <p>如果你写了一些 Rust 代码，你可以把它编译成 WebAssembly！这份教程将带你编译 Rust 项目为 wasm 并在一个现存的 web 应用中使用它。</p>
            <a href="#rust_和_webassembly_用例" title="Permalink to Rust 和 WebAssembly 用例">Rust 和 WebAssembly 用例</a>
            <h2>Rust 和 WebAssembly 用例</h2>
            <div @click.prevent="hello" :name="foo" #bar="dar"><p>Rust 和 WebAssembly 有两大主要用例：</p>
            <ul>
            <li>构建完整应用——整个 Web 应用都基于 Rust 开发！</li>
            <li>构建应用的组成部分——在现存的 JavaScript 前端中使用 Rust。</li>
            <%= link_to "FTP管理", "/", class: "subnav-item #{(params[:title_tab].blank? || params[:title_tab] == 'sftp_index') ? 'active' : ''}" %>
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
        <style type="text/css">
        /* 在css里面的注释会转换 */
        </style>
        <script type="text/javascript">
        // 这个script也会转换
        const a = "hello你好";
        </script>
        </body>
        </html>"#;

        let lint_result = lint_html(html);
        assert_eq!("", lint_result.error);
        assert_eq!(7, lint_result.lines.len());
    }
}
