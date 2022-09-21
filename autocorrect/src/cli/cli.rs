use clap::{Parser, Subcommand};

#[derive(Debug, Parser, Clone)]
#[clap(
    name = "AutoCorrect",
    author = "Jason Lee <huacnlee@gmail.com>",
    version
)]
#[clap(about = "A linter and formatter for help you improve copywriting, to correct spaces, punctuations between CJK (Chinese, Japanese, Korean).", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    #[clap(long, parse(from_flag), help = "Lint and output problems.")]
    pub lint: bool,

    #[clap(long, help = "Automatically fix problems and rewrite file.")]
    pub fix: bool,

    #[clap(long, help = "Print debug information.")]
    pub debug: bool,

    #[clap(
        name = "FORMAT",
        long = "format",
        help = "Output format.",
        default_value = "diff",
        possible_values = &["diff", "json"]
    )]
    pub formatter: String,

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
        value_parser,
        multiple = true
    )]
    pub files: Vec<String>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    #[clap(name = "init", about = "Initialize AutoCorrect config file.")]
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
    #[clap(
        name = "update",
        alias = "upgrade",
        about = "Update AutoCorrect to latest version."
    )]
    Update {},
}
