use std::path::PathBuf;

pub struct Config {
    pub canvas_scale: f32,
    pub file_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            canvas_scale: 1.0,
            file_path: None,
        }
    }
}
