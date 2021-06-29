use autocorrect::{
  format, format_go, format_html, format_javascript, format_ruby, format_rust, format_sql,
  format_yaml,
};
use clap::{crate_version, App, Arg};
use glob::glob;
use std::collections::HashMap;
use std::ffi::OsStr;
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
    "yaml" => "yaml",
    "yml" => "yaml",
    "plain" => "text",
    "text" => "text",
    "txt" => "text",
    "markdown" => "text",
    "md" => "text",
    "rust" => "rust",
    "rs" => "rust",
    "sql" => "sql",
    "ruby" => "ruby",
    "rb" => "ruby",
    "js" => "javascript",
    "jsx" => "javascript",
    "javascript" => "javascript",
    "go" => "go"
  );
}

pub fn main() {
  let matches = App::new("AutoCorrect")
    .author("Jason Lee <huacnlee@gmail.com")
    .version(crate_version!())
    .about("Automatically add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).")
    .arg(
      Arg::with_name("text").help("Target filepath or string (Plain text) for format").takes_value(true).required(false)
    )
    .arg(
      Arg::with_name("type").long("type").short("t").help("File content type [text, html, yaml], default detect with file extension.").takes_value(true)
    )
    .arg(
      Arg::with_name("auto_correct_all").long("auto-correct-all").short("A").help("Auto-correct and rewrite file.")
    )
    .get_matches();

  let auto_correct_all = matches.is_present("auto_correct_all");
  if let Some(file_name) = matches.value_of("text") {
    let path_exist = Path::new(file_name).exists();
    if !path_exist {
      let mut ext = "";
      if let Some(_type) = matches.value_of("type") {
        ext = _type;
      }
      format_and_output("", false, ext, file_name);
      return;
    }

    for f in glob(file_name).unwrap() {
      let path: String;
      match f {
        Ok(_path) => path = String::from(_path.to_str().unwrap()),
        Err(_e) => break,
      }
      let raw = fs::read_to_string(path.as_str()).unwrap();
      let mut ext = get_file_extension(path.as_str());
      if let Some(_type) = matches.value_of("type") {
        ext = _type;
      }

      format_and_output(path.as_str(), auto_correct_all, ext, raw.as_str());
    }
  }
}

fn format_and_output(path: &str, auto_correct_all: bool, ext: &str, raw: &str) {
  let mut out = String::from(raw);
  if EXT_MAPS.contains_key(ext) {
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
      "text" => {
        out = format(raw);
      }
      _ => {}
    }
  }

  if auto_correct_all && path.len() > 0 {
    fs::write(Path::new(path), out).unwrap();
  } else {
    println!("{}", out);
  }
}

fn get_file_extension(filepath: &str) -> &str {
  let ext = Path::new(filepath)
    .extension()
    .and_then(OsStr::to_str)
    .unwrap();

  return ext;
}
