use eframe::egui::{Key, KeyboardShortcut, Modifiers};

pub(super) struct Shortcut {
    pub app_quit: KeyboardShortcut,
    pub file_open: KeyboardShortcut,
}

impl Default for Shortcut {
    fn default() -> Self {
        Self {
            app_quit: KeyboardShortcut::new(Modifiers::CTRL, Key::Q),
            file_open: KeyboardShortcut::new(Modifiers::CTRL, Key::O),
        }
    }
}
