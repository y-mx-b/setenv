mod cli;
mod env;
mod format;

use clap::Parser;
use cli::*;
use env::Env;
use env_logger;
use log::{debug, info};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> std::io::Result<ExitCode> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    // exit if input is not a file
    if !cli.file.is_file() {
        eprintln!("Input file is not a file.");
        return Ok(ExitCode::FAILURE);
    }

    // initialize relevant information
    let output_path = match cli.output {
        Some(file_name) => PathBuf::from(file_name),
        None => cli.file.with_extension(cli.format.to_string()),
    };

    // vprint useful information
    info!("Input File: {:?}", cli.file);
    info!("Output File: {:?}", output_path);
    info!("Format: {}", cli.format);

    // TODO do this safely, better error messages
    // decode env file from TOML
    let toml_contents = fs::read_to_string(cli.file)?;
    debug!("TOML:\n{}", toml_contents);
    let data: Env = toml::from_str(&toml_contents)?;
    debug!("Contents:\n{:?}", data);

    // convert input TOML to shell script
    let output = data.to_string(cli.format);

    // create new output file, clear if already exists
    let mut output_file = fs::File::create(output_path)?;

    // write output to file
    // TODO do it more safely
    output_file.write_all(output.as_bytes())?;
    Ok(ExitCode::SUCCESS)
}
