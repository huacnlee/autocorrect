// autocorrect: false
use autocorrect::format;
use clap::{crate_version, App, Arg};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

mod logger;
mod progress;

use logger::Logger;

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

static AUTOCORRECTIGNORE: &str = ".autocorrectignore";

lazy_static! {
  static ref FILE_TYPES: HashMap<&'static str, &'static str> = map!(
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
    "js.erb" => "javascript",
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

#[derive(Clone)]
struct Option {
    lint: bool,
    fix: bool,
    debug: bool,
    formatter: String,
}

pub fn main() {
    let mut option = Option {
        debug: false,
        fix: false,
        lint: false,
        formatter: String::from(""),
    };

    let matches = App::new("AutoCorrect")
    .author("Jason Lee <huacnlee@gmail.com")
    .version(crate_version!())
    .about("A linter and formatter for help you improve copywriting, to correct spaces, punctuations between CJK (Chinese, Japanese, Korean).")
    .arg(
      Arg::with_name("file").help("Target filepath or dir for format").takes_value(true).required(true).multiple(true)
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

    Logger::init().expect("Init logger error");

    let work_dir: std::path::PathBuf = std::env::current_dir().expect("");
    let autocorrect_path = work_dir.join(Path::new(AUTOCORRECTIGNORE));

    option.fix = matches.is_present("fix");
    // disable lint when fix mode
    option.lint = matches.is_present("lint") && !option.fix;
    option.debug = matches.is_present("debug");
    let formatter = matches.value_of("formatter").unwrap_or("").to_lowercase();
    option.formatter = formatter;

    let mut arg_files = matches.values_of("file").unwrap();
    let arg_filetype = matches.value_of("filetype").unwrap();

    // calc run time
    let start_t = std::time::SystemTime::now();
    let mut lint_results: Vec<String> = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut threads = Vec::new();

    // create a walker
    // take first file arg, because ignore::WalkBuilder::new need a file path.
    let first_file = arg_files.next().expect("Not file args");
    let mut walker = ignore::WalkBuilder::new(Path::new(first_file));
    // Add other files
    for arg_file in arg_files {
        walker.add(arg_file);
    }
    walker
        .skip_stdout(true)
        .parents(true)
        .git_ignore(true)
        .follow_links(false);

    // create ignorer for ignore directly file
    let mut ignore_builder = ignore::gitignore::GitignoreBuilder::new("./");
    if let Some(path) = autocorrect_path.to_str() {
        if let Some(err) = ignore_builder.add(Path::new(path)) {
            if option.debug {
                println!("Fail to add ignore file: {}, {}", path, err);
            }
        }
    }
    let ignorer = ignore_builder.build().unwrap();

    for result in walker.build() {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if ignorer.matched_path_or_any_parents(path, false).is_ignore()
                    || ignorer.matched_path_or_any_parents(path, true).is_ignore()
                {
                    // skip ignore file
                    continue;
                }

                // ignore unless file
                if !path.is_file() {
                    continue;
                }

                // println!("{}", path.display());

                let filepath = String::from(path.to_str().unwrap());
                let mut filetype = get_file_extension(path);
                if arg_filetype != "" {
                    filetype = String::from(arg_filetype);
                }
                if !FILE_TYPES.contains_key(filetype.as_str()) {
                    continue;
                }

                let tx = tx.clone();
                let option = option.clone();
                let filepath = filepath.clone();
                let filetype = filetype.clone();

                let thread = std::thread::spawn(move || {
                    if let Ok(raw) = fs::read_to_string(&filepath) {
                        if option.lint {
                            let mut lint_results: Vec<String> = Vec::new();
                            lint_and_output(
                                filepath.as_str(),
                                filetype.as_str(),
                                raw.as_str(),
                                &option,
                                &mut lint_results,
                            );

                            for lint_result in lint_results {
                                tx.send(lint_result).unwrap();
                            }
                        } else {
                            format_and_output(
                                filepath.as_str(),
                                filetype.as_str(),
                                raw.as_str(),
                                &option,
                            );
                        }
                    }
                });
                threads.push(thread);
            }
            Err(_err) => {
                log::error!("ERROR: {}", _err);
            }
        }
    }

    // wait all threads send result
    loop {
        match rx.try_recv() {
            Ok(lint_result) => lint_results.push(lint_result),
            Err(_) => break,
        }
    }

    // wait all threads complete
    for th in threads {
        th.join().unwrap();
    }

    if option.lint {
        if option.formatter == "json" {
            log::info!(
                r#"{{"count": {},"messages": [{}]}}"#,
                lint_results.len(),
                lint_results.join(",")
            );
        } else {
            log::info!("\n");

            if lint_results.len() > 0 {
                // diff will use stderr output
                log::error!("{}", lint_results.join("\n"));
            }

            // print time spend from start_t to now
            log::info!(
                "AutoCorrect spend time: {}ms\n",
                start_t.elapsed().unwrap().as_millis()
            );

            if lint_results.len() > 0 {
                std::process::exit(1);
            }
        }
    } else {
        if option.fix {
            log::info!("Done.\n");

            // print time spend from start_t to now
            log::info!(
                "AutoCorrect spend time: {}ms\n",
                start_t.elapsed().unwrap().as_millis()
            );
        }
    }
}

fn format_and_output(filepath: &str, filetype: &str, raw: &str, option: &Option) {
    // if option.debug {
    //     log::info!("-> {}", filepath);
    // }

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

    if option.fix {
        if result.has_error() {
            if option.debug {
                log::error!("{}\n{}", filepath, result.error);
            }
            return;
        }

        // do not rewrite ignored file
        if filepath.len() > 0 {
            if result.out.eq(&String::from(raw)) {
                progress::ok(true);
            } else {
                progress::err(true);
            }

            fs::write(Path::new(filepath), result.out).unwrap();
        }
    } else {
        if result.has_error() {
            println!("{}", raw);
            return;
        }

        // print a single file output
        println!("{}", result.out);
    }
}

fn lint_and_output(
    filepath: &str,
    filetype: &str,
    raw: &str,
    option: &Option,
    results: &mut Vec<String>,
) {
    let diff_mode = option.formatter != "json";

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
        progress::ok(diff_mode);
        return;
    } else {
        progress::err(diff_mode);
    }

    result.filepath = String::from(filepath);

    if diff_mode {
        if result.has_error() {
            if option.debug {
                log::error!("{}\n{}", filepath, result.error);
                return;
            }
        }

        results.push(format!("{}", result.to_diff()));
    } else {
        results.push(format!("{}", result.to_json()));
    }
}

// get file extension from filepath
fn get_file_extension(path: &Path) -> String {
    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
        return String::from(ext);
    }

    return String::from("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_get_file_extension() {
        assert_eq!("rb", get_file_extension("/foo/bar/dar.rb"));
        assert_eq!("html.erb", get_file_extension("/foo/bar/dar.html.erb"));
        assert_eq!("js", get_file_extension("/dar.js"));
        assert_eq!("", get_file_extension("/foo/bar/dar"));
    }
}
