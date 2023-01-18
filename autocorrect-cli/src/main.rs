// autocorrect: false
use clap::Parser;
use initializer::InitOption;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::SystemTime;

mod cli;
mod initializer;
mod logger;
mod progress;
mod update;

use cli::Cli;
use logger::Logger;
use logger::SystemTimeDuration;
use owo_colors::OwoColorize;
use threadpool::ThreadPool;

extern crate autocorrect;

include!(concat!(env!("OUT_DIR"), "/config_template.rs"));

static DEFAULT_CONFIG_FILE: &str = ".autocorrectrc";

macro_rules! bench {
    ($name: expr, $block: block) => {
        let start = SystemTime::now();
        $block;
        log::debug!("{} {}ms", $name, start.elapsed_millis());
    };
    () => {};
}

pub fn main() {
    let mut cli = Cli::parse();

    // Set log level
    let log_level = cli.log_level();
    Logger::init(log_level).expect("Init logger error");

    if cli.threads == 0 {
        cli.threads = num_cpus::get();
    }
    log::debug!("Threads: {}", cli.threads);

    match cli.command {
        Some(cli::Commands::Init { local, force }) => {
            initializer::run(&cli, &InitOption { force, local });
            return;
        }
        Some(cli::Commands::Update {}) => {
            match update::run() {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e);
                    std::process::exit(1);
                }
            }
            return;
        }
        _ => {}
    }

    load_config(&cli.config_file);

    let mut arg_files = cli.files.iter();

    // calc run time
    let start_t = SystemTime::now();

    let mut lint_results: Vec<String> = Vec::new();
    let lint_errors_count = std::sync::Arc::new(std::sync::Mutex::new(0));
    let lint_warnings_count = std::sync::Arc::new(std::sync::Mutex::new(0));

    if cli.stdin {
        let mut _err_count = 0;
        let mut _warn_count = 0;

        let raw = read_stdin();

        if cli.lint {
            lint_and_output(
                "<STDIN>",
                "text",
                &raw,
                &cli,
                &mut lint_results,
                &mut _err_count,
                &mut _warn_count,
            );
        } else {
            format_and_output("", "text", &raw, &cli)
        }

        *lint_errors_count.lock().unwrap() += _err_count;
        *lint_warnings_count.lock().unwrap() += _warn_count;
    } else {
        let (tx, rx) = std::sync::mpsc::channel();

        let pool = ThreadPool::new(cli.threads);

        // create a walker
        // take first file arg, because ignore::WalkBuilder::new need a file path.
        let first_file = arg_files.next().expect("Not file args");
        let mut walker = ignore::WalkBuilder::new(Path::new(&first_file));
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
            if let Err(err) = result {
                log::error!("ERROR: {}", err);
                continue;
            }

            let entry = result.unwrap();
            let path = entry.path();
            let filepath = path.to_str().unwrap_or("");

            if ignorer.is_ignored(filepath) {
                // skip ignore file
                continue;
            }

            // ignore unless file
            if !path.is_file() {
                continue;
            }

            let mut filetype = autocorrect::get_file_extension(filepath);
            if let Some(ref ftype) = cli.filetype {
                filetype = ftype.to_owned();
            }
            if !autocorrect::is_support_type(&filetype) {
                continue;
            }

            let cli = cli.clone();
            let tx = tx.clone();
            let lint_errors_count = lint_errors_count.clone();
            let lint_warnings_count = lint_warnings_count.clone();
            let filepath = filepath.to_owned();
            let filetype = filetype.clone();

            pool.execute(move || match read_file(&filepath) {
                Ok(raw) => {
                    bench!(format!("Done {}", filepath), {
                        if cli.lint {
                            let mut lint_results: Vec<String> = Vec::new();

                            let mut _err_count = 0;
                            let mut _warn_count = 0;
                            lint_and_output(
                                &filepath,
                                &filetype,
                                &raw,
                                &cli,
                                &mut lint_results,
                                &mut _err_count,
                                &mut _warn_count,
                            );

                            *lint_errors_count.lock().unwrap() += _err_count;
                            *lint_warnings_count.lock().unwrap() += _warn_count;

                            for lint_result in lint_results {
                                tx.send(lint_result).unwrap();
                            }
                        } else {
                            format_and_output(&filepath, &filetype, &raw, &cli);
                        }
                    });
                }
                Err(err) => {
                    log::error!("Failed to read: {} error: {}", filepath, err);
                }
            });
        }
        // wait all threads complete
        pool.join();

        // wait all threads send result
        while let Ok(lint_result) = rx.try_recv() {
            lint_results.push(lint_result)
        }
    }

    log::debug!("Lint result found: {} issues.", lint_results.len());

    if cli.lint {
        if cli.formatter.is_json() {
            log::info!(
                r#"{{"count": {},"messages": [{}]}}"#,
                lint_results.len(),
                lint_results.join(",")
            );
        } else {
            let _err_count = *lint_errors_count.lock().unwrap();
            let _warn_count = *lint_warnings_count.lock().unwrap();

            log::info!("\n");
            log::info!("{}", lint_results.join(""));

            log::info!(
                "{}, {}\n",
                format!("Error: {}", _err_count).red(),
                format!("Warning: {}", _warn_count).yellow(),
            );

            // print time spend from start_t to now
            log::info!("AutoCorrect spend time {}ms\n", start_t.elapsed_millis());

            if _err_count > 0 {
                // Exit with code = 1
                std::process::exit(1);
            }
        }
    } else if cli.fix {
        log::info!("\n");

        // print time spend from start_t to now
        log::info!("AutoCorrect spend time: {}ms\n", start_t.elapsed_millis());
    }
}

#[inline]
fn read_file(filepath: &str) -> io::Result<String> {
    let out;

    bench!(format!("Loaded {}", filepath), {
        out = fs::read_to_string(filepath);
    });

    out
}

/// Read stdin into a string
#[inline]
fn read_stdin() -> String {
    io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .join("\n")
}

#[inline]
pub fn load_config(filename: &str) {
    log::debug!("Load config: {}", filename);

    autocorrect::config::load_file(filename).unwrap_or_else(|e| {
        panic!("Load config file: {}\nerror: {}", filename, e);
    });
}

fn format_and_output(filepath: &str, filetype: &str, raw: &str, cli: &Cli) {
    let result = autocorrect::format_for(raw, filetype);

    if cli.fix && !filepath.is_empty() {
        if result.has_error() {
            log::debug!("{}\n{}", filepath, result.error);
            return;
        }

        // do not rewrite ignored file
        if !filepath.is_empty() {
            if result.out.eq(&String::from(raw)) {
                progress::ok(!cli.debug);
            } else {
                progress::err(!cli.debug);
            }

            fs::write(Path::new(filepath), result.out).unwrap();
        }
    } else {
        if result.has_error() {
            log::error!("{}", raw);
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
    cli: &Cli,
    results: &mut Vec<String>,
    errors_count: &mut usize,
    warings_count: &mut usize,
) {
    let diff_mode = cli.formatter.is_diff();
    let mut result = autocorrect::lint_for(raw, filetype);
    result.filepath = String::from(filepath);

    *errors_count += result.errors_count();
    *warings_count += result.warnings_count();

    // do not print anything, when not lint results
    if !cli.debug {
        if result.lines.is_empty() {
            progress::ok(diff_mode);
            return;
        }

        if *errors_count > 0 {
            progress::err(diff_mode);
        } else if *warings_count > 0 {
            progress::warn(diff_mode);
        }
    }

    if diff_mode {
        if result.has_error() {
            log::debug!("{}\n{}", filepath, result.error);
            return;
        }

        results.push(result.to_diff(cli.no_diff_bg_color));
    } else {
        results.push(result.to_json());
    }
}
