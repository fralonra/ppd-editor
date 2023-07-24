use eframe::{
    egui::{CursorIcon, Response, Sense, Ui, Widget, WidgetText},
    epaint::{pos2, vec2, Color32, Rect, Stroke, Vec2},
};
use material_icons::{icon_to_char, Icon};

use crate::common::{allocate_size_center_in_rect, layout_text_widget, TextureData};

pub struct ImageUpload<'a> {
    size: f32,
    rounding: f32,
    preview_auto_resize: bool,

    texture: Option<&'a TextureData>,

    editable: bool,
    removable: bool,

    edit_hint: WidgetText,
    remove_hint: WidgetText,

    on_edit: Option<Box<dyn 'a + FnOnce()>>,
    on_remove: Option<Box<dyn 'a + FnOnce()>>,
}

impl<'a> Widget for ImageUpload<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.allocate_response(Vec2::splat(self.size), Sense::click());

        let rect = response.rect;

        let stroke = Stroke::new(1.0, Color32::from_gray(128));

        match self.texture {
            Some(texture) => {
                let preview_rect = if self.preview_auto_resize {
                    rect
                } else {
                    allocate_size_center_in_rect(texture.width as f32, texture.height as f32, &rect)
                };

                ui.painter().image(
                    texture.texture.id(),
                    preview_rect,
                    Rect::from([pos2(0.0, 0.0), pos2(1.0, 1.0)]),
                    Color32::WHITE,
                );

                ui.painter_at(rect).rect_stroke(
                    rect,
                    self.rounding,
                    Stroke::new(self.rounding.min(self.size * 0.5), ui.visuals().window_fill),
                );

                if self.editable || self.removable {
                    if ui.rect_contains_pointer(rect) {
                        ui.put(rect, |ui: &mut Ui| {
                            ui.painter().rect_filled(
                                rect,
                                self.rounding,
                                Color32::from_rgba_premultiplied(20, 20, 20, 180),
                            );

                            let mut content_size = Vec2::default();
                            let button_padding = ui.spacing().button_padding;

                            if self.editable {
                                let (_text, size) = layout_text_widget(
                                    ui,
                                    icon_to_char(Icon::Edit).to_string(),
                                    button_padding,
                                );

                                content_size += size;
                            }

                            if self.removable {
                                let (_text, size) = layout_text_widget(
                                    ui,
                                    icon_to_char(Icon::Delete).to_string(),
                                    button_padding,
                                );

                                content_size += size;
                            }

                            let button_count = if self.editable && self.removable {
                                2.0
                            } else {
                                1.0
                            };

                            let item_spacing = ui.spacing().item_spacing;
                            let content_size = vec2(
                                content_size[0] + (button_count - 1.0) * item_spacing.x,
                                content_size[1],
                            );

                            let content_rect = Rect::from_center_size(rect.center(), content_size);

                            ui.allocate_ui_at_rect(content_rect, |ui| {
                                ui.horizontal_centered(|ui| {
                                    if self.editable {
                                        let resp = ui.button(icon_to_char(Icon::Edit).to_string());

                                        let resp = if !self.edit_hint.is_empty() {
                                            resp.on_hover_text(self.edit_hint)
                                        } else {
                                            resp
                                        };

                                        if resp.clicked() {
                                            if let Some(on_edit) = self.on_edit {
                                                on_edit();
                                            }
                                        }
                                    }

                                    if self.removable {
                                        let resp =
                                            ui.button(icon_to_char(Icon::Delete).to_string());

                                        let resp = if !self.remove_hint.is_empty() {
                                            resp.on_hover_text(self.remove_hint)
                                        } else {
                                            resp
                                        };

                                        if resp.clicked() {
                                            if let Some(on_remove) = self.on_remove {
                                                on_remove();
                                            }
                                        }
                                    }
                                });
                            })
                            .response
                        });
                    }
                }
            }
            None => {
                let center = rect.center();
                let half_len = self.size * 0.3;

                ui.painter().line_segment(
                    [center - vec2(0.0, half_len), center + vec2(0.0, half_len)],
                    stroke,
                );
                ui.painter().line_segment(
                    [center - vec2(half_len, 0.0), center + vec2(half_len, 0.0)],
                    stroke,
                );

                if response.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand)
                }
            }
        }

        ui.painter().rect_stroke(rect, self.rounding, stroke);

        response
    }
}

impl<'a> ImageUpload<'a> {
    pub fn new(texture: Option<&'a TextureData>) -> Self {
        Self {
            size: 50.0,
            rounding: 5.0,
            preview_auto_resize: false,
            texture,
            editable: true,
            removable: true,
            edit_hint: WidgetText::default(),
            remove_hint: WidgetText::default(),
            on_edit: None,
            on_remove: None,
        }
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    pub fn edit_hint(mut self, edit_hint: impl Into<WidgetText>) -> Self {
        self.edit_hint = edit_hint.into();
        self
    }

    pub fn on_edit(mut self, on_edit: impl 'a + FnOnce()) -> Self {
        self.on_edit = Some(Box::new(on_edit));
        self
    }

    pub fn on_remove(mut self, on_remove: impl 'a + FnOnce()) -> Self {
        self.on_remove = Some(Box::new(on_remove));
        self
    }

    pub fn preview_auto_resize(mut self, preview_auto_resize: bool) -> Self {
        self.preview_auto_resize = preview_auto_resize;
        self
    }

    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    pub fn remove_hint(mut self, remove_hint: impl Into<WidgetText>) -> Self {
        self.remove_hint = remove_hint.into();
        self
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}
