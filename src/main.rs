// autocorrect: false
use autocorrect::{
  format, format_csharp, format_css, format_dart, format_go, format_html, format_java,
  format_javascript, format_json, format_kotlin, format_objective_c, format_php, format_python,
  format_ruby, format_rust, format_sql, format_swift, format_yaml, get_file_extension,
  is_ignore_auto_correct,
};
use clap::{crate_version, App, Arg};
use glob::glob;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
    "markdown" => "text",
    "md" => "text",
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
      Arg::with_name("fix").long("fix").help("Automatically fix problems and rewrite file.")
    )
    .get_matches();

  let fix = matches.is_present("fix");
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
            format_and_output(_path.to_str().unwrap(), fix);
          }
          Err(_e) => {}
        }
      }
    }
  }
}

fn format_and_output(path: &str, fix: bool) {
  if let Ok(raw) = fs::read_to_string(path) {
    let raw = raw.as_str();
    let mut out = String::from(raw);

    let ext = get_file_extension(path);

    let ignore = is_ignore_auto_correct(raw);

    if EXT_MAPS.contains_key(ext) && !ignore {
      match EXT_MAPS[ext] {
        "html" => {
          out = format_html(raw);
        }
        "yaml" => {
          out = format_yaml(raw);
        }
        "sql" => {
          out = format_sql(raw);
        }
        "rust" => {
          out = format_rust(raw);
        }
        "ruby" => {
          out = format_ruby(raw);
        }
        "go" => {
          out = format_go(raw);
        }
        "javascript" => {
          out = format_javascript(raw);
        }
        "css" => {
          out = format_css(raw);
        }
        "json" => {
          out = format_json(raw);
        }
        "python" => {
          out = format_python(raw);
        }
        "objective_c" => {
          out = format_objective_c(raw);
        }
        "csharp" => {
          out = format_csharp(raw);
        }
        "swift" => {
          out = format_swift(raw);
        }
        "java" => {
          out = format_java(raw);
        }
        "kotlin" => {
          out = format_kotlin(raw);
        }
        "php" => {
          out = format_php(raw);
        }
        "dart" => {
          out = format_dart(raw);
        }
        "text" => {
          out = format(raw);
        }
        _ => {}
      }
    }

    if fix {
      // do not rewrite ignored file
      if !ignore && path.len() > 0 {
        fs::write(Path::new(path), out).unwrap();
      }
    } else {
      println!("{}", out);
      // only print once
      std::process::exit(0);
    }
  }
}
