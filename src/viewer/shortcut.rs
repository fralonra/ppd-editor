use eframe::egui::{Key, KeyboardShortcut, Modifiers};

pub(super) struct Shortcut {
    pub app_quit: KeyboardShortcut,
    pub file_open: KeyboardShortcut,
    pub viewport_center: KeyboardShortcut,
    pub viewport_fit: KeyboardShortcut,
    pub viewport_move_down: KeyboardShortcut,
    pub viewport_move_left: KeyboardShortcut,
    pub viewport_move_right: KeyboardShortcut,
    pub viewport_move_up: KeyboardShortcut,
    pub zoom_in: KeyboardShortcut,
    pub zoom_out: KeyboardShortcut,
    pub zoom_reset: KeyboardShortcut,
}

impl Default for Shortcut {
    fn default() -> Self {
        Self {
            app_quit: KeyboardShortcut::new(Modifiers::CTRL, Key::Q),
            file_open: KeyboardShortcut::new(Modifiers::CTRL, Key::O),
            viewport_center: KeyboardShortcut::new(Modifiers::CTRL, Key::J),
            viewport_fit: KeyboardShortcut::new(Modifiers::CTRL, Key::K),
            viewport_move_down: KeyboardShortcut::new(Modifiers::NONE, Key::ArrowDown),
            viewport_move_left: KeyboardShortcut::new(Modifiers::NONE, Key::ArrowLeft),
            viewport_move_right: KeyboardShortcut::new(Modifiers::NONE, Key::ArrowRight),
            viewport_move_up: KeyboardShortcut::new(Modifiers::NONE, Key::ArrowUp),
            zoom_in: KeyboardShortcut::new(Modifiers::NONE, Key::PlusEquals),
            zoom_out: KeyboardShortcut::new(Modifiers::NONE, Key::Minus),
            zoom_reset: KeyboardShortcut::new(Modifiers::CTRL, Key::Num0),
        }
    }
}
