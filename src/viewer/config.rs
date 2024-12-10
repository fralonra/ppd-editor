use std::path::PathBuf;

pub struct Config {
    pub file_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self { file_path: None }
    }
}
