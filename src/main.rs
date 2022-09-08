mod cli;
mod format;
mod helper;

use clap::Parser;
use cli::*;
use format::*;

fn main() {
    let cli = Cli::parse();

    let v = cli.verbose;

    // TODO check if provided file is a file

    let output_name = match cli.output {
        Some(name) => name,
        None => format!(
            "{}.{}",
            match cli.file.file_stem() {
                None => panic!("No file name!"),
                // WARNING will crash if file name containts invalid Unicode, fix somehow idk
                // does this even need fixing?
                stem => stem.unwrap().to_str().unwrap().to_string(),
            },
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
