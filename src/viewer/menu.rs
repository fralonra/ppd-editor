use eframe::{
    egui::{menu, Button, Ui},
    epaint::vec2,
};

use super::{actions::Action, ViewerApp};

impl ViewerApp {
    pub(super) fn ui_menu_bar(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_open)) {
            self.actions.push_back(Action::FileOpen);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.viewport_center)) {
            self.actions.push_back(Action::ViewportCenter);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.viewport_fit)) {
            self.actions.push_back(Action::ViewportFit);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.viewport_move_down)) {
            self.actions
                .push_back(Action::ViewportMove(vec2(0.0, -10.0)));
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.viewport_move_left)) {
            self.actions
                .push_back(Action::ViewportMove(vec2(10.0, 0.0)));
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.viewport_move_right)) {
            self.actions
                .push_back(Action::ViewportMove(vec2(-10.0, 0.0)));
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.viewport_move_up)) {
            self.actions
                .push_back(Action::ViewportMove(vec2(0.0, 10.0)));
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.zoom_reset)) {
            self.actions.push_back(Action::ViewportZoomReset);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.zoom_in)) {
            self.actions
                .push_back(Action::ViewportZoomTo(self.viewport.scale * 2.0));
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.zoom_out)) {
            self.actions
                .push_back(Action::ViewportZoomTo(self.viewport.scale * 0.5));
        }

        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui
                    .add(
                        Button::new("Open")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.file_open)),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::FileOpen);

                    ui.close_menu();
                }

                ui.separator();

                if ui
                    .add(
                        Button::new("Quit")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.app_quit)),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::AppQuit);

                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                if ui
                    .add(
                        Button::new("Zoom Out")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.zoom_out)),
                    )
                    .clicked()
                {
                    self.actions
                        .push_back(Action::ViewportZoomTo(self.viewport.scale * 0.5));

                    ui.close_menu();
                }

                if ui
                    .add(
                        Button::new("Zoom In")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.zoom_in)),
                    )
                    .clicked()
                {
                    self.actions
                        .push_back(Action::ViewportZoomTo(self.viewport.scale * 2.0));

                    ui.close_menu();
                }

                if ui
                    .add(
                        Button::new("Zoom 1:1")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.zoom_reset)),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::ViewportZoomReset);

                    ui.close_menu();
                }

                ui.separator();

                if ui
                    .add(
                        Button::new("Center In Viewport").shortcut_text(
                            ui.ctx().format_shortcut(&self.shortcut.viewport_center),
                        ),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::ViewportCenter);

                    ui.close_menu();
                }

                if ui
                    .add(
                        Button::new("Fit In Viewport")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.viewport_fit)),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::ViewportFit);

                    ui.close_menu();
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    self.actions.push_back(Action::WindowAboutVisible(true));

                    ui.close_menu();
                }
            });
        });
    }
}
