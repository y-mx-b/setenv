mod cli;
mod format;
mod helper;

use clap::Parser;
use cli::*;
use format::*;

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

    // convert input TOML to shell script
    match cli.format {
        Format::Sh => {}
        _ => {}
    }
}
