use crate::format::Format;
use serde::Deserialize;
use std::collections::HashMap;

const fn _vi_mode_default() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct Env {
    #[serde(alias = "vi-mode", default = "_vi_mode_default")]
    vi_mode: bool,
    #[serde(alias = "env-vars", default)]
    env_vars: HashMap<String, Vec<String>>,
    #[serde(default)]
    aliases: HashMap<String, String>,
    // TODO add functions
}

impl Env {
    fn to_shell(self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        // vi-mode
        if self.vi_mode {
            output.push("set -o vi".to_string())
        }

        // env-vars
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

        // aliases
        for (k, value) in self.aliases {
            let alias = k.to_lowercase();

            output.push(format!(
                "alias {}='{}'",
                alias,
                value
            ));
        }

        output
    }

    fn to_fish(self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        // vi-mode
        if self.vi_mode {
            output.push("fish_vi_key_bindings".to_string());
        }

        // env-vars
        for (v, mut value) in self.env_vars {
            let var = v.to_uppercase();
            value = value
                .iter_mut()
                .map(|item| format!("\"{}\"", item))
                .collect();

            // separate with spaces for arrays
            output.push(format!("set -gx {} ${} {}", var, var, value.join(" ")));
        }

        // aliases
        for (k, value) in self.aliases {
            let alias = k.to_lowercase();
            output.push(format!("alias {} '{}'", alias, value));
        }

        output
    }

    fn to_tcsh(self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        // vi-mode
        if self.vi_mode {
            output.push("bindkey -v".to_string());
        }

        // env-vars
        // TODO errors out if var is not set already
        for (v, value) in self.env_vars {
            let var = v.to_uppercase();

            output.push(format!(
                "setenv {} \"${{{}}}:{}:\"",
                var,
                var,
                value.join(":"),
            ));
        }

        // aliases
        for (k, value) in self.aliases {
            let alias = k.to_uppercase();

            output.push(format!("alias {} '{}'", alias, value));
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
