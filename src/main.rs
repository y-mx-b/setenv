mod cli;
mod helper;

use cli::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let v = cli.verbose;
    let name = match cli {
        Cli {
            output: Some(name),
            ..
        } => {
                name
            },
        Cli {
            file,
            output: None,
            ..
        } => {
                // WARNING will crash if file name containts invalid Unicode
                file.file_name().unwrap().to_str().unwrap().to_string()
            }
    };
    vprintln!(v, "File name: {}", name);
}
