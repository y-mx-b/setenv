use clap::{ValueEnum, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// TOML file containing environment variables and definitions
    #[clap(value_parser)]
    pub file: PathBuf,

    /// The name of the output file
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// The format to use
    #[clap(default_value_t = Format::Sh)]
    #[clap(short, long, arg_enum, value_parser)]
    pub format: Format,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "CONFIG")]
    pub config: Option<PathBuf>,

    /// Print extra information
    #[clap(short, long, action)]
    pub verbose: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Format {
    Sh,
    Bash,
    Zsh,
    Tcsh,
    Fish,
}