use autocorrect::{format, format_html, format_yaml};
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
    "md" => "text"
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
    .get_matches();

  if let Some(file_name) = matches.value_of("text") {
    let path_exist = Path::new(file_name).exists();
    if !path_exist {
      let mut ext = "";
      if let Some(_type) = matches.value_of("type") {
        ext = _type;
      }
      format_and_output(ext, file_name);
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

      format_and_output(ext, raw.as_str());
    }
  }
}

fn format_and_output(ext: &str, raw: &str) {
  if EXT_MAPS.contains_key(ext) {
    match EXT_MAPS[ext] {
      "html" => {
        println!("{}", format_html(raw));
      }
      "yaml" => {
        println!("{}", format_yaml(raw));
      }
      "text" => {
        println!("{}", format(raw));
      }
      _ => {
        println!("{}", raw);
      }
    }
  } else {
    // else return raw
    println!("{}", raw);
  }
}

fn get_file_extension(filepath: &str) -> &str {
  let ext = Path::new(filepath)
    .extension()
    .and_then(OsStr::to_str)
    .unwrap();

  return ext;
}
