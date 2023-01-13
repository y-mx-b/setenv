use clap::ValueEnum;
use std::fmt;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Format {
    Sh,
    Tcsh,
    Fish,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Format::Sh => "sh",
                Format::Tcsh => "tcsh",
                Format::Fish => "fish",
            }
        )
    }
}
