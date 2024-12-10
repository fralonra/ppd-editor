use std::path::PathBuf;

use anyhow::Result;

use crate::fixed_vec::FixedVec;

const RECENT_FILE_COUNT: usize = 5;

const KEY_RECENT_FILES: &'static str = "recent_files";

pub struct Storage {
    pub recent_files: FixedVec<PathBuf>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            recent_files: FixedVec::new(RECENT_FILE_COUNT),
        }
    }
}

impl Storage {
    pub fn restore(&mut self, storage: &dyn eframe::Storage) -> Result<()> {
        if let Some(value) = storage.get_string(KEY_RECENT_FILES) {
            self.recent_files = serde_json::from_str(&value)?;

            self.recent_files
                .resize(RECENT_FILE_COUNT, PathBuf::default());
        }

        Ok(())
    }

    pub fn save(&self, storage: &mut dyn eframe::Storage) -> Result<()> {
        storage.set_string(KEY_RECENT_FILES, serde_json::to_string(&self.recent_files)?);

        Ok(())
    }
}
