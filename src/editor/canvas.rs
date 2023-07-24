use eframe::{
    egui::{CursorIcon, Id, Sense, Ui},
    epaint::{pos2, vec2, Color32, Pos2, Rect, Stroke, Vec2},
};

use crate::common::determine_doll_rect;

use super::{actions::Action, EditorApp};

impl EditorApp {
    pub(super) fn ui_canvas(&mut self, ui: &mut Ui) {
        let doll = self.actived_doll.map(|id| self.ppd.get_doll(id)).flatten();

        if doll.is_none() {
            return;
        }

        let doll = doll.unwrap();

        let (resp, painter) = ui.allocate_painter(ui.available_size(), Sense::click());

        if resp.clicked() {
            if !self.window_slot_visible {
                self.actived_slot = None;
            }
        }

        let canvas_rect = resp.rect;

        // paint doll
        let doll_rect = determine_doll_rect(doll, &canvas_rect);

        let scale = doll_rect.width() / (doll.width as f32);

        painter.rect_stroke(doll_rect, 0.0, Stroke::new(1.0, Color32::from_gray(60)));

        if let Some(texture) = self.textures_doll.get(&doll.id()) {
            let doll_image_position = doll_rect.min + vec2(doll.offset.x, doll.offset.y) * scale;

            let doll_image_rect = Rect::from([
                doll_image_position,
                doll_image_position
                    + vec2(doll.image.width as f32, doll.image.height as f32) * scale,
            ]);

            painter.image(
                texture.texture.id(),
                doll_image_rect,
                Rect::from([pos2(0.0, 0.0), pos2(1.0, 1.0)]),
                Color32::WHITE,
            )
        }

        // paint slots
        let slots = doll.slots.clone();

        for slot_id in slots {
            let slot = self.ppd.get_slot(slot_id);

            if slot.is_none() {
                continue;
            }

            let is_visible = self.visible_slots.contains(&slot_id);
            let is_locked = self.locked_slots.contains(&slot_id);

            let slot = slot.unwrap();

            let min = doll_rect.min + vec2(slot.position.x, slot.position.y) * scale;
            let max = min + vec2(slot.width as f32, slot.height as f32) * scale;

            let slot_rect = Rect::from([min, max]);

            let slot_resp = ui
                .allocate_rect(slot_rect, Sense::drag())
                .context_menu(|ui| {
                    if ui.button("Edit slot").clicked() {
                        self.actions.push(Action::SlotEdit(slot_id));

                        ui.close_menu();
                    }

                    if ui.button("Delete slot").clicked() {
                        self.actions.push(Action::SlotRemove(slot_id));

                        ui.close_menu();
                    }
                });

            if slot_resp.dragged() {
                self.actived_slot = Some(slot_id);
            }

            if slot_resp.hovered() && !is_locked {
                ui.ctx().set_cursor_icon(CursorIcon::Move);
            }

            let is_actived_slot = self
                .actived_slot
                .map_or(false, |actived_slot| actived_slot == slot_id);

            // paint fragment
            if is_visible {
                let fragment = is_actived_slot
                    .then(|| {
                        self.actived_fragment
                            .map(|id| {
                                slot.candidates
                                    .contains(&id)
                                    .then(|| self.ppd.get_fragment(id))
                            })
                            .flatten()
                            .flatten()
                    })
                    .flatten()
                    .or_else(|| {
                        slot.candidates
                            .first()
                            .map(|id| self.ppd.get_fragment(*id))
                            .flatten()
                    });

                if let Some(fragment) = fragment {
                    if let Some(fragment_texture) = self.textures_fragment.get(&fragment.id()) {
                        let fragment_rect = if slot.constrainted {
                            slot_rect
                        } else {
                            let min = slot_rect.min + vec2(slot.anchor.x, slot.anchor.y) * scale
                                - vec2(fragment.pivot.x, fragment.pivot.y) * scale;
                            let max = min
                                + vec2(fragment.image.width as f32, fragment.image.height as f32)
                                    * scale;

                            Rect::from([min, max])
                        };

                        painter.image(
                            fragment_texture.texture.id(),
                            fragment_rect,
                            Rect::from([pos2(0.0, 0.0), pos2(1.0, 1.0)]),
                            Color32::WHITE,
                        );
                    }
                }
            }

            // paint actived slot
            if is_actived_slot {
                let mut min = min;
                let mut max = max;

                let actived_stroke = Stroke::new(2.0, Color32::from_gray(180));

                painter.rect_stroke(slot_rect, 0.0, actived_stroke);

                if !is_locked {
                    // paint controls
                    let control_size = Vec2::splat(8.0);

                    control_point(
                        format!("slot_{}_control_tl", slot_id),
                        slot_rect.left_top(),
                        control_size,
                        actived_stroke,
                        CursorIcon::ResizeNwSe,
                        ui,
                        |pos| {
                            min = pos;
                        },
                    );

                    control_point(
                        format!("slot_{}_control_tr", slot_id),
                        slot_rect.right_top(),
                        control_size,
                        actived_stroke,
                        CursorIcon::ResizeNeSw,
                        ui,
                        |pos| {
                            min.y = pos.y;
                            max.x = pos.x;
                        },
                    );

                    control_point(
                        format!("slot_{}_control_br", slot_id),
                        slot_rect.right_bottom(),
                        control_size,
                        actived_stroke,
                        CursorIcon::ResizeNwSe,
                        ui,
                        |pos| {
                            max = pos;
                        },
                    );

                    control_point(
                        format!("slot_{}_control_bl", slot_id),
                        slot_rect.left_bottom(),
                        control_size,
                        actived_stroke,
                        CursorIcon::ResizeNeSw,
                        ui,
                        |pos| {
                            min.x = pos.x;
                            max.y = pos.y;
                        },
                    );

                    // paint anchor
                    let anchor_delta = if !slot.constrainted {
                        let anchor_radius = 5.0;
                        let anchor_point =
                            slot_rect.min + vec2(slot.anchor.x, slot.anchor.y) * scale;
                        let anchor_rect =
                            Rect::from_center_size(anchor_point, Vec2::splat(anchor_radius));

                        painter.circle_stroke(
                            anchor_point,
                            anchor_radius,
                            Stroke::new(3.0, Color32::from_gray(220)),
                        );

                        let anchor_resp = ui.interact(
                            anchor_rect,
                            Id::new(format!("slot_{}_anchor", slot_id)),
                            Sense::drag(),
                        );

                        Some(anchor_resp.drag_delta())
                    } else {
                        None
                    };

                    // update slot
                    if let Some(slot) = self.ppd.get_slot_mut(slot_id) {
                        let min = min - doll_rect.min;
                        let max = max - doll_rect.min;

                        slot.position.x = min.x.round();
                        slot.position.y = min.y.round();

                        let drag_delta = slot_resp.drag_delta();
                        slot.position.x += drag_delta.x;
                        slot.position.y += drag_delta.y;

                        slot.width = (max.x.round() - min.x.round()) as u32;
                        slot.height = (max.y.round() - min.y.round()) as u32;

                        if let Some(anchor_delta) = anchor_delta {
                            slot.anchor.x += anchor_delta.x;
                            slot.anchor.y += anchor_delta.y;
                        }
                    }
                }
            } else {
                painter.rect_stroke(slot_rect, 0.0, Stroke::new(1.0, Color32::from_gray(128)));
            }
        }
    }
}

fn control_point(
    id: impl Into<Id>,
    center_point: Pos2,
    size: Vec2,
    stroke: Stroke,
    cursor_icon: CursorIcon,
    ui: &mut Ui,
    mut on_drag: impl FnMut(Pos2),
) {
    let rect = Rect::from_center_size(center_point, size);

    ui.painter_at(rect).rect_stroke(rect, 0.0, stroke);

    let resp = ui.interact(rect, id.into(), Sense::drag());

    if resp.hovered() {
        ui.ctx().set_cursor_icon(cursor_icon);
    }

    if resp.dragged() {
        if let Some(pointer) = ui.ctx().pointer_interact_pos() {
            on_drag(pointer)
        }
    }
}
