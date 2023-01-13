use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Env {
    #[serde(alias = "env-vars")]
    env_vars: HashMap<String, Vec<String>>,
}

impl Env {
    pub fn to_sh(self) -> String {
        let mut output: Vec<String> = Vec::new();
        for (var, value) in self.env_vars {
            // POSIX sh doesn't have arrays, this separator is for PATH variables
            output.push(format!("{}=\"{}\"; export {}", var, value.join(":"), var));
        }
        output.join("\n")
    }

    pub fn to_fish(self) -> String {
        let mut output: Vec<String> = Vec::new();
        for (var, mut value) in self.env_vars {
            // surround each item in `value` in double quotes
            for item in &mut value[..] {
                item.insert(0, '"');
                item.push('"');
            }
            // separate with spaces for arrays
            output.push(format!("set -gx {} {}", var, value.join(" ")))
        }
        output.join("\n")
    }

    pub fn to_tcsh(self) -> String {
        let mut output: Vec<String> = Vec::new();
        for (var, mut value) in self.env_vars {
            // surround each item in `value` in double quotes
            for item in &mut value[..] {
                item.insert(0, '"');
                item.push('"');
            }

            // separate with spaces for arrays
            output.push(format!("setenv {}={}", var, value.join(" ")))
        }
        output.join("\n")
    }
}
