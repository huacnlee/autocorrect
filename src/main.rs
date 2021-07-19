// autocorrect: false
use autocorrect::{format, is_ignore_auto_correct};
use clap::{crate_version, App, Arg};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

mod code;
mod csharp;
mod css;
mod dart;
mod go;
mod html;
mod java;
mod javascript;
mod json;
mod kotlin;
mod markdown;
mod objective_c;
mod php;
mod python;
mod ruby;
mod rust;
mod sql;
mod strings;
mod swift;
mod yaml;

#[macro_use]
extern crate lazy_static;

macro_rules! map {
  {$($key:expr => $value:expr),+} => {{
      let mut m = HashMap::new();
      $(
          m.insert($key, $value);
      )+
      m
  }};
}

lazy_static! {
  static ref FILE_TYPES: HashMap<&'static str, &'static str> = map!(
    "html" => "html",
    "htm" => "html",
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
    // crystal
    "cr" => "ruby",
    "crystal" => "ruby",
    // javascript
    "js" => "javascript",
    "jsx" => "javascript",
    "javascript" => "javascript",
    "ts" => "javascript",
    "tsx" => "javascript",
    "typescript" => "javascript",
    // css
    "css" => "css",
    "scss" => "css",
    "sass" => "css",
    "less" => "css",
    // json
    "json" => "json",
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
    // swift
    "swift" => "swift",
    // kotlin
    "kotlin" => "kotlin",
    // php
    "php" => "php",
    // dart
    "dart" => "dart",
    // text
    "plain" => "text",
    "txt" => "text",
    // markdown
    "markdown" => "markdown",
    "md" => "markdown",
    // plain
    "text" => "text"
  );
}

pub fn main() {
    let matches = App::new("AutoCorrect")
    .author("Jason Lee <huacnlee@gmail.com")
    .version(crate_version!())
    .about("Automatically add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).")
    .arg(
      Arg::with_name("file").help("Target filepath or dir for format").takes_value(true).required(true)
    )
    .arg(
      Arg::with_name("fix").long("fix").help("Automatically fix problems and rewrite file.").required(false)
    )
    .arg(
      Arg::with_name("lint").long("lint").help("Lint and output problems.")
    )
    .arg(
        Arg::with_name("filetype").long("type").help("Directly use set file type").default_value("").required(false)
      )
    .arg(
        Arg::with_name("formatter").long("format").help("Choose an output formatter.").default_value("diff").possible_values(&["json", "diff"]).required(false)
    )
    .arg(
        Arg::with_name("debug").long("debug").help("Print debug message.")
    )
    .get_matches();

    let fix = matches.is_present("fix");
    // disable lint when fix mode
    let lint = matches.is_present("lint") && !fix;
    #[allow(unused_variables)]
    let debug = matches.is_present("debug");
    let formatter = matches.value_of("formatter").unwrap_or("").to_lowercase();
    let arg_file = matches.value_of("file").unwrap_or("");
    let arg_filetype = matches.value_of("filetype").unwrap();

    // calc run time
    let start_t = std::time::SystemTime::now();

    let mut walker = ignore::WalkBuilder::new(arg_file);
    walker.follow_links(false);

    if let Some(_err) = walker.add_ignore(Path::new(".autocorrectignore")) {
        // ignore not exist
    }

    let mut lint_results: Vec<String> = Vec::new();

    for result in walker.build() {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // ignore unless file
                if !path.is_file() {
                    continue;
                }

                if lint {
                    print!(".")
                }

                let filepath = path.to_str().unwrap();
                let mut filetype = get_file_extension(path);
                if arg_filetype != "" {
                    filetype = arg_filetype;
                }
                if !FILE_TYPES.contains_key(filetype) {
                    continue;
                }

                if let Ok(raw) = fs::read_to_string(filepath) {
                    if lint {
                        lint_and_output(
                            filepath,
                            filetype,
                            raw.as_str(),
                            formatter.as_str(),
                            &mut lint_results,
                        )
                    } else {
                        format_and_output(filepath, filetype, raw.as_str(), fix);
                    }
                }
            }
            Err(_err) => {
                println!("ERROR: {}", _err);
            }
        }
    }

    println!("\n\n-----------------------------------------");

    if lint {
        if formatter == "json" {
            println!(
                r#"{{"count": {},"messages": [{}]}}"#,
                lint_results.len(),
                lint_results.join(",")
            );
        } else {
            // print time spend from start_t to now
            println!(
                "AutoCorrect spend time: {}ms\n",
                start_t.elapsed().unwrap().as_millis()
            );

            if lint_results.len() > 0 {
                // diff will use stderr output
                eprint!("{}", lint_results.join("\n"));
                std::process::exit(1);
            }
        }
    }
}

fn format_and_output(filepath: &str, filetype: &str, raw: &str, fix: bool) {
    let ignore = is_ignore_auto_correct(raw);

    // print raw content and exist when ignore enable and not fix
    if ignore && !fix {
        println!("{}", raw);
        std::process::exit(0);
    }

    let result = match FILE_TYPES[filetype] {
        "html" => html::format_html(raw),
        "yaml" => yaml::format_yaml(raw),
        "sql" => sql::format_sql(raw),
        "rust" => rust::format_rust(raw),
        "ruby" => ruby::format_ruby(raw),
        "go" => go::format_go(raw),
        "javascript" => javascript::format_javascript(raw),
        "css" => css::format_css(raw),
        "json" => json::format_json(raw),
        "python" => python::format_python(raw),
        "objective_c" => objective_c::format_objective_c(raw),
        "strings" => strings::format_strings(raw),
        "csharp" => csharp::format_csharp(raw),
        "swift" => swift::format_swift(raw),
        "java" => java::format_java(raw),
        "kotlin" => kotlin::format_kotlin(raw),
        "php" => php::format_php(raw),
        "dart" => dart::format_dart(raw),
        "markdown" => markdown::format_markdown(raw),
        "text" => markdown::format_markdown(raw),
        _ => code::FormatResult::new(raw),
    };

    if fix {
        // do not rewrite ignored file
        if filepath.len() > 0 {
            fs::write(Path::new(filepath), result.out).unwrap();
        }
    } else {
        // print a single file output
        println!("{}", result.out);
    }
}

fn lint_and_output(
    filepath: &str,
    filetype: &str,
    raw: &str,
    formatter: &str,
    results: &mut Vec<String>,
) {
    let ignore = is_ignore_auto_correct(raw);

    // skip lint ignored file, just return
    if ignore {
        return;
    }

    let mut result = match FILE_TYPES[filetype] {
        "html" => html::lint_html(raw),
        "yaml" => yaml::lint_yaml(raw),
        "sql" => sql::lint_sql(raw),
        "rust" => rust::lint_rust(raw),
        "ruby" => ruby::lint_ruby(raw),
        "go" => go::lint_go(raw),
        "javascript" => javascript::lint_javascript(raw),
        "css" => css::lint_css(raw),
        "json" => json::lint_json(raw),
        "python" => python::lint_python(raw),
        "objective_c" => objective_c::lint_objective_c(raw),
        "strings" => strings::lint_strings(raw),
        "csharp" => csharp::lint_csharp(raw),
        "swift" => swift::lint_swift(raw),
        "java" => java::lint_java(raw),
        "kotlin" => kotlin::lint_kotlin(raw),
        "php" => php::lint_php(raw),
        "dart" => dart::lint_dart(raw),
        "markdown" => markdown::lint_markdown(raw),
        "text" => markdown::lint_markdown(raw),
        _ => code::LintResult::new(raw),
    };

    // do not print anything, when not lint results
    if result.lines.len() == 0 {
        return;
    }

    result.filepath = String::from(filepath);

    if formatter == "json" {
        results.push(format!("{}", result.to_json()));
    } else {
        // diff will use stderr output
        results.push(format!("{}", result.to_diff()));
    }
}

// get file extension from filepath
fn get_file_extension(path: &Path) -> &str {
    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
        return ext;
    }

    return "";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_get_file_extension() {
        assert_eq!("rb", get_file_extension("/foo/bar/dar.rb"));
        assert_eq!("js", get_file_extension("/dar.js"));
        assert_eq!("", get_file_extension("/foo/bar/dar"));
    }
}
