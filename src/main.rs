use autocorrect::{format, format_html};
use clap::{crate_version, App, Arg};
use glob::glob;
use std::fs;
use std::path::Path;

pub fn main() {
  let matches = App::new("AutoCorrect")
    .author("Jason Lee <huacnlee@gmail.com")
    .version(crate_version!())
    .about("Automatically add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).")
    .arg(
      Arg::with_name("text").help("Target filepath or string (Plain text) for format").takes_value(true).required(false)
    )
    .arg(
      Arg::with_name("html").long("html").help("Use for HTML format")
    )
    .get_matches();

  if let Some(file_name) = matches.value_of("text") {
    let path_exist = Path::new(file_name).exists();
    if path_exist {
      for f in glob(file_name).unwrap() {
        let path: String;
        match f {
          Ok(_path) => path = String::from(_path.to_str().unwrap()),
          Err(_e) => break,
        }
        let raw = fs::read_to_string(path).unwrap();
        let raw = raw.as_str();
        if matches.is_present("html") {
          println!("{}", format(raw));
        } else {
          println!("{}", format_html(raw));
        }
      }
    } else {
      let raw = file_name;
      if matches.is_present("html") {
        println!("{}", format(raw));
      } else {
        println!("{}", format_html(raw));
      }
    }
  }
}
