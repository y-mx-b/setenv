mod cli;
mod env;
mod format;

use anyhow::{Context, Result};
use clap::Parser;
use cli::*;
use env::Env;
use env_logger;
use log::{debug, info};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    // initialize relevant information
    let output_path = match cli.output {
        Some(file_name) => PathBuf::from(file_name),
        None => cli.file.with_extension(cli.format.to_string()),
    };

    // vprint useful information
    info!("Input File: {:?}", cli.file);
    info!("Output File: {:?}", output_path);
    info!("Format: {}", cli.format);

    let toml_contents = fs::read_to_string(&cli.file)
        .with_context(|| format!("Failed to read from file `{}`", cli.file.display()))?;
    debug!("TOML:\n{}", toml_contents);
    let data: Env = toml::from_str(&toml_contents)?;
    debug!("Data:\n{:?}", data);

    // convert input TOML to shell script
    let output = data.to_string(cli.format);

    // create new output file, clear if already exists
    let mut output_file = fs::File::create(output_path)?;

    // write output to file
    output_file.write_all(output.as_bytes())?;
    Ok(ExitCode::SUCCESS)
}
