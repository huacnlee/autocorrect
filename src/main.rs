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
    .arg(
      Arg::with_name("lint").long("lint").help("Lint and output problems.")
    )
    .get_matches();

  let fix = matches.is_present("fix");
  // disable lint when fix mode
  let lint = matches.is_present("lint") && !fix;

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
            format_and_output(_path.to_str().unwrap(), fix, lint);
          }
          Err(_e) => {}
        }
      }
    }
  }
}

fn format_and_output(path: &str, fix: bool, lint: bool) {
  if let Ok(raw) = fs::read_to_string(path) {
    let raw = raw.as_str();
    let mut out = String::from(raw);

    let ext = get_file_extension(path);

    let ignore = is_ignore_auto_correct(raw);

    if EXT_MAPS.contains_key(ext) && !ignore {
      match EXT_MAPS[ext] {
        "html" => {
          out = html::format_html(raw, lint);
        }
        "yaml" => {
          out = yaml::format_yaml(raw, lint);
        }
        "sql" => {
          out = sql::format_sql(raw, lint);
        }
        "rust" => {
          out = rust::format_rust(raw, lint);
        }
        "ruby" => {
          out = ruby::format_ruby(raw, lint);
        }
        "go" => {
          out = go::format_go(raw, lint);
        }
        "javascript" => {
          out = javascript::format_javascript(raw, lint);
        }
        "css" => {
          out = css::format_css(raw, lint);
        }
        "json" => {
          out = json::format_json(raw, lint);
        }
        "python" => {
          out = python::format_python(raw, lint);
        }
        "objective_c" => {
          out = objective_c::format_objective_c(raw, lint);
        }
        "csharp" => {
          out = csharp::format_csharp(raw, lint);
        }
        "swift" => {
          out = swift::format_swift(raw, lint);
        }
        "java" => {
          out = java::format_java(raw, lint);
        }
        "kotlin" => {
          out = kotlin::format_kotlin(raw, lint);
        }
        "php" => {
          out = php::format_php(raw, lint);
        }
        "dart" => {
          out = dart::format_dart(raw, lint);
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
