// autocorrect: false
use clap::Parser;
use initializer::InitOption;
use std::fs;
use std::path::Path;

mod cli;
mod initializer;
mod logger;
mod progress;
mod update;

use cli::Cli;
use logger::Logger;
use threadpool::ThreadPool;

extern crate autocorrect;

include!(concat!(env!("OUT_DIR"), "/config_template.rs"));

static DEFAULT_CONFIG_FILE: &str = ".autocorrectrc";

pub fn load_config(config_file: &str) -> Result<(), autocorrect::config::Error> {
    autocorrect::config::load_file(config_file)?;

    Ok(())
}

pub fn main() {
    let mut cli = Cli::parse();

    // Set log level
    let log_level = if cli.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
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

    log::debug!("Load config: {}", cli.config_file);
    load_config(&cli.config_file).unwrap_or_else(|e| {
        panic!("Load config error: {}", e);
    });

    let mut arg_files = cli.files.clone().into_iter();

    // calc run time
    let start_t = std::time::SystemTime::now();
    let mut lint_results: Vec<String> = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();

    let pool = ThreadPool::new(cli.threads);
    // let mut threads = Vec::new();

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
                if cli.filetype.is_some() {
                    filetype = cli.filetype.clone().unwrap();
                }
                if !autocorrect::is_support_type(filetype.as_str()) {
                    continue;
                }

                let tx = tx.clone();
                let cli = cli.clone();
                let filepath = filepath.clone();
                let filetype = filetype.clone();

                pool.execute(move || {
                    if let Ok(raw) = fs::read_to_string(&filepath) {
                        let file_start_t = std::time::SystemTime::now();

                        if cli.lint {
                            let mut lint_results: Vec<String> = Vec::new();
                            lint_and_output(
                                filepath.as_str(),
                                filetype.as_str(),
                                raw.as_str(),
                                &cli,
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
                                &cli,
                            );
                        }

                        if cli.debug {
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

    log::debug!("\n\nLint result found: {} issues.", lint_results.len());

    if cli.lint {
        if cli.formatter == "json" {
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
    } else if cli.fix {
        log::info!("Done.\n");

        // print time spend from start_t to now
        log::info!(
            "AutoCorrect spend time: {}ms\n",
            start_t.elapsed().unwrap().as_millis()
        );
    }
}

fn format_and_output(filepath: &str, filetype: &str, raw: &str, cli: &Cli) {
    let result = autocorrect::format_for(raw, filetype);

    if cli.fix {
        if result.has_error() {
            log::debug!("{}\n{}", filepath, result.error);
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
    cli: &Cli,
    results: &mut Vec<String>,
) {
    let diff_mode = cli.formatter != "json";
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
        if result.has_error() {
            log::debug!("{}\n{}", filepath, result.error);
            return;
        }

        results.push(result.to_diff());
    } else {
        results.push(result.to_json());
    }
}
