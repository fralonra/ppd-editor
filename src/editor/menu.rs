use eframe::{
    egui::{menu, Button, Ui},
    epaint::vec2,
};

use super::{actions::Action, EditorApp};

impl EditorApp {
    pub(super) fn menu_doll(&mut self, ui: &mut Ui, id: Option<u32>) {
        let doll = id.map(|id| self.ppd.get_doll(id)).flatten();

        if ui.button("New Doll").clicked() {
            self.actions.push_back(Action::DollCreate);

            ui.close_menu();
        }

        ui.add_enabled_ui(doll.is_some(), |ui| {
            if ui
                .add_enabled(self.ppd.dolls().len() > 1, Button::new("Delete Doll"))
                .clicked()
            {
                self.actions
                    .push_back(Action::DollRemoveRequest(doll.unwrap().id()));

                ui.close_menu();
            }

            ui.separator();

            if ui
                .add_enabled(
                    !doll.unwrap().image.is_empty(),
                    Button::new("Resize to Background Size"),
                )
                .clicked()
            {
                self.actions
                    .push_back(Action::DollResizeToBackground(doll.unwrap().id()));

                ui.close_menu();
            }
        });
    }

    pub(super) fn menu_fragment(&mut self, ui: &mut Ui, id: Option<u32>) {
        let fragment = id.map(|id| self.ppd.get_fragment(id)).flatten();

        if ui.button("New Fragment").clicked() {
            self.actions.push_back(Action::FragmentCreate);

            ui.close_menu();
        }

        ui.add_enabled_ui(fragment.is_some(), |ui| {
            if ui.button("Edit Fragment").clicked() {
                self.actions
                    .push_back(Action::FragmentEdit(fragment.unwrap().id()));

                ui.close_menu();
            }

            if ui.button("Delete Fragment").clicked() {
                self.actions
                    .push_back(Action::FragmentRemoveRequest(fragment.unwrap().id()));

                ui.close_menu();
            }

            ui.separator();

            if ui.button("Manage Associated Slots").clicked() {
                self.actions
                    .push_back(Action::AssociatedSlotsEdit(fragment.unwrap().id()));

                ui.close_menu();
            }
        });
    }

    pub(super) fn menu_slot(&mut self, ui: &mut Ui, id: Option<u32>) {
        let slot = id.map(|id| self.ppd.get_slot(id)).flatten();

        if ui.button("New Slot").clicked() {
            self.actions.push_back(Action::SlotCreate);

            ui.close_menu();
        }

        ui.add_enabled_ui(slot.is_some(), |ui| {
            if ui.button("Edit Slot").clicked() {
                self.actions.push_back(Action::SlotEdit(slot.unwrap().id()));

                ui.close_menu();
            }

            if ui.button("Delete Slot").clicked() {
                self.actions
                    .push_back(Action::SlotRemoveRequest(slot.unwrap().id()));

                ui.close_menu();
            }

            ui.separator();

            if ui
                .add(
                    Button::new("Copy Slot")
                        .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.slot_copy)),
                )
                .clicked()
            {
                self.actions.push_back(Action::SlotCopy(slot.unwrap().id()));

                ui.close_menu();
            }
        });

        if ui
            .add_enabled(
                self.actived_doll.is_some() && self.slot_copy.is_some(),
                Button::new("Paste Slot")
                    .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.slot_paste)),
            )
            .clicked()
        {
            self.actions
                .push_back(Action::SlotPaste(self.actived_doll.unwrap()));

            ui.close_menu();
        }

        ui.add_enabled_ui(slot.is_some(), |ui| {
            if ui
                .add_enabled(
                    self.actived_doll.is_some() && slot.is_some(),
                    Button::new("Duplicate Slot")
                        .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.slot_duplicate)),
                )
                .clicked()
            {
                self.actions.push_back(Action::SlotDuplicate(
                    self.actived_doll.unwrap(),
                    slot.unwrap().id(),
                ));

                ui.close_menu();
            }
        });
    }

    pub(super) fn ui_menu_bar(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_new)) {
            self.actions.push_back(Action::FileNew);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_open)) {
            self.actions.push_back(Action::FileOpen);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_save)) {
            self.actions.push_back(Action::FileSave);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_save_as)) {
            self.actions.push_back(Action::FileSaveAs);
        }

        if let Some(slot_id) = self.actived_slot {
            if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.slot_copy)) {
                self.actions.push_back(Action::SlotCopy(slot_id));
            }
        }

        if self.actived_doll.is_some() && self.slot_copy.is_some() {
            if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.slot_paste)) {
                self.actions
                    .push_back(Action::SlotPaste(self.actived_doll.unwrap()));
            }
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

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.zoom_in)) {
            self.actions
                .push_back(Action::ViewportZoomTo(self.config.canvas_scale * 2.0));
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.zoom_out)) {
            self.actions
                .push_back(Action::ViewportZoomTo(self.config.canvas_scale * 0.5));
        }

        if self.actived_doll.is_some() && self.actived_slot.is_some() {
            if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.slot_duplicate)) {
                self.actions.push_back(Action::SlotDuplicate(
                    self.actived_doll.unwrap(),
                    self.actived_slot.unwrap(),
                ));
            }
        }

        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui
                    .add(
                        Button::new("New")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.file_new)),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::FileNew);

                    ui.close_menu();
                }

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
                        Button::new("Save")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.file_save)),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::FileSave);

                    ui.close_menu();
                }

                if ui
                    .add(
                        Button::new("Save As")
                            .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.file_save_as)),
                    )
                    .clicked()
                {
                    self.actions.push_back(Action::FileSaveAs);

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
                        .push_back(Action::ViewportZoomTo(self.config.canvas_scale * 0.5));

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
                        .push_back(Action::ViewportZoomTo(self.config.canvas_scale * 2.0));

                    ui.close_menu();
                }
            });

            ui.menu_button("Doll", |ui| {
                self.menu_doll(ui, self.actived_doll);
            });

            ui.menu_button("Slot", |ui| {
                self.menu_slot(ui, self.actived_slot);
            });

            ui.menu_button("Fragment", |ui| {
                self.menu_fragment(ui, self.actived_fragment);
            });

            ui.menu_button("Help", |ui| {});
        });
    }
}
