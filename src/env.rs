use std::collections::HashMap;
use std::path::PathBuf;

pub struct Env {
    vars: HashMap<String, Vec<String>>,
}
