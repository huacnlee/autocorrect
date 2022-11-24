mod code;
mod types;

mod asciidoc;
mod c;
mod conf;
mod csharp;
mod css;
mod dart;
mod elixir;
mod gettext;
mod go;
mod html;
mod java;
mod javascript;
mod json;
mod kotlin;
mod latex;
mod markdown;
mod objective_c;
mod php;
mod python;
mod ruby;
mod rust;
mod scala;
mod sql;
mod strings;
mod swift;
mod xml;
mod yaml;

pub use code::*;
pub use types::*;

pub use asciidoc::*;
pub use c::*;
pub use conf::*;
pub use csharp::*;
pub use css::*;
pub use dart::*;
pub use elixir::*;
pub use gettext::*;
pub use go::*;
pub use html::*;
pub use java::*;
pub use javascript::*;
pub use json::*;
pub use kotlin::*;
pub use latex::*;
pub use markdown::*;
pub use objective_c::*;
pub use php::*;
pub use python::*;
pub use ruby::*;
pub use rust::*;
pub use scala::*;
pub use sql::*;
pub use strings::*;
pub use swift::*;
pub use xml::*;
pub use yaml::*;

/// Lint a file content with filetype.
///
/// Example:
///
/// ```
//  extern crate autocorrect;
//
/// let raw = r#"
/// <article>
///   <h1>这是 Heading 标题</h1>
///   <div class="content">
///     <p>你好 Rust 世界<strong>Bold 文本</strong></p>
///     <p>这是第二行 p 标签</p>
///   </div>
/// </article>
/// "#;
/// autocorrect::lint_for(raw, "html");
/// autocorrect::lint_for(raw, "index.html");
/// ```
pub fn lint_for(raw: &str, filename_or_ext: &str) -> LintResult {
    let mut result = match types::match_filename(filename_or_ext).as_str() {
        "html" => lint_html(raw),
        "yaml" => lint_yaml(raw),
        "sql" => lint_sql(raw),
        "rust" => lint_rust(raw),
        "ruby" => lint_ruby(raw),
        "elixir" => lint_elixir(raw),
        "go" => lint_go(raw),
        "javascript" => lint_javascript(raw),
        "css" => lint_css(raw),
        "json" => lint_json(raw),
        "python" => lint_python(raw),
        "objective_c" => lint_objectivec(raw),
        "strings" => lint_strings(raw),
        "csharp" => lint_csharp(raw),
        "swift" => lint_swift(raw),
        "java" => lint_java(raw),
        "scala" => lint_scala(raw),
        "kotlin" => lint_kotlin(raw),
        "php" => lint_php(raw),
        "dart" => lint_dart(raw),
        "markdown" => lint_markdown(raw),
        "latex" => lint_latex(raw),
        "asciidoc" => lint_asciidoc(raw),
        "gettext" => lint_gettext(raw),
        "conf" => lint_conf(raw),
        "c" => lint_c(raw),
        "xml" => lint_xml(raw),
        "text" => lint_markdown(raw),
        _ => LintResult::new(raw),
    };

    result.filepath = String::from(filename_or_ext);

    result
}

/// Format a file content with filetype.
///
/// Example:
///
/// ```
//  extern crate autocorrect;
//
/// let raw = r#"
/// <article>
///   <h1>这是 Heading 标题</h1>
///   <div class="content">
///     <p>你好 Rust 世界<strong>Bold 文本</strong></p>
///     <p>这是第二行 p 标签</p>
///   </div>
/// </article>
/// "#;
/// autocorrect::format_for(raw, "html");
/// autocorrect::format_for(raw, "index.html");
/// ```
pub fn format_for(raw: &str, filename_or_ext: &str) -> FormatResult {
    let result = match types::match_filename(filename_or_ext).as_str() {
        "html" => format_html(raw),
        "yaml" => format_yaml(raw),
        "sql" => format_sql(raw),
        "rust" => format_rust(raw),
        "ruby" => format_ruby(raw),
        "elixir" => format_elixir(raw),
        "go" => format_go(raw),
        "javascript" => format_javascript(raw),
        "css" => format_css(raw),
        "json" => format_json(raw),
        "python" => format_python(raw),
        "objective_c" => format_objectivec(raw),
        "strings" => format_strings(raw),
        "csharp" => format_csharp(raw),
        "swift" => format_swift(raw),
        "java" => format_java(raw),
        "scala" => format_scala(raw),
        "kotlin" => format_kotlin(raw),
        "php" => format_php(raw),
        "dart" => format_dart(raw),
        "markdown" => format_markdown(raw),
        "latex" => format_latex(raw),
        "asciidoc" => format_asciidoc(raw),
        "gettext" => format_gettext(raw),
        "conf" => format_conf(raw),
        "c" => format_c(raw),
        "xml" => format_xml(raw),
        "text" => format_markdown(raw),
        _ => {
            let mut result = FormatResult::new(raw);
            result.out = String::from(raw);
            result
        }
    };

    result
}
