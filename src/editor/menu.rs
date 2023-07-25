use eframe::egui::{menu, Button, Ui};

use super::{actions::Action, EditorApp};

impl EditorApp {
    pub(super) fn ui_menu_bar(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.app_quit)) {
            self.actions.push_back(Action::AppQuit)
        }

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

            ui.menu_button("Edit", |ui| {});

            ui.menu_button("Doll", |ui| {
                let doll = self.actived_doll.map(|id| self.ppd.get_doll(id)).flatten();

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
                });

                ui.separator();

                ui.add_enabled_ui(doll.is_some(), |ui| {
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
            });

            ui.menu_button("Slot", |ui| {
                let slot = self.actived_slot.map(|id| self.ppd.get_slot(id)).flatten();

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
                });
            });

            ui.menu_button("Fragment", |ui| {
                let fragment = self
                    .actived_fragment
                    .map(|id| self.ppd.get_fragment(id))
                    .flatten();

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
                });
            });

            // ui.menu_button("Preview", |ui| {
            //     if ui
            //         .add(
            //             Button::new("Open in Viewer")
            //                 .shortcut_text(ui.ctx().format_shortcut(&self.shortcut.preview)),
            //         )
            //         .clicked()
            //     {
            //         self.actions.push_back(Action::PpdPreview);

            //         ui.close_menu();
            //     }
            // });

            ui.menu_button("Help", |ui| {});
        });
    }
}
