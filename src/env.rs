use serde::Deserialize;
use std::collections::HashMap;
use crate::format::Format;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Env {
    #[serde(alias = "env-vars")]
    env_vars: HashMap<String, Vec<String>>,
    #[serde(alias = "vi-mode")]
    vi_mode: bool,
    // TODO add aliases
}

impl Env {
    pub fn to_string(self, format: Format) -> String {
        let mut output: Vec<String> = Vec::new();

        match format {
            Format::Sh => {
                for (v, value) in self.env_vars {
                    let var = v.to_uppercase(); // env vars uppercase by convention

                    // POSIX sh doesn't have arrays, : is separator by convention
                    output.push(format!("{}=\"${}:{}\"; export {}", var, var, value.join(":"), var));
                }

                if self.vi_mode {
                    output.push("set -o vi".to_string())
                }
            }
            Format::Fish => {
                for (v, mut value) in self.env_vars {
                    let var = v.to_uppercase();

                    for item in &mut value[..] {
                        item.insert(0, '"');
                        item.push('"');
                    }

                    // separate with spaces for arrays
                    output.push(format!("set -gx ${} {}", var, value.join(" ")));
                }

                if self.vi_mode {
                    output.push("fish_vi_key_bindings".to_string());
                }
            }
            Format::Tcsh => {
                for (v, mut value) in self.env_vars {
                    let var = v.to_uppercase();

                    for item in &mut value[..] {
                        item.insert(0, '"');
                        item.push('"');
                    }

                    output.push(format!("setenv {} = (${} {})", var, var, value.join(" ")));
                }

                if self.vi_mode {
                    output.push("bindkey -v".to_string());
                }
            }
        }

        output.join("\n")
    }

    // TODO extract repeated prepocessing to make it more general
    pub fn to_sh(self) -> String {
        let mut output: Vec<String> = Vec::new();

        output.push("#!/bin/sh".to_string());

        for (v, value) in self.env_vars {
            let var = v.to_uppercase(); // env vars are uppercase by convention

            // POSIX sh doesn't have arrays, this separator is for PATH variables
            output.push(format!("{}=\"${}:{}\"; export {}", var, var, value.join(":"), var));
        }
        if self.vi_mode {
            output.push("set -o vi".to_string())
        }

        output.join("\n")
    }

    pub fn to_fish(self) -> String {
        let mut output: Vec<String> = Vec::new();

        // surround each item in `value` in double quotes
        for (v, mut value) in self.env_vars {
            let var = v.to_uppercase(); // uppercase by convention

            for item in &mut value[..] {
                item.insert(0, '"');
                item.push('"');
            }

            // separate with spaces for arrays
            output.push(format!("set -gx {} {}", var, value.join(" ")));
        }

        if self.vi_mode {
            output.push("fish_vi_key_bindings".to_string());
        }

        output.join("\n")
    }

    pub fn to_tcsh(self) -> String {
        let mut output: Vec<String> = Vec::new();

        for (v, mut value) in self.env_vars {
            let var = v.to_uppercase(); // uppercase by convention

            // surround each item in `value` in double quotes
            for item in &mut value[..] {
                item.insert(0, '"');
                item.push('"');
            }

            // separate with spaces for arrays
            output.push(format!("setenv {} = ({})", var, value.join(" ")))
        }

        if self.vi_mode {
            output.push("bindkey -v".to_string());
        }

        output.join("\n")
    }
}
