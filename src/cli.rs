use crate::format::*;
use clap::Parser;
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
    #[clap(short, long, value_enum, value_parser)]
    pub format: Format,

    /// Print extra information
    #[clap(short, long, action)]
    pub verbose: bool,
}
