use std::collections::HashMap;

lazy_static! {
  pub static ref FILE_TYPES: HashMap<&'static str, &'static str> = map!(
    "html" => "html",
    "htm" => "html",
    "vue" => "html",
    "ejs" => "html",
    "html.erb" => "html",
    // yaml
    "yaml" => "yaml",
    "yml" => "yaml",
    // rust
    "rust" => "rust",
    "rs" => "rust",
    // sql
    "sql" => "sql",
    // ruby
    "ruby" => "ruby",
    "rb" => "ruby",
    "Gemfile" => "ruby",
    // crystal
    "crystal" => "ruby",
    "cr" => "ruby",
    // elixir
    "elixir" => "elixir",
    "ex" => "elixir",
    "exs" => "elixir",
    // javascript
    "js" => "javascript",
    "jsx" => "javascript",
    "javascript" => "javascript",
    "ts" => "javascript",
    "tsx" => "javascript",
    "typescript" => "javascript",
    "js.erb" => "javascript",
    // css
    "css" => "css",
    "scss" => "css",
    "sass" => "css",
    "less" => "css",
    // json
    "json" => "json",
    "json5" => "json",
    // go
    "go" => "go",
    // python
    "python" => "python",
    "py" => "python",
    // objective-c
    "objective_c" => "objective_c",
    "objective-c" => "objective_c",
    "m" => "objective_c",
    "h" => "objective_c",
    // strings for Cocoa
    "strings" => "strings",
    // csharp
    "csharp" => "csharp",
    "cs" => "csharp",
    // java
    "java" => "java",
    // scala
    "scala" => "scala",
    // swift
    "swift" => "swift",
    // kotlin
    "kotlin" => "kotlin",
    // php
    "php" => "php",
    // dart
    "dart" => "dart",
    // markdown
    "markdown" => "markdown",
    "md" => "markdown",
    // LaTeX
    "latex" => "latex",
    "tex" => "latex",
    // AsciiDoc
    "asciidoc" => "asciidoc",
    "adoc" => "asciidoc",
    // gettext
    "po" => "gettext",
    "pot" => "gettext",
    // conf
    "properties" => "conf",
    "conf" => "conf",
    "ini" => "conf",
    "cfg" => "conf",
    "toml" => "conf",
    // C or C++
    "cc" => "c",
    "cpp" => "c",
    "c" => "c",
    // plain
    "text" => "text",
    "plain" => "text",
    "txt" => "text"
  );
}

// dectermines file_type is support
pub fn match_filename(filename_or_ext: &str) -> &str {
    let ext = get_file_extension(filename_or_ext);
    if !is_support_type(ext.as_str()) {
        return "";
    }

    return FILE_TYPES[ext.as_str()];
}

// dectermines file_type is support
pub fn is_support_type(filename_or_ext: &str) -> bool {
    FILE_TYPES.contains_key(filename_or_ext)
}

// get file extension from filepath, return filename if not has exit
#[allow(clippy::comparison_chain)]
pub fn get_file_extension(filename: &str) -> String {
    let filename = filename.trim();
    if is_support_type(filename) {
        return String::from(filename);
    }

    let filename = filename.split('/').last().unwrap().to_string();
    let path_parts: Vec<&str> = filename.split('.').collect();
    let mut ext: String = path_parts.last().unwrap().to_string();

    let part_len = path_parts.len();
    if part_len > 2 {
        let double_ext = path_parts[(part_len - 2)..part_len].join(".");
        if is_support_type(double_ext.as_str()) {
            ext = double_ext
        }
    } else if part_len < 2 {
        ext = filename;
    }

    ext
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn is_is_support_type() {
        assert!(is_support_type("html"));
        assert!(is_support_type("htm"));
        assert!(is_support_type("html.erb"));
        assert!(is_support_type("rust"));
        assert!(is_support_type("rs"));

        assert!(!is_support_type("foo"));
        assert!(!is_support_type("index.html"));
        assert!(!is_support_type("gettext"));
        assert!(!is_support_type("gettext.po"));
        assert!(!is_support_type("gettext.pot"));
    }

    #[test]
    fn is_get_file_extension() {
        assert_eq!("text", get_file_extension("text"));
        assert_eq!("txt", get_file_extension("txt"));
        assert_eq!("rb", get_file_extension("/foo/bar/dar.rb"));
        assert_eq!("rb", get_file_extension("/foo/bar/aaa.dar.rb"));
        assert_eq!("html.erb", get_file_extension("/foo/bar/dar.html.erb"));
        assert_eq!("html.erb", get_file_extension("html.erb"));
        assert_eq!("Gemfile", get_file_extension("Gemfile"));
        assert_eq!("js", get_file_extension("/dar.js"));
        assert_eq!("dar", get_file_extension("/foo/bar/dar"));
    }
}
