use std::path::PathBuf;

pub struct Config {
    pub canvas_show_slot_boundaries: bool,
    pub canvas_snap_tolerance: f32,
    pub file_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            canvas_show_slot_boundaries: false,
            canvas_snap_tolerance: 10.0,
            file_path: None,
        }
    }
}
