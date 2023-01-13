mod cli;
mod env;
mod format;
mod helper;

use clap::Parser;
use cli::*;
use format::*;
use std::fs;
use env::Env;
use std::io::Write;

fn main() {
    let cli = Cli::parse();

    // exit if input is not a file
    if !cli.file.is_file() {
        eprintln!("Input file is not a file.");
        return;
    }

    // initialize relevant information
    let v = cli.verbose;
    let output_name = match cli.output {
        Some(name) => name,
        None => format!(
            "{}.{}",
            match cli.file.file_stem() {
                None => panic!("No file name!"),
                // WARNING will crash if file name contains invalid Unicode, fix somehow idk
                // does this even need fixing?
                stem => stem.unwrap().to_str().unwrap().to_string(),
            },
            cli.format.to_string()
        ),
    };

    // vprint useful information
    vprintln!(v, "Input File: {:?}", cli.file);
    vprintln!(v, "Output File: {}", output_name);
    vprintln!(v, "Format: {}", cli.format);

    // TODO do this safely, better error messages
    // decode env file from TOML
    let toml_contents = fs::read_to_string(cli.file).expect("Failed to read file");
    let data: Env = toml::from_str(&toml_contents).unwrap();
    vprintln!(v, "Contents:\n{:?}", data);

    // convert input TOML to shell script
    let output = match cli.format {
        Format::Sh => data.to_sh(),
        Format::Fish => data.to_fish(),
        Format::Tcsh => data.to_tcsh(),
    };
    vprintln!(v, "Output:\n{}", output);

    // write output to file
    // TODO do it more safely
    let mut output_file = fs::File::create(output_name).unwrap();
    output_file.write_all(output.as_bytes()).unwrap();
}
