mod cli;
mod helper;

use clap::Parser;
use cli::*;

fn main() {
    let cli = Cli::parse();

    let v = cli.verbose;
    let output_name = match cli.output {
        Some(name) => name,
        // WARNING will crash if file name containts invalid Unicode
        // TODO replace file extension
        None => cli.file.file_name().unwrap().to_str().unwrap().to_string(),
    };

    vprintln!(v, "File name: {}", output_name);

    vprintln!(v, "Format: {:?}", cli.format);

    match cli.format {
        Format::Sh => {}
        _ => {}
    }
}
