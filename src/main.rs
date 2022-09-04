mod cli;

use cli::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

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
                file.file_name().unwrap().to_str().unwrap().to_string()
            }
    };
    println!("output name: {}", name);
}
