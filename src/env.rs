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
    #[serde(default)]
    aliases: HashMap<String, Vec<String>>,
}

impl Env {
    // TODO implement aliases
    fn to_shell(self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

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

        output
    }

    fn to_fish(self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

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

        output
    }

    fn to_tcsh(self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

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

        output
    }

    pub fn to_string(self, format: Format) -> String {
        match format {
            Format::Sh => self.to_shell(),
            Format::Fish => self.to_fish(),
            Format::Tcsh => self.to_tcsh(),
        }
        .join("\n")
    }
}
