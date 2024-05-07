use clap::{Parser, Subcommand, ValueEnum};

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum OutputFormatter {
    Diff,
    Json,
    Rdjson,
}

impl OutputFormatter {
    pub fn is_diff(&self) -> bool {
        *self == OutputFormatter::Diff
    }
}

#[derive(Debug, Parser, Clone)]
#[command(name = "AutoCorrect")]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[clap(long, action = clap::ArgAction::SetTrue, help = "Lint and output problems.")]
    pub lint: bool,

    #[clap(long, help = "Automatically fix problems and rewrite file.")]
    pub fix: bool,

    #[clap(long, help = "Print debug information.")]
    pub debug: bool,

    #[clap(long, short = 'q', help = "Do not print progress information.")]
    pub quiet: bool,

    #[clap(
        name = "FORMAT",
        long = "format",
        help = "Output format.",
        default_value = "diff"
    )]
    #[arg(value_enum)]
    pub formatter: OutputFormatter,

    #[clap(
        long = "threads",
        help = "Number of threads, 0 - use number of CPU.",
        default_value = "0"
    )]
    pub threads: usize,

    #[clap(
        name = "CONFIG",
        short = 'c',
        long = "config",
        help = "Special config file.",
        default_value = crate::DEFAULT_CONFIG_FILE
    )]
    pub config_file: String,

    #[clap(long = "type", help = "Directly use set file type.")]
    pub filetype: Option<String>,

    #[clap(
        name = "FILE",
        help = "Target filepath or dir for format.",
        default_value = ".",
        value_parser
    )]
    pub files: Vec<String>,

    #[clap(long = "stdin", help = "Input text from <STDIN>")]
    pub stdin: bool,

    #[clap(
        long = "no-diff-bg-color",
        alias = "ndbc",
        help = "Disable diff background color for diff output."
    )]
    pub no_diff_bg_color: bool,
}

#[derive(Debug, Subcommand, Clone)]
pub(crate) enum Commands {
    #[command(name = "init", about = "Initialize AutoCorrect config file.")]
    Init {
        #[clap(
            long,
            help = "Use the built-in config file for without remote download."
        )]
        local: bool,

        #[clap(
            short = 'f',
            long,
            help = "Override if the config file already exists."
        )]
        force: bool,
    },
    #[cfg(feature = "update")]
    #[command(
        name = "update",
        alias = "upgrade",
        about = "Update AutoCorrect to latest version."
    )]
    Update {},
}

impl Cli {
    pub fn log_level(&self) -> log::LevelFilter {
        if self.debug && !self.quiet {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        }
    }
}
