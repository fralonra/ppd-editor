use std::collections::HashMap;

use eframe::{
    egui::{
        Button, CentralPanel, ComboBox, Context, Grid, Painter, RichText, Sense, SidePanel,
        TopBottomPanel, Ui,
    },
    epaint::{pos2, vec2, Color32, Rect, TextureId, Vec2},
};
use material_icons::{icon_to_char, Icon};
use paperdoll_tar::paperdoll::{doll::Doll, render_material::RenderMaterial, slot::Slot};

use crate::common::{determine_doll_rect, TextureData};

use super::{actions::Action, ViewerApp};

impl ViewerApp {
    pub(super) fn ui(&mut self, ctx: &Context) {
        if self.ppd.is_none() {
            CentralPanel::default().show(ctx, |ui| {
                self.ui_splash(ui);
            });

            return;
        }

        TopBottomPanel::top("action")
            .exact_height(40.0)
            .show(ctx, |ui| {
                self.ui_action_bar(ui);
            });

        SidePanel::right("control")
            .resizable(false)
            .show(ctx, |ui| {
                self.ui_control(ui);
            });

        CentralPanel::default().show(ctx, |ui| {
            self.ui_canvas(ui);
        });
    }

    fn ui_action_bar(&mut self, ui: &mut Ui) {
        if self.ppd.is_none() {
            return;
        }

        let ppd = self.ppd.as_ref().unwrap();

        let ctx = ui.ctx();

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.app_quit)) {
            self.actions.push(Action::AppQuit);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut.file_open)) {
            self.actions.push(Action::FileOpen);
        }

        ui.spacing_mut().item_spacing.y = 10.0;

        ui.horizontal_centered(|ui| {
            if ui
                .add(Button::new(icon_to_char(Icon::FileOpen).to_string()))
                .on_hover_text("Open paperdoll file")
                .clicked()
            {
                self.actions.push(Action::FileOpen);

                ui.close_menu();
            }

            if ppd.dolls().len() > 0 {
                ui.separator();

                ui.label("Doll: ");

                let doll_title = ppd
                    .get_doll(self.actived_doll)
                    .map_or("Doll Not Found".to_owned(), map_doll_title);

                ComboBox::from_label("")
                    .selected_text(doll_title)
                    .show_ui(ui, |ui| {
                        for (id, doll) in ppd.dolls() {
                            ui.selectable_value(&mut self.actived_doll, *id, map_doll_title(doll));
                        }
                    });
            }
        });
    }

    fn ui_canvas(&mut self, ui: &mut Ui) {
        if self.ppd.is_none() {
            return;
        }

        let ppd = self.ppd.as_ref().unwrap();

        let doll = ppd.get_doll(self.actived_doll);

        if doll.is_none() {
            return;
        }

        let doll = doll.unwrap();

        let doll_rect = determine_doll_rect(doll, &ui.max_rect());

        let painter = ui.painter_at(doll_rect);

        if let Ok(material) = ppd.render(doll.id(), &self.slot_map, true) {
            render_paperdoll(
                material,
                &self.textures_doll,
                &self.textures_fragment,
                &painter,
            )
        }
    }

    fn ui_control(&mut self, ui: &mut Ui) {
        if self.ppd.is_none() {
            return;
        }

        let ppd = self.ppd.as_ref().unwrap();

        let doll = ppd.get_doll(self.actived_doll);

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

                                    self.actions
                                        .push(Action::SlotFragmentChanged(*id, *current_index));
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

                                    self.actions
                                        .push(Action::SlotFragmentChanged(*id, *current_index));
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
                    ui.vertical_centered(|ui| {
                        ui.heading("Paperdoll Viewer");
                        ui.strong(env!("CARGO_PKG_VERSION"));

                        if let Some(hash) = option_env!("GIT_COMMIT_HASH") {
                            ui.strong(hash);
                        }
                    });

                    if let Some(homepage) = option_env!("CARGO_PKG_HOMEPAGE") {
                        ui.hyperlink(homepage);
                    }

                    if ui
                        .button(RichText::new("Open From File").heading())
                        .clicked()
                    {
                        self.actions.push(Action::FileOpen);
                    }
                });
            },
        );
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

fn render_paperdoll(
    material: RenderMaterial,
    textures_doll: &HashMap<u32, TextureData>,
    textures_fragment: &HashMap<u32, TextureData>,
    painter: &Painter,
) {
    let RenderMaterial { doll, slots, .. } = material;

    if let Some(piece) = doll {
        if let Some(texture) = textures_doll.get(&piece.id) {
            render_texture(
                &texture.texture.id(),
                piece.position.x,
                piece.position.y,
                piece.image.width as f32,
                piece.image.height as f32,
                painter,
            )
        }
    }

    for piece in slots {
        if let Some(texture) = textures_fragment.get(&piece.id) {
            render_texture(
                &texture.texture.id(),
                piece.position.x,
                piece.position.y,
                piece.image.width as f32,
                piece.image.height as f32,
                painter,
            )
        }
    }

    fn render_texture(
        texture_id: &TextureId,
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        painter: &Painter,
    ) {
        let min = painter.clip_rect().min + vec2(left, top);
        let max = min + vec2(width, height);

        let rect = Rect::from([min, max]);

        painter.image(
            *texture_id,
            rect,
            Rect::from([pos2(0.0, 0.0), pos2(1.0, 1.0)]),
            Color32::WHITE,
        );
    }
}