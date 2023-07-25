use eframe::egui::{Key, KeyboardShortcut, Modifiers};

pub(super) struct Shortcut {
    pub app_quit: KeyboardShortcut,
    pub file_new: KeyboardShortcut,
    pub file_open: KeyboardShortcut,
    pub file_save: KeyboardShortcut,
    pub file_save_as: KeyboardShortcut,
    pub preview: KeyboardShortcut,
    pub slot_copy: KeyboardShortcut,
    pub slot_duplicate: KeyboardShortcut,
    pub slot_paste: KeyboardShortcut,
}

impl Default for Shortcut {
    fn default() -> Self {
        Self {
            app_quit: KeyboardShortcut::new(Modifiers::CTRL, Key::Q),
            file_new: KeyboardShortcut::new(Modifiers::CTRL, Key::N),
            file_open: KeyboardShortcut::new(Modifiers::CTRL, Key::O),
            file_save: KeyboardShortcut::new(Modifiers::CTRL, Key::S),
            file_save_as: KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, Key::S),
            preview: KeyboardShortcut::new(Modifiers::CTRL, Key::P),
            slot_copy: KeyboardShortcut::new(Modifiers::CTRL, Key::C),
            slot_duplicate: KeyboardShortcut::new(Modifiers::CTRL, Key::D),
            slot_paste: KeyboardShortcut::new(Modifiers::CTRL, Key::V),
        }
    }
}
