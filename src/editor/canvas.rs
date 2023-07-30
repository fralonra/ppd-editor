use eframe::{
    egui::{
        scroll_area::ScrollBarVisibility, Context, CursorIcon, Id, PointerButton, Response,
        ScrollArea, Sense, Ui,
    },
    epaint::{pos2, vec2, Color32, Pos2, Rect, Stroke, Vec2},
};
use paperdoll_tar::paperdoll::doll::Doll;

use super::{actions::Action, EditorApp};

impl EditorApp {
    pub(super) fn ui_canvas(&mut self, ui: &mut Ui) {
        ScrollArea::both()
            .auto_shrink([false, false])
            .enable_scrolling(false)
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                let doll = self.actived_doll.map(|id| self.ppd.get_doll(id)).flatten();

                if doll.is_none() {
                    return;
                }

                let doll = doll.unwrap();

                let scale = self.viewport.scale;

                let (viewport_rect, viewport_resp) =
                    ui.allocate_exact_size(ui.available_size(), Sense::drag());

                self.viewport.rect = viewport_rect;

                if viewport_resp.drag_started_by(PointerButton::Primary) {
                    if !self.window_slot_visible {
                        self.actived_slot = None;
                    }
                }

                if viewport_resp.dragged_by(PointerButton::Secondary) {
                    self.viewport.offset += drag_move(&viewport_resp, scale, ui.ctx());
                }

                ui.painter()
                    .rect_filled(viewport_rect, 0.0, Color32::from_gray(60));

                let doll_rect =
                    determine_doll_rect(doll, &viewport_rect, scale, self.viewport.offset);

                if ui.ui_contains_pointer() {
                    if let Some(pointer) = ui.ctx().pointer_interact_pos() {
                        self.actions.push_back(Action::CursorMoved(Some(
                            pos2(pointer.x / scale, pointer.y / scale)
                                - vec2(doll_rect.min.x, doll_rect.min.y) / scale,
                        )));
                    }
                } else {
                    self.actions.push_back(Action::CursorMoved(None));
                }

                ui.input(|i| {
                    if i.scroll_delta.y != 0.0 {
                        self.actions.push_back(Action::ViewportZoomTo(
                            self.viewport.scale + i.scroll_delta.y / 100.0,
                        ));
                    }
                });

                // paint doll
                let painter = ui.painter_at(ui.max_rect());

                painter.rect_stroke(doll_rect, 0.0, Stroke::new(1.0, Color32::from_gray(90)));

                if let Some(texture) = self.textures_doll.get(&doll.id()) {
                    let doll_image_position =
                        doll_rect.min + vec2(doll.offset.x, doll.offset.y) * scale;

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

                    let is_actived_slot = self
                        .actived_slot
                        .map_or(false, |actived_slot| actived_slot == slot_id);
                    let is_visible = self.visible_slots.contains(&slot_id);
                    let is_locked = self.locked_slots.contains(&slot_id);

                    let slot = slot.unwrap();

                    let mut new_positions = slot.positions.clone();
                    let mut new_width = slot.width;
                    let mut new_height = slot.height;
                    let mut anchor_delta = None;

                    for (position_index, position) in slot.positions.iter().enumerate() {
                        let min = doll_rect.min + vec2(position.x, position.y) * scale;
                        let max = min + vec2(slot.width as f32, slot.height as f32) * scale;

                        let slot_rect = Rect::from([min, max]);

                        let slot_resp =
                            ui.allocate_rect(slot_rect, Sense::drag())
                                .context_menu(|ui| {
                                    if is_actived_slot {
                                        if ui.button("Edit slot").clicked() {
                                            self.actions.push_back(Action::SlotEdit(slot_id));

                                            ui.close_menu();
                                        }

                                        if ui.button("Delete slot").clicked() {
                                            self.actions
                                                .push_back(Action::SlotRemoveRequest(slot_id));

                                            ui.close_menu();
                                        }
                                    } else {
                                        ui.close_menu();
                                    }
                                });

                        if slot_resp.dragged_by(PointerButton::Primary) {
                            self.actived_slot = Some(slot_id);
                        }

                        if slot_resp.dragged_by(PointerButton::Secondary) && !is_actived_slot {
                            self.viewport.offset += drag_move(&slot_resp, scale, ui.ctx());
                        }

                        if slot_resp.hovered() && !slot_resp.dragged() && !is_locked {
                            ui.ctx().set_cursor_icon(CursorIcon::Move);
                        }

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
                                if let Some(fragment_texture) =
                                    self.textures_fragment.get(&fragment.id())
                                {
                                    let fragment_rect = if slot.constrainted {
                                        slot_rect
                                    } else {
                                        let min = slot_rect.min
                                            + vec2(slot.anchor.x, slot.anchor.y) * scale
                                            - vec2(fragment.pivot.x, fragment.pivot.y) * scale;
                                        let max = min
                                            + vec2(
                                                fragment.image.width as f32,
                                                fragment.image.height as f32,
                                            ) * scale;

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
                                    format!(
                                        "slot_{}_position_{}_control_tl",
                                        slot_id, position_index
                                    ),
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
                                    format!(
                                        "slot_{}_position_{}_control_t",
                                        slot_id, position_index
                                    ),
                                    slot_rect.center_top(),
                                    control_size,
                                    actived_stroke,
                                    CursorIcon::ResizeVertical,
                                    ui,
                                    |pos| {
                                        min.y = pos.y;
                                    },
                                );

                                control_point(
                                    format!(
                                        "slot_{}_position_{}_control_tr",
                                        slot_id, position_index
                                    ),
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
                                    format!(
                                        "slot_{}_position_{}_control_r",
                                        slot_id, position_index
                                    ),
                                    slot_rect.right_center(),
                                    control_size,
                                    actived_stroke,
                                    CursorIcon::ResizeHorizontal,
                                    ui,
                                    |pos| {
                                        max.x = pos.x;
                                    },
                                );

                                control_point(
                                    format!(
                                        "slot_{}_position_{}_control_br",
                                        slot_id, position_index
                                    ),
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
                                    format!(
                                        "slot_{}_position_{}_control_b",
                                        slot_id, position_index
                                    ),
                                    slot_rect.center_bottom(),
                                    control_size,
                                    actived_stroke,
                                    CursorIcon::ResizeVertical,
                                    ui,
                                    |pos| {
                                        max.y = pos.y;
                                    },
                                );

                                control_point(
                                    format!(
                                        "slot_{}_position_{}_control_bl",
                                        slot_id, position_index
                                    ),
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

                                control_point(
                                    format!(
                                        "slot_{}_position_{}_control_l",
                                        slot_id, position_index
                                    ),
                                    slot_rect.left_center(),
                                    control_size,
                                    actived_stroke,
                                    CursorIcon::ResizeHorizontal,
                                    ui,
                                    |pos| {
                                        min.x = pos.x;
                                    },
                                );

                                // paint anchor
                                anchor_delta = if !slot.constrainted {
                                    let anchor_radius = 5.0;
                                    let anchor_point =
                                        slot_rect.min + vec2(slot.anchor.x, slot.anchor.y) * scale;
                                    let anchor_rect = Rect::from_center_size(
                                        anchor_point,
                                        Vec2::splat(anchor_radius),
                                    );

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

                                // store updates
                                let min = (min - doll_rect.min) / scale;
                                let max = (max - doll_rect.min) / scale;

                                if let Some(position) = new_positions.iter_mut().nth(position_index)
                                {
                                    position.x = min.x.round();
                                    position.y = min.y.round();

                                    if slot_resp.dragged_by(PointerButton::Primary) {
                                        let drag_delta = slot_resp.drag_delta();
                                        position.x += drag_delta.x / scale;
                                        position.y += drag_delta.y / scale;
                                    }
                                }

                                let width = (max.x.round() - min.x.round()) as u32;
                                if width != slot.width {
                                    new_width = width;
                                }

                                let height = (max.y.round() - min.y.round()) as u32;
                                if height != slot.height {
                                    new_height = height;
                                }
                            }
                        } else {
                            if self.config.canvas_show_slot_boundaries {
                                painter.rect_stroke(
                                    slot_rect,
                                    0.0,
                                    Stroke::new(1.0, Color32::from_gray(128)),
                                );
                            }
                        }
                    }

                    if is_actived_slot {
                        // update slot
                        if let Some(slot) = self.ppd.get_slot_mut(slot_id) {
                            for (index, position) in new_positions.iter().enumerate() {
                                slot.positions[index] = *position;
                            }

                            slot.width = new_width;
                            slot.height = new_height;

                            if let Some(anchor_delta) = anchor_delta {
                                slot.anchor.x += anchor_delta.x / scale;
                                slot.anchor.y += anchor_delta.y / scale;
                            }
                        }
                    }
                }
            });
    }
}

fn control_point(
    id: impl Into<Id>,
    center_point: Pos2,
    size: Vec2,
    stroke: Stroke,
    cursor_icon: CursorIcon,
    ui: &mut Ui,
    mut on_dragged: impl FnMut(Pos2),
) {
    let rect = Rect::from_center_size(center_point, size);

    ui.painter_at(rect).rect_stroke(rect, 0.0, stroke);

    let resp = ui.interact(rect, id.into(), Sense::drag());

    if resp.hovered() {
        ui.ctx().set_cursor_icon(cursor_icon);
    }

    if resp.dragged_by(PointerButton::Primary) {
        if let Some(pointer) = ui.ctx().pointer_interact_pos() {
            on_dragged(pointer)
        }
    }
}

fn determine_doll_rect(doll: &Doll, container_rect: &Rect, scale: f32, offset: Vec2) -> Rect {
    Rect::from_center_size(
        container_rect.center(),
        vec2(doll.width as f32, doll.height as f32) * scale,
    )
    .translate(offset * scale)
}

fn drag_move(response: &Response, scale: f32, ctx: &Context) -> Vec2 {
    ctx.set_cursor_icon(CursorIcon::Grabbing);

    response.drag_delta() / scale
}
