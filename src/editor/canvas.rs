mod snap;

use eframe::{
    egui::{
        scroll_area::ScrollBarVisibility, CursorIcon, Id, PointerButton, ScrollArea, Sense, Ui,
    },
    epaint::{pos2, vec2, Color32, Pos2, Rect, Stroke, Vec2},
};
use paperdoll_tar::paperdoll::common::Point;

use crate::common::{determine_doll_rect, drag_move};

use self::snap::{drag_snap, SnapInput, SnapOutput, SnapType};

use super::{actions::Action, EditorApp};

#[derive(Default)]
pub enum CanvasState {
    #[default]
    Idle,
    ActivedSlotHover,
    Dragging,
    DraggingAnchor,
    DraggingSlot,
    ResizingSlot,
}

#[derive(Default, PartialEq)]
enum DragRestrict {
    #[default]
    None,
    Horizontal,
    Vertical,
}

#[derive(Default, PartialEq)]
enum RatioKeepOptions {
    #[default]
    Idle,
    MinX,
    MinY,
    MaxX,
    MaxY,
}

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

                let mut state = CanvasState::default();

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

                    state = CanvasState::Dragging;
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

                let mut anchor_point = None;
                let mut slot_drag_point = None;

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

                    let aspect_ratio = slot.width as f32 / slot.height as f32;

                    let mut new_positions = slot.positions.clone();
                    let mut new_width = slot.width;
                    let mut new_height = slot.height;
                    let mut ratio_keep_options = RatioKeepOptions::default();
                    let mut snap_input = SnapInput::default();

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

                        if slot_resp.hovered() && !is_locked {
                            ui.ctx().set_cursor_icon(CursorIcon::Move);

                            if is_actived_slot {
                                state = CanvasState::ActivedSlotHover;
                            }
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
                                            + (vec2(slot.anchor.x, slot.anchor.y)
                                                - vec2(fragment.pivot.x, fragment.pivot.y))
                                                * scale;
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
                            let actived_stroke = Stroke::new(2.0, Color32::from_gray(180));

                            painter.rect_stroke(slot_rect, 0.0, actived_stroke);

                            if !is_locked {
                                let dragged = slot_resp.dragged();
                                let mut control_point_dragged = false;
                                let mut anchor_point_dragged = false;

                                let mut min = min;
                                let mut max = max;

                                if dragged {
                                    slot_drag_point = ui.ctx().pointer_interact_pos();

                                    if self.canvas_original_pos_slot_and_drag_offset.is_none() {
                                        self.canvas_original_pos_slot_and_drag_offset =
                                            ui.ctx().pointer_interact_pos().map(|pos| {
                                                (
                                                    slot.positions.clone(),
                                                    (pos - doll_rect.min) / scale
                                                        - vec2(position.x, position.y),
                                                )
                                            });
                                    }
                                }

                                // paint controls
                                let control_size = Vec2::splat(8.0);

                                let is_alt_pressed = ui.input(|i| i.modifiers.alt);
                                let is_ctrl_pressed = ui.input(|i| i.modifiers.ctrl);
                                let is_shift_pressed = ui.input(|i| i.modifiers.shift);

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

                                        snap_input.min = Some((min, SnapType::Both));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MinY;
                                        }

                                        control_point_dragged = true;
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

                                        snap_input.min = Some((min, SnapType::Y));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MaxX;
                                        }

                                        control_point_dragged = true;
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

                                        snap_input.min = Some((min, SnapType::Y));
                                        snap_input.max = Some((max, SnapType::X));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MaxX;
                                        }

                                        control_point_dragged = true;
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

                                        snap_input.max = Some((max, SnapType::X));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MaxY;
                                        }

                                        control_point_dragged = true;
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

                                        snap_input.max = Some((max, SnapType::Both));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MaxX;
                                        }

                                        control_point_dragged = true;
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

                                        snap_input.max = Some((max, SnapType::Y));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MaxX;
                                        }

                                        control_point_dragged = true;
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

                                        snap_input.min = Some((min, SnapType::X));
                                        snap_input.max = Some((max, SnapType::Y));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MaxY;
                                        }

                                        control_point_dragged = true;
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

                                        snap_input.min = Some((min, SnapType::X));

                                        if is_ctrl_pressed {
                                            ratio_keep_options = RatioKeepOptions::MaxY;
                                        }

                                        control_point_dragged = true;
                                    },
                                );

                                // paint anchor
                                if !slot.constrainted {
                                    let anchor_radius = 5.0;
                                    let anchor_center =
                                        slot_rect.min + vec2(slot.anchor.x, slot.anchor.y) * scale;
                                    let anchor_rect = Rect::from_center_size(
                                        anchor_center,
                                        Vec2::splat(anchor_radius),
                                    );

                                    painter.circle_stroke(
                                        anchor_center,
                                        anchor_radius,
                                        Stroke::new(3.0, Color32::from_gray(220)),
                                    );

                                    let anchor_resp = ui.interact(
                                        anchor_rect,
                                        Id::new(format!("slot_{}_anchor", slot_id)),
                                        Sense::drag(),
                                    );

                                    if anchor_resp.dragged() {
                                        anchor_point = ui
                                            .ctx()
                                            .pointer_interact_pos()
                                            .map(|pos| (pos, slot_rect));

                                        if self.canvas_original_pos_anchor.is_none() {
                                            self.canvas_original_pos_anchor = Some(slot.anchor);
                                        }

                                        if let Some((point, _)) = anchor_point {
                                            snap_input.anchor = Some((point, SnapType::Both));
                                        }

                                        state = CanvasState::DraggingAnchor;

                                        anchor_point_dragged = true;
                                    }
                                }

                                // store updates
                                if dragged || control_point_dragged || anchor_point_dragged {
                                    let mut drag_offset: Option<Vec2> = None;
                                    let mut drag_restrict = DragRestrict::default();

                                    if dragged {
                                        if let Some(global_point) = slot_drag_point {
                                            if let Some((origins, offset)) =
                                                &self.canvas_original_pos_slot_and_drag_offset
                                            {
                                                let delta =
                                                    global_point - slot_rect.min - *offset * scale;

                                                min += delta;
                                                max += delta;

                                                snap_input.min = Some((min, SnapType::Both));
                                                snap_input.max = Some((max, SnapType::Both));

                                                // restrict direction
                                                if is_ctrl_pressed {
                                                    let point =
                                                        (global_point - doll_rect.min) / scale;

                                                    let origin = origins[position_index];

                                                    let x_offset = point.x - origin.x - offset.x;
                                                    let y_offset = point.y - origin.y - offset.y;

                                                    if x_offset.abs() > y_offset.abs() {
                                                        drag_restrict = DragRestrict::Horizontal;
                                                    } else {
                                                        drag_restrict = DragRestrict::Vertical;
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    if control_point_dragged {
                                        // keep aspect ratio
                                        match ratio_keep_options {
                                            RatioKeepOptions::MinY => {
                                                min.y = max.y - (max.x - min.x) / aspect_ratio;
                                            }
                                            RatioKeepOptions::MaxX => {
                                                max.x = (max.y - min.y) * aspect_ratio + min.x;
                                            }
                                            RatioKeepOptions::MaxY => {
                                                max.y = (max.x - min.x) / aspect_ratio + min.y;
                                            }
                                            _ => {}
                                        }
                                    }

                                    // snapping
                                    if !is_alt_pressed
                                        && ratio_keep_options == RatioKeepOptions::Idle
                                    {
                                        let snap_stroke = Stroke::new(1.0, Color32::LIGHT_RED);

                                        let snap_output = self.snap_in_doll(&snap_input, doll_rect);

                                        if dragged || control_point_dragged {
                                            let width = slot.width as f32 * scale;
                                            let height = slot.height as f32 * scale;

                                            let mut snap_min_x = false;
                                            let mut snap_min_y = false;
                                            let mut snap_max_x = false;
                                            let mut snap_max_y = false;

                                            if drag_restrict != DragRestrict::Vertical {
                                                if let Some(x) = snap_output.min.x {
                                                    snap_min_x = true;

                                                    min.x = x;
                                                }

                                                if let Some(x) = snap_output.max.x {
                                                    snap_max_x = true;

                                                    max.x = x;
                                                }
                                            }

                                            if drag_restrict != DragRestrict::Horizontal {
                                                if let Some(y) = snap_output.min.y {
                                                    snap_min_y = true;

                                                    min.y = y;
                                                }

                                                if let Some(y) = snap_output.max.y {
                                                    snap_max_y = true;

                                                    max.y = y;
                                                }
                                            }

                                            if let Some(global_point) = slot_drag_point {
                                                let x_not_fit = max.x - min.x != width;
                                                let y_not_fit = max.y - min.y != height;

                                                if x_not_fit || y_not_fit {
                                                    let is_cursor_near_min = global_point
                                                        .distance(min)
                                                        < global_point.distance(max);

                                                    if x_not_fit
                                                        && drag_restrict != DragRestrict::Vertical
                                                    {
                                                        if snap_max_x && snap_min_x {
                                                            if is_cursor_near_min {
                                                                snap_max_x = false;

                                                                max.x = min.x + width;
                                                            } else {
                                                                snap_min_x = false;

                                                                min.x = max.x - width;
                                                            }
                                                        } else if snap_max_x {
                                                            snap_min_x = false;

                                                            min.x = max.x - width;
                                                        } else if snap_min_x {
                                                            snap_max_x = false;

                                                            max.x = min.x + width;
                                                        }
                                                    }

                                                    if y_not_fit
                                                        && drag_restrict != DragRestrict::Horizontal
                                                    {
                                                        if snap_max_y && snap_min_y {
                                                            if is_cursor_near_min {
                                                                snap_max_y = false;

                                                                max.y = min.y + height;
                                                            } else {
                                                                snap_min_y = false;

                                                                min.y = max.y - height;
                                                            }
                                                        } else if snap_max_y {
                                                            snap_min_y = false;

                                                            min.y = max.y - height;
                                                        } else if snap_min_y {
                                                            snap_max_y = false;

                                                            max.y = min.y + height;
                                                        }
                                                    }
                                                }
                                            }

                                            if snap_min_x {
                                                painter.vline(
                                                    min.x,
                                                    painter.clip_rect().y_range(),
                                                    snap_stroke,
                                                );
                                            }

                                            if snap_min_y {
                                                painter.hline(
                                                    painter.clip_rect().x_range(),
                                                    min.y,
                                                    snap_stroke,
                                                );
                                            }

                                            if snap_max_x {
                                                painter.vline(
                                                    max.x,
                                                    painter.clip_rect().y_range(),
                                                    snap_stroke,
                                                );
                                            }

                                            if snap_max_y {
                                                painter.hline(
                                                    painter.clip_rect().x_range(),
                                                    max.y,
                                                    snap_stroke,
                                                );
                                            }
                                        }

                                        if anchor_point_dragged {
                                            if let Some((anchor_point, _)) = &mut anchor_point {
                                                if let Some(x) = snap_output.anchor.x {
                                                    anchor_point.x = x;

                                                    painter.vline(
                                                        anchor_point.x,
                                                        painter.clip_rect().y_range(),
                                                        snap_stroke,
                                                    );
                                                }

                                                if let Some(y) = snap_output.anchor.y {
                                                    anchor_point.y = y;

                                                    painter.hline(
                                                        painter.clip_rect().x_range(),
                                                        anchor_point.y,
                                                        snap_stroke,
                                                    );
                                                }
                                            }
                                        }
                                    }

                                    if dragged || control_point_dragged {
                                        if let Some(position) =
                                            new_positions.iter_mut().nth(position_index)
                                        {
                                            let mut top_left =
                                                ((min - doll_rect.min) / scale).round();
                                            let bottom_right =
                                                ((max - doll_rect.min) / scale).round();

                                            if drag_restrict != DragRestrict::None {
                                                if let Some((origins, _)) =
                                                    &self.canvas_original_pos_slot_and_drag_offset
                                                {
                                                    if drag_restrict == DragRestrict::Horizontal {
                                                        top_left.y = origins[position_index].y;
                                                    } else {
                                                        top_left.x = origins[position_index].x;
                                                    }
                                                }
                                            }

                                            if is_shift_pressed
                                                && (top_left.x != position.x
                                                    || top_left.y != position.y)
                                            {
                                                drag_offset = Some(vec2(
                                                    top_left.x - position.x,
                                                    top_left.y - position.y,
                                                ));
                                            }

                                            position.x = top_left.x;
                                            position.y = top_left.y;

                                            if control_point_dragged {
                                                new_width = (bottom_right.x.round()
                                                    - top_left.x.round())
                                                    as u32;
                                                new_height = (bottom_right.y.round()
                                                    - top_left.y.round())
                                                    as u32;
                                            }
                                        }
                                    }

                                    if dragged && is_shift_pressed {
                                        if let Some(offset) = drag_offset {
                                            for (index, position) in
                                                new_positions.iter_mut().enumerate()
                                            {
                                                if index == position_index {
                                                    continue;
                                                }

                                                position.x += offset.x;
                                                position.y += offset.y;
                                            }
                                        }
                                    }

                                    if control_point_dragged {
                                        state = CanvasState::ResizingSlot;
                                    }

                                    if dragged {
                                        state = CanvasState::DraggingSlot;
                                    }
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

                            if let Some((anchor_point, slot_rect)) = anchor_point {
                                let anchor_point = (anchor_point - slot_rect.min) / scale;

                                if ui.input(|i| i.modifiers.ctrl) {
                                    if let Some(point) = self.canvas_original_pos_anchor {
                                        if (anchor_point.x - point.x).abs()
                                            > (anchor_point.y - point.y).abs()
                                        {
                                            slot.anchor.x = anchor_point.x;
                                            slot.anchor.y = point.y;
                                        } else {
                                            slot.anchor.y = anchor_point.y;
                                            slot.anchor.x = point.x;
                                        }
                                    }
                                } else {
                                    slot.anchor.x = anchor_point.x;
                                    slot.anchor.y = anchor_point.y;
                                }
                            }
                        }
                    }
                }

                if anchor_point.is_none() {
                    self.canvas_original_pos_anchor = None;
                }

                if slot_drag_point.is_none() {
                    self.canvas_original_pos_slot_and_drag_offset = None;
                } else {
                    ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                }

                self.actions.push_back(Action::CanvasStateChanged(state));
            });
    }

    fn snap_in_doll(&self, input: &SnapInput, doll_rect: Rect) -> SnapOutput {
        let basis_rects = vec![doll_rect];

        drag_snap(input, basis_rects, self.config.canvas_snap_tolerance)
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
            on_dragged(pointer);
        }
    }
}

fn slot_drag(position: &mut Point, origin: Point, drag_offset: Vec2, restrict: &DragRestrict) {
    position.x += drag_offset.x;
    position.y += drag_offset.y;

    match restrict {
        DragRestrict::Horizontal => {
            position.y = origin.y;
        }
        DragRestrict::Vertical => {
            position.x = origin.x;
        }
        _ => {}
    }
}
