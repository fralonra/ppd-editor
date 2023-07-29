use std::path::PathBuf;

pub struct Config {
    pub canvas_show_slot_boundaries: bool,
    pub file_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            canvas_show_slot_boundaries: false,
            file_path: None,
        }
    }
}
