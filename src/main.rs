mod cli;
mod format;
mod helper;

use clap::Parser;
use cli::*;
use format::*;

fn main() {
    let cli = Cli::parse();

    let v = cli.verbose;
    let output_name = match cli.output {
        Some(name) => name,
        // WARNING will crash if file name containts invalid Unicode
        None => format!(
            "{}.{}",
            cli.file.file_stem().unwrap().to_str().unwrap().to_string(),
            cli.format.to_string()
        ),
    };

    vprintln!(v, "File name: {}", output_name);

    vprintln!(v, "Format: {}", cli.format);

    match cli.format {
        Format::Sh => {}
        _ => {}
    }
}
