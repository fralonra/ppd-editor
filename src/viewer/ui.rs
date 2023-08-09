use eframe::{
    egui::{
        scroll_area::ScrollBarVisibility, Button, CentralPanel, ComboBox, Context, Grid,
        PointerButton, RichText, ScrollArea, Sense, SidePanel, TopBottomPanel, Ui, Window,
    },
    emath::Align2,
    epaint::{pos2, vec2, Color32, Rect, Stroke, Vec2},
};
use material_icons::{icon_to_char, Icon};
use paperdoll_tar::paperdoll::{doll::Doll, slot::Slot};

use crate::common::{determine_doll_rect, drag_move};

use super::{actions::Action, ViewerApp};

impl ViewerApp {
    pub(super) fn ui(&mut self, ctx: &Context) {
        if self.ppd.is_none() {
            CentralPanel::default().show(ctx, |ui| {
                self.ui_splash(ui);
            });

            return;
        }

        TopBottomPanel::top("menu").show(ctx, |ui| {
            self.ui_menu_bar(ui);
        });

        TopBottomPanel::top("action")
            .exact_height(40.0)
            .show(ctx, |ui| {
                self.ui_action_bar(ui);
            });

        TopBottomPanel::bottom("status").show(ctx, |ui| {
            self.ui_status_bar(ui);
        });

        SidePanel::right("control")
            .resizable(false)
            .show(ctx, |ui| {
                self.ui_control(ui);
            });

        CentralPanel::default().show(ctx, |ui| {
            self.ui_canvas(ui);
        });

        self.ui_about_window(ctx);
    }

    fn ui_about_window(&mut self, ctx: &Context) {
        if !self.window_about_visible {
            return;
        }

        Window::new("About")
            .pivot(Align2::CENTER_CENTER)
            .default_pos(ctx.screen_rect().center())
            .collapsible(false)
            .resizable(false)
            .open(&mut self.window_about_visible)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Paperdoll Viewer");

                    let hash = option_env!("GIT_COMMIT_HASH").unwrap_or_default();

                    ui.strong(format!(" v{} {}", env!("CARGO_PKG_VERSION"), hash));

                    if let Some(homepage) = option_env!("CARGO_PKG_HOMEPAGE") {
                        ui.hyperlink(homepage);
                    }
                });

                ui.allocate_space(vec2(ui.available_width(), 10.0));
            });
    }

    fn ui_action_bar(&mut self, ui: &mut Ui) {
        if self.ppd.is_none() {
            return;
        }

        let ppd = self.ppd.as_ref().unwrap();

        let ctx = ui.ctx();

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.app_quit)) {
            self.actions.push_back(Action::AppQuit);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_open)) {
            self.actions.push_back(Action::FileOpen);
        }

        ui.spacing_mut().item_spacing.y = 10.0;

        ui.horizontal_centered(|ui| {
            if ui
                .add(Button::new(icon_to_char(Icon::FileOpen).to_string()))
                .on_hover_text("Open paperdoll file")
                .clicked()
            {
                self.actions.push_back(Action::FileOpen);

                ui.close_menu();
            }

            if ppd.dolls().len() > 0 {
                ui.separator();

                ui.label("Doll: ");

                let doll_title = ppd
                    .get_doll(self.paperdoll.doll)
                    .map_or("Doll Not Found".to_owned(), map_doll_title);

                ComboBox::from_label("")
                    .selected_text(doll_title)
                    .show_ui(ui, |ui| {
                        for (id, doll) in ppd.dolls() {
                            if ui
                                .selectable_value(
                                    &mut self.paperdoll.doll,
                                    *id,
                                    map_doll_title(doll),
                                )
                                .changed()
                            {
                                self.actions.push_back(Action::DollChanged);
                            }
                        }
                    });
            }
        });
    }

    fn ui_canvas(&mut self, ui: &mut Ui) {
        if self.ppd.is_none() {
            return;
        }

        ScrollArea::both()
            .auto_shrink([false, false])
            .enable_scrolling(false)
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                let ppd = self.ppd.as_ref().unwrap();

                let doll = ppd.get_doll(self.paperdoll.doll);

                if doll.is_none() {
                    return;
                }

                let doll = doll.unwrap();

                let (viewport_rect, viewport_resp) =
                    ui.allocate_exact_size(ui.available_size(), Sense::drag());

                self.viewport.rect = viewport_rect;

                if viewport_resp.dragged_by(PointerButton::Secondary) {
                    self.viewport.offset +=
                        drag_move(&viewport_resp, self.viewport.scale, ui.ctx());
                }

                ui.input(|i| {
                    let zoom_delta = i.zoom_delta();
                    if zoom_delta != 1.0 {
                        self.actions
                            .push_back(Action::ViewportZoomTo(self.viewport.scale * zoom_delta));
                    } else {
                        if i.scroll_delta.x != 0.0 {
                            self.actions
                                .push_back(Action::ViewportMove(vec2(i.scroll_delta.x, 0.0)));
                        }

                        if i.scroll_delta.y != 0.0 {
                            self.actions
                                .push_back(Action::ViewportMove(vec2(0.0, i.scroll_delta.y)));
                        }
                    }
                });

                let doll_rect = determine_doll_rect(
                    doll,
                    &viewport_rect,
                    self.viewport.scale,
                    self.viewport.offset,
                );

                let painter = ui.painter_at(ui.max_rect());

                painter.rect_stroke(doll_rect, 0.0, Stroke::new(1.0, Color32::from_gray(60)));

                if let Some(texture) = &self.texture {
                    painter.image(
                        texture.texture.id(),
                        doll_rect,
                        Rect::from([pos2(0.0, 0.0), pos2(1.0, 1.0)]),
                        Color32::WHITE,
                    );
                }
            });
    }

    fn ui_control(&mut self, ui: &mut Ui) {
        if self.ppd.is_none() {
            return;
        }

        let ppd = self.ppd.as_ref().unwrap();

        let doll = ppd.get_doll(self.paperdoll.doll);

        if doll.is_none() {
            return;
        }

        let doll = doll.unwrap();

        let slots = &doll.slots;

        Grid::new("control")
            .num_columns(2)
            .max_col_width(200.0)
            .show(ui, |ui| {
                for id in slots {
                    if let Some(slot) = ppd.get_slot(*id) {
                        ui.strong(map_slot_title(slot));

                        ui.horizontal_centered(|ui| {
                            if ui
                                .button(icon_to_char(Icon::ChevronLeft).to_string())
                                .clicked()
                            {
                                if let Some(current_index) = self.slot_index_map.get_mut(&id) {
                                    *current_index -= 1;

                                    if *current_index < 0 {
                                        if slot.required || *current_index < -1 {
                                            *current_index = slot.candidates.len() as isize - 1;
                                        }
                                    }

                                    self.actions.push_back(Action::SlotFragmentChanged(
                                        *id,
                                        *current_index,
                                    ));
                                }
                            }

                            let (rect, _) = ui.allocate_exact_size(
                                vec2(100.0, ui.available_height()),
                                Sense::hover(),
                            );

                            ui.allocate_ui_at_rect(rect, |ui| {
                                ui.centered_and_justified(|ui| {
                                    let desc = self.slot_index_map.get(&id).map_or(
                                        "Error: index not found",
                                        |index| {
                                            if *index < -1 {
                                                return "Error: index is not valid";
                                            }

                                            if *index == -1 {
                                                return if slot.required {
                                                    "Error: fragment required"
                                                } else {
                                                    "-"
                                                };
                                            }

                                            let fragment = slot
                                                .candidates
                                                .iter()
                                                .nth(*index as usize)
                                                .map(|fragment_id| ppd.get_fragment(*fragment_id))
                                                .flatten();

                                            fragment.map_or(
                                                "Error: fragment not found",
                                                |fragment| {
                                                    fragment
                                                        .desc
                                                        .is_empty()
                                                        .then_some("-")
                                                        .unwrap_or(fragment.desc.as_str())
                                                },
                                            )
                                        },
                                    );

                                    ui.label(desc);
                                })
                            });

                            if ui
                                .button(icon_to_char(Icon::ChevronRight).to_string())
                                .clicked()
                            {
                                if let Some(current_index) = self.slot_index_map.get_mut(&id) {
                                    *current_index += 1;

                                    if *current_index >= slot.candidates.len() as isize {
                                        *current_index = if slot.required { 0 } else { -1 };
                                    }

                                    self.actions.push_back(Action::SlotFragmentChanged(
                                        *id,
                                        *current_index,
                                    ));
                                }
                            }
                        });

                        ui.end_row();
                    }
                }
            });
    }

    fn ui_splash(&mut self, ui: &mut Ui) {
        ui.spacing_mut().button_padding = Vec2::splat(8.0);

        ui.allocate_ui_at_rect(
            Rect::from_center_size(ui.max_rect().center(), vec2(400.0, 200.0)),
            |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Paperdoll Viewer");

                    let hash = option_env!("GIT_COMMIT_HASH").unwrap_or_default();

                    ui.strong(format!(" v{} {}", env!("CARGO_PKG_VERSION"), hash));

                    if let Some(homepage) = option_env!("CARGO_PKG_HOMEPAGE") {
                        ui.hyperlink(homepage);
                    }

                    if ui
                        .button(RichText::new("Open From File").heading())
                        .clicked()
                    {
                        self.actions.push_back(Action::FileOpen);
                    }

                    let recent_files = self.storage.recent_files.iter();

                    if recent_files.len() != 0 {
                        ui.allocate_space(vec2(1.0, 10.0));

                        ui.group(|ui| {
                            ui.set_width(ui.available_width());

                            ui.heading("Recently opened projects");

                            Grid::new("recent").num_columns(2).show(ui, |ui| {
                                for file in recent_files {
                                    let file_name = file
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string();

                                    if ui.add(Button::new(file_name).frame(false)).double_clicked()
                                    {
                                        self.actions
                                            .push_back(Action::FileOpenPath(file.to_path_buf()));
                                    }

                                    if ui
                                        .add(
                                            Button::new(
                                                RichText::new(file.to_string_lossy().to_string())
                                                    .color(Color32::from_gray(90)),
                                            )
                                            .frame(false),
                                        )
                                        .double_clicked()
                                    {
                                        self.actions
                                            .push_back(Action::FileOpenPath(file.to_path_buf()));
                                    }

                                    ui.end_row();
                                }
                            });
                        });
                    }
                });
            },
        );
    }

    fn ui_status_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.set_width(200.0);

                ui.allocate_space(vec2(100.0, 1.0));

                ui.label(format!("{}%", self.viewport.scale * 100.0));
            });

            ui.horizontal(|ui| {
                ui.strong("Ctrl + Scroll");
                ui.label("or");
                ui.strong("+/-");
                ui.label("to zoom in / out");

                ui.strong("Right Drag");
                ui.label("to drag around");

                ui.strong("Arrow Keys");
                ui.label("to move around");

                ui.strong("Scroll");
                ui.label("to move vertically");

                ui.strong("Shift + Scroll");
                ui.label("to move horizontally");
            });
        });
    }
}

fn map_doll_title(doll: &Doll) -> String {
    doll.desc
        .is_empty()
        .then_some(format!("Unnamed Doll - {}", doll.id()))
        .map_or(doll.desc.clone(), |s| s)
}

fn map_slot_title(slot: &Slot) -> String {
    slot.desc
        .is_empty()
        .then_some(format!("Unnamed Slot - {}", slot.id()))
        .map_or(slot.desc.clone(), |s| s)
}
