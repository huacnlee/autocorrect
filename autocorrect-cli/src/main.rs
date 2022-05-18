// autocorrect: false
use clap::{crate_version, App, Arg};
use initializer::InitOption;
use std::fs;
use std::path::Path;

mod initializer;
mod logger;
mod progress;
mod update;

use logger::Logger;
use threadpool::ThreadPool;

extern crate autocorrect;

include!(concat!(env!("OUT_DIR"), "/config_template.rs"));

#[derive(Clone)]
pub struct CliOption {
    lint: bool,
    fix: bool,
    debug: bool,
    formatter: String,
    threads: usize,
    config_file: String,
}

static DEFAULT_CONFIG_FILE: &str = ".autocorrectrc";

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    return App::new("AutoCorrect")
    .author("Jason Lee <huacnlee@gmail.com")
    .version(crate_version!())
    .about("A linter and formatter for help you improve copywriting, to correct spaces, punctuations between CJK (Chinese, Japanese, Korean).")
    .arg(
      Arg::with_name("file").help("Target filepath or dir for format.").takes_value(true).default_value(".").multiple(true)
    )
    .arg(
      Arg::with_name("fix").long("fix").help("Automatically fix problems and rewrite file.")
    )
    .arg(
      Arg::with_name("lint").long("lint").help("Lint and output problems.")
    )
    .arg(
        Arg::with_name("filetype").long("type").help("Directly use set file type.")
      )
    .arg(
        Arg::with_name("formatter").long("format").help("Choose an output formatter.").default_value("diff").possible_values(&["json", "diff"])
    )
    .arg(
        Arg::with_name("debug").long("debug").help("Print debug message.")
    )
    .arg(
        Arg::with_name("config").long("config").short("c").help("Special config file.").default_value(DEFAULT_CONFIG_FILE)
    )
    .arg(
        Arg::with_name("threads").long("threads").help("Number of threads, 0 - use number of CPU.").default_value("0")
    )
    .subcommand(
        App::new("init")
        .arg(Arg::with_name("remote").long("remote").help("Use GitHub remote config template."))
        .arg(Arg::with_name("force").long("force").short("f").help("Override config if it exist."))
        .about("Init AutoCorrect config file.")
    )
    .subcommand(
        App::new("upgrade").alias("update")
        .about("Upgrade AutoCorrect to latest version.")
    )
    .get_matches();
}

pub fn load_config(config_file: &str) -> Result<(), autocorrect::config::Error> {
    autocorrect::config::load_file(config_file)?;

    Ok(())
}

pub fn main() {
    let mut option = CliOption {
        debug: false,
        fix: false,
        lint: false,
        formatter: String::from(""),
        threads: 0,
        config_file: String::from(""),
    };

    let matches = get_matches();
    Logger::init().expect("Init logger error");

    option.fix = matches.is_present("fix");
    // disable lint when fix mode
    option.lint = matches.is_present("lint") && !option.fix;
    option.debug = matches.is_present("debug");
    let formatter = matches.value_of("formatter").unwrap_or("").to_lowercase();
    option.formatter = formatter;
    option.threads = matches
        .value_of("threads")
        .unwrap_or("0")
        .parse::<usize>()
        .unwrap_or(0);
    option.config_file = matches
        .value_of("config")
        .unwrap_or(DEFAULT_CONFIG_FILE)
        .to_string();

    if let Some(sub_matches) = matches.subcommand_matches("init") {
        let init_option = InitOption {
            remote: sub_matches.is_present("remote"),
            force: sub_matches.is_present("force"),
        };

        initializer::run(&option, &init_option);
        return;
    }

    if let Some(_sub_matches) = matches.subcommand_matches("upgrade") {
        match update::run() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    if option.debug {
        println!("Load config: {}", option.config_file);
    }
    load_config(&option.config_file).unwrap_or_else(|e| {
        panic!("Load config error: {}", e);
    });

    if option.threads == 0 {
        option.threads = num_cpus::get();
    }

    let mut arg_files = matches.values_of("file").unwrap();
    let arg_filetype = matches.value_of("filetype").unwrap_or("");

    // calc run time
    let start_t = std::time::SystemTime::now();
    let mut lint_results: Vec<String> = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();

    let pool = ThreadPool::new(option.threads);
    // let mut threads = Vec::new();

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
    let ignorer = autocorrect::ignorer::Ignorer::new("./");

    for result in walker.build() {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if ignorer.is_ignored(path.to_str().unwrap()) {
                    // skip ignore file
                    continue;
                }

                // ignore unless file
                if !path.is_file() {
                    continue;
                }

                // println!("{}", path.display());

                let filepath = String::from(path.to_str().unwrap());
                let mut filetype = autocorrect::get_file_extension(filepath.as_str());
                if !arg_filetype.is_empty() {
                    filetype = String::from(arg_filetype);
                }
                if !autocorrect::is_support_type(filetype.as_str()) {
                    continue;
                }

                let tx = tx.clone();
                let option = option.clone();
                let filepath = filepath.clone();
                let filetype = filetype.clone();

                pool.execute(move || {
                    if let Ok(raw) = fs::read_to_string(&filepath) {
                        let file_start_t = std::time::SystemTime::now();

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

                        if option.debug {
                            log::info!(
                                "{} {}ms\n",
                                filepath,
                                file_start_t.elapsed().unwrap().as_millis()
                            );
                        }
                    }
                });
            }
            Err(_err) => {
                log::error!("ERROR: {}", _err);
            }
        }
    }
    // wait all threads complete
    // println!("\n---- threads {}", threads.len());
    pool.join();

    // wait all threads send result
    while let Ok(lint_result) = rx.try_recv() {
        lint_results.push(lint_result)
    }

    if option.debug {
        println!("\n\nLint result found: {} issues.", lint_results.len());
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

            if !lint_results.is_empty() {
                // diff will use stderr output
                log::error!("{}", lint_results.join("\n"));
            }

            // print time spend from start_t to now
            log::info!(
                "AutoCorrect spend time {}ms\n",
                start_t.elapsed().unwrap().as_millis()
            );

            if !lint_results.is_empty() {
                std::process::exit(1);
            }
        }
    } else if option.fix {
        log::info!("Done.\n");

        // print time spend from start_t to now
        log::info!(
            "AutoCorrect spend time: {}ms\n",
            start_t.elapsed().unwrap().as_millis()
        );
    }
}

fn format_and_output(filepath: &str, filetype: &str, raw: &str, option: &CliOption) {
    let result = autocorrect::format_for(raw, filetype);

    if option.fix {
        if result.has_error() {
            if option.debug {
                log::error!("{}\n{}", filepath, result.error);
            }
            return;
        }

        // do not rewrite ignored file
        if !filepath.is_empty() {
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
    option: &CliOption,
    results: &mut Vec<String>,
) {
    let diff_mode = option.formatter != "json";
    let mut result = autocorrect::lint_for(raw, filetype);
    result.filepath = String::from(filepath);

    // do not print anything, when not lint results
    if result.lines.is_empty() {
        progress::ok(diff_mode);
        return;
    } else {
        progress::err(diff_mode);
    }

    if diff_mode {
        if result.has_error() && option.debug {
            log::error!("{}\n{}", filepath, result.error);
            return;
        }

        results.push(result.to_diff());
    } else {
        results.push(result.to_json());
    }
}
