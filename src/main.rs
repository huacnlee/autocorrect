// autocorrect: false
use autocorrect::{format, get_file_extension, is_ignore_auto_correct};
use clap::{crate_version, App, Arg};
use glob::glob;
use std::collections::HashMap;
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
  static ref EXT_MAPS: HashMap<&'static str, &'static str> = map!(
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
      Arg::with_name("file").help("Target filepath or dir for format").takes_value(true).required(false).multiple(true)
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
    .get_matches();

    let fix = matches.is_present("fix");
    // disable lint when fix mode
    let lint = matches.is_present("lint") && !fix;
    let formatter = matches.value_of("formatter").unwrap();
    let arg_filetype = matches.value_of("filetype").unwrap();

    if let Some(file_names) = matches.values_of("file") {
        for file_name in file_names {
            let filepath = Path::new(file_name);
            let mut file_name = String::from(file_name);

            if !filepath.is_file() {
                file_name.push_str("/**/*");
            }

            file_name = file_name.replace("//", "/");

            for f in glob(file_name.as_str()).unwrap() {
                match f {
                    Ok(_path) => {
                        let filepath = _path.to_str().unwrap();
                        let mut filetype = get_file_extension(filepath);
                        if arg_filetype != "" {
                            filetype = arg_filetype;
                        }

                        if !EXT_MAPS.contains_key(filetype) {
                            continue;
                        }

                        if let Ok(raw) = fs::read_to_string(filepath) {
                            if lint {
                                lint_and_output(filepath, filetype, raw.as_str(), formatter)
                            } else {
                                format_and_output(filepath, filetype, raw.as_str(), fix);
                            }
                        }
                    }
                    Err(_e) => {}
                }
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

    let result = match EXT_MAPS[filetype] {
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

fn lint_and_output(filepath: &str, filetype: &str, raw: &str, formatter: &str) {
    let ignore = is_ignore_auto_correct(raw);

    // skip lint ignored file, just return
    if ignore {
        return;
    }

    let mut result = match EXT_MAPS[filetype] {
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

    if formatter.to_lowercase() == "json" {
        println!("{}", result.to_json());
    } else {
        // diff will use stderr output
        eprintln!("{}", result.to_diff());
    }
}
