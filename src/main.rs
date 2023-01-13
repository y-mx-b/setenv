mod cli;
mod env;
mod format;
mod helper;

use clap::Parser;
use cli::*;
use env::Env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    // exit if input is not a file
    if !cli.file.is_file() {
        eprintln!("Input file is not a file.");
        return ExitCode::FAILURE;
    }

    // initialize relevant information
    let v = cli.verbose;
    let output_path = match cli.output {
        Some(file_name) => PathBuf::from(file_name),
        None => cli.file.with_extension(cli.format.to_string()),
    };

    // vprint useful information
    vprintln!(v, "Input File: {:?}", cli.file);
    vprintln!(v, "Output File: {:?}", output_path);
    vprintln!(v, "Format: {}", cli.format);

    // TODO do this safely, better error messages
    // decode env file from TOML
    let toml_contents = fs::read_to_string(cli.file).expect("Failed to read file");
    let data: Env = toml::from_str(&toml_contents).unwrap();
    vprintln!(v, "Contents:\n{:?}", data);

    // convert input TOML to shell script
    let output = data.to_string(cli.format);

    // create new output file, clear if already exists
    let mut output_file = fs::File::create(output_path).unwrap();

    // write output to file
    // TODO do it more safely
    output_file.write_all(output.as_bytes()).unwrap();

    ExitCode::SUCCESS
}
