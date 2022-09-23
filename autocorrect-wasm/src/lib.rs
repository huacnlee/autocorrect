extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod config;
pub mod ignorer;

/// Automatically add spaces between Chinese and English words.
///
/// This method only work for plain text.
///
/// # Example
///
/// ```
/// extern crate autocorrect;
///
/// println!("{}", autocorrect::format("学习如何用 Rust 构建 Application"));
/// // => "学习如何用 Rust 构建 Application"
///
/// println!("{}", autocorrect::format("于 3 月 10 日开始"));
/// // => "于 3 月 10 日开始"
///
/// println!("{}", autocorrect::format("既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"));
/// // => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"
/// ```
#[wasm_bindgen]
pub fn format(text: &str) -> String {
    autocorrect::format(text)
}

/// Format content with filetype, and return a json result.
#[wasm_bindgen(js_name = "formatFor")]
pub fn format_for(raw: &str, filename_or_ext: &str) -> wasm_bindgen::JsValue {
    let result = autocorrect::format_for(raw, filename_or_ext);
    wasm_bindgen::JsValue::from_serde(&result).unwrap()
}

/// Lint content with filetype, and return a json result.
#[wasm_bindgen(js_name = "lintFor")]
pub fn lint_for(raw: &str, filename_or_ext: &str) -> wasm_bindgen::JsValue {
    let result = autocorrect::lint_for(raw, filename_or_ext);
    wasm_bindgen::JsValue::from_serde(&result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // autocorrect: false

    #[test]
    fn test_format() {
        assert_eq!(
            "学习如何用 Rust 构建 Application",
            format("学习如何用Rust构建Application")
        );
    }
}
