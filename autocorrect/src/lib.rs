// autocorrect: false
/*!
Automatically add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).

## Features

- Auto add spacings between CJK (Chinese, Japanese, Korean) and English words.
- HTML content support.

## Example

Use `autocorrect::format` to format plain text.

```rust
extern crate autocorrect;

fn main() {
    println!("{}", autocorrect::format("长桥 LongBridge App 下载"));
    // => "长桥 LongBridge App 下载"

    println!("{}", autocorrect::format("Ruby 2.7 版本第 1 次发布"));
    // => "Ruby 2.7 版本第 1 次发布"

    println!("{}", autocorrect::format("于 3 月 10 日开始"));
    // => "于 3 月 10 日开始"

    println!("{}", autocorrect::format("包装日期为2013年3月10日"));
    // => "包装日期为2013年3月10日"

    println!("{}", autocorrect::format("全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。"));
    // => "全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。"

    println!("{}", autocorrect::format("既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"));
    // => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"

    println!("{}", autocorrect::format("전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고， 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."));
    // => "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고， 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."
}
```
*/

#[macro_use]
extern crate lazy_static;

macro_rules! regexp {
    ($($arg:tt)*) => {{
        let reg_str = format!($($arg)*);

        let mut rule_str = String::from(reg_str).replace(
            r"\p{CJK}",
            r"\p{Han}|\p{Hangul}|\p{Hanunoo}|\p{Katakana}|\p{Hiragana}|\p{Bopomofo}",
        );

         rule_str = String::from(rule_str).replace(
            r"\p{CJK_N}",
            r"\p{Han}\p{Hangul}\p{Hanunoo}\p{Katakana}\p{Hiragana}\p{Bopomofo}",
        );

        rule_str = String::from(rule_str).replace(
            r"\p{CJ}",
            r"\p{Han}|\p{Katakana}|\p{Hiragana}|\p{Bopomofo}",
        );

        rule_str = String::from(rule_str).replace(
            r"\p{CJ_N}",
            r"\p{Han}\p{Katakana}\p{Hiragana}\p{Bopomofo}",
        );

        // println!("{}", rule_str);
        let res = regex::Regex::new(&rule_str).unwrap();
        res
    }};
}

#[macro_export]
macro_rules! map {
    {$($key:expr => $value:expr),+ $(,)?} => {{
        let mut m = HashMap::new();
        $(
            m.insert($key, $value);
        )+
        m
    }};
    () => (
        HashMap::new()
    );
}

mod code;
mod diff;
mod format;
mod result;
mod rule;
mod serde_any;

pub mod config;
pub mod ignorer;

pub use code::{format_for, get_file_extension, is_support_type, lint_for};
pub use config::Config;
pub use format::*;
pub use result::{FormatResult, LineResult, LintResult};
pub use rule::{halfwidth, spellcheck};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!("Hello 世界。", format("Hello世界."));
    }

    #[test]
    fn test_format_for() {
        assert_eq!("Hello 世界。", format_for("Hello世界.", "text").out);
    }
}
