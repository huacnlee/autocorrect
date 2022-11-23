// autocorrect: false
use clap::Parser;
use initializer::InitOption;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::SystemTime;
use time::SystemTimeDuration;

mod cli;
mod initializer;
mod progress;
mod time;
mod update;

use cli::Cli;
use colored::*;
use threadpool::ThreadPool;

extern crate autocorrect;

include!(concat!(env!("OUT_DIR"), "/config_template.rs"));

static DEFAULT_CONFIG_FILE: &str = ".autocorrectrc";

pub fn load_config(config_file: &str) -> Result<(), autocorrect::config::Error> {
    autocorrect::config::load_file(config_file)?;

    Ok(())
}

fn init_logger(level: tracing::Level) {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .with_level(false)
        .with_line_number(false)
        .without_time()
        .with_max_level(level)
        .init();
}

pub fn main() {
    let mut cli = Cli::parse();

    // Set log level
    let log_level = if cli.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    init_logger(log_level);

    if cli.threads == 0 {
        cli.threads = num_cpus::get();
    }
    tracing::debug!("Threads: {}", cli.threads);

    match cli.command {
        Some(cli::Commands::Init { local, force }) => {
            initializer::run(&cli, &InitOption { force, local });
            return;
        }
        Some(cli::Commands::Update {}) => {
            match update::run() {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("{}", e);
                    std::process::exit(1);
                }
            }
            return;
        }
        _ => {}
    }

    tracing::debug!("Load config: {}", cli.config_file);
    load_config(&cli.config_file).unwrap_or_else(|e| {
        panic!("Load config error: {}", e);
    });

    let mut arg_files = cli.files.clone().into_iter();

    // calc run time
    let start_t = SystemTime::now();
    let mut lint_results: Vec<String> = Vec::new();
    let lint_errors_count = std::sync::Arc::new(std::sync::Mutex::new(0));
    let lint_warnings_count = std::sync::Arc::new(std::sync::Mutex::new(0));

    if cli.stdin {
        let mut _err_count = 0;
        let mut _warn_count = 0;

        let raw = io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .join("\n");

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
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    let path_str = path.to_str().unwrap_or("");

                    if ignorer.is_ignored(path_str) {
                        // skip ignore file
                        continue;
                    }

                    // ignore unless file
                    if !path.is_file() {
                        continue;
                    }

                    // println!("{}", path.display());

                    let filepath = String::from(path_str);
                    let mut filetype = autocorrect::get_file_extension(&filepath);
                    if let Some(ref ftype) = cli.filetype {
                        filetype = ftype.clone();
                    }
                    if !autocorrect::is_support_type(&filetype) {
                        continue;
                    }

                    let cli = cli.clone();
                    let tx = tx.clone();
                    let lint_errors_count = lint_errors_count.clone();
                    let lint_warnings_count = lint_warnings_count.clone();
                    let filepath = filepath.clone();
                    let filetype = filetype.clone();

                    pool.execute(move || {
                        if let Ok(raw) = read_file(&filepath) {
                            let t = SystemTime::now();
                            tracing::debug!("Process {}", filepath);
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

                            tracing::debug!("Done {} {}ms\n", filepath, t.elapsed_millis());
                        }
                    });
                }
                Err(_err) => {
                    tracing::error!("ERROR: {}", _err);
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
    }

    tracing::debug!("\n\nLint result found: {} issues.", lint_results.len());

    if cli.lint {
        if cli.formatter == "json" {
            tracing::info!(
                r#"{{"count": {},"messages": [{}]}}"#,
                lint_results.len(),
                lint_results.join(",")
            );
        } else {
            tracing::info!("\n");

            let _err_count = *lint_errors_count.lock().unwrap();
            let _warn_count = *lint_warnings_count.lock().unwrap();

            tracing::info!(
                "{}, {}\n",
                format!("Error: {}", _err_count).red(),
                format!("Warning: {}", _warn_count).yellow(),
            );

            if !lint_results.is_empty() {
                // diff will use stderr output
                tracing::info!("{}", lint_results.join("\n"));
            }

            // print time spend from start_t to now
            tracing::info!("AutoCorrect spend time {}ms\n", start_t.elapsed_millis());

            if _err_count > 0 {
                // Exit with code = 1
                std::process::exit(1);
            }
        }
    } else if cli.fix {
        tracing::info!("\n");

        // print time spend from start_t to now
        tracing::info!("AutoCorrect spend time: {}ms\n", start_t.elapsed_millis());
    }
}

fn read_file(filepath: &str) -> io::Result<String> {
    let t = SystemTime::now();
    tracing::debug!("Loading {} ...", filepath);

    let out = fs::read_to_string(filepath);

    tracing::debug!("Loaded {} {}ms", filepath, t.elapsed_millis());

    out
}

fn format_and_output(filepath: &str, filetype: &str, raw: &str, cli: &Cli) {
    let result = autocorrect::format_for(raw, filetype);

    if cli.fix && !filepath.is_empty() {
        if result.has_error() {
            tracing::debug!("{}\n{}", filepath, result.error);
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
            tracing::error!("{}", raw);
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
    let diff_mode = cli.formatter != "json";
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
            tracing::debug!("{}\n{}", filepath, result.error);
            return;
        }

        results.push(result.to_diff());
    } else {
        results.push(result.to_json());
    }
}
