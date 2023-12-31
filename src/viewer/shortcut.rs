use eframe::{
    egui::{Context, Key, KeyboardShortcut, Modifiers},
    epaint::vec2,
};

use super::{actions::Action, ViewerApp};

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

impl ViewerApp {
    pub(super) fn handle_shortcut(&mut self, ctx: &Context) {
        ctx.input_mut(|i| {
            if i.consume_shortcut(&self.shortcut.file_open) {
                self.actions.push_back(Action::FileOpen);
            }

            if i.consume_shortcut(&self.shortcut.viewport_center) {
                self.actions.push_back(Action::ViewportCenter);
            }

            if i.consume_shortcut(&self.shortcut.viewport_fit) {
                self.actions.push_back(Action::ViewportFit);
            }

            if i.consume_shortcut(&self.shortcut.viewport_move_down) {
                self.actions
                    .push_back(Action::ViewportMove(vec2(0.0, -10.0)));
            }

            if i.consume_shortcut(&self.shortcut.viewport_move_left) {
                self.actions
                    .push_back(Action::ViewportMove(vec2(10.0, 0.0)));
            }

            if i.consume_shortcut(&self.shortcut.viewport_move_right) {
                self.actions
                    .push_back(Action::ViewportMove(vec2(-10.0, 0.0)));
            }

            if i.consume_shortcut(&self.shortcut.viewport_move_up) {
                self.actions
                    .push_back(Action::ViewportMove(vec2(0.0, 10.0)));
            }

            if i.consume_shortcut(&self.shortcut.zoom_reset) {
                self.actions.push_back(Action::ViewportZoomReset);
            }

            if i.consume_shortcut(&self.shortcut.zoom_in) {
                self.actions
                    .push_back(Action::ViewportZoomTo(self.viewport.scale * 2.0));
            }

            if i.consume_shortcut(&self.shortcut.zoom_out) {
                self.actions
                    .push_back(Action::ViewportZoomTo(self.viewport.scale * 0.5));
            }
        });
    }
}
