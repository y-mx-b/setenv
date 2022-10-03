use crate::format::Format;
use serde::Deserialize;
use std::collections::HashMap;

const fn _vi_mode_default() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct Env {
    #[serde(alias = "env-vars", default)]
    env_vars: HashMap<String, Vec<String>>,
    #[serde(alias = "vi-mode", default = "_vi_mode_default")]
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
                    output.push(format!(
                        "{}=\"${}:{}\"; export {}",
                        var,
                        var,
                        value.join(":"),
                        var
                    ));
                }

                if self.vi_mode {
                    output.push("set -o vi".to_string())
                }
            }
            Format::Fish => {
                for (v, mut value) in self.env_vars {
                    let var = v.to_uppercase();
                    value = value
                        .iter_mut()
                        .map(|item| format!("\"{}\"", item))
                        .collect();

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
                    value = value
                        .iter_mut()
                        .map(|item| format!("\"{}\"", item))
                        .collect();

                    output.push(format!("setenv {} = (${} {})", var, var, value.join(" ")));
                }

                if self.vi_mode {
                    output.push("bindkey -v".to_string());
                }
            }
        }

        output.join("\n")
    }
}
