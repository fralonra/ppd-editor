use eframe::{
    egui::{Layout, Response, Sense, Ui, Widget, WidgetText},
    emath::Align,
};
use material_icons::{icon_to_char, Icon};
use paperdoll_tar::paperdoll::slot::Slot;

use crate::common::layout_text_widget;

pub struct SlotEntry<'a> {
    slot: &'a Slot,
    actived: bool,
    visible: bool,

    visibility_changable: bool,
    editable: bool,
    removable: bool,

    visibility_hint: WidgetText,
    edit_hint: WidgetText,
    remove_hint: WidgetText,

    on_visiblity_changed: Option<Box<dyn 'a + FnOnce(bool)>>,
    on_edit: Option<Box<dyn 'a + FnOnce(&'a Slot)>>,
    on_remove: Option<Box<dyn 'a + FnOnce(&'a Slot)>>,
}

impl<'a> Widget for SlotEntry<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let text = if self.slot.desc.is_empty() {
            WidgetText::from(format!("Unnamed Slot - {}", self.slot.id()))
        } else {
            WidgetText::from(&self.slot.desc)
        };

        let padding = ui.spacing().button_padding;

        let (text, desired_size) = layout_text_widget(ui, text, padding);

        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

        let visuals = ui.style().interact_selectable(&response, self.actived);

        if self.actived {
            let rect = rect.expand(visuals.expansion);

            ui.painter()
                .rect(rect, 0.0, visuals.weak_bg_fill, visuals.bg_stroke);
        }

        let text_pos = ui
            .layout()
            .align_size_within_rect(text.size(), rect.shrink2(padding))
            .min;

        text.paint_with_visuals(ui.painter(), text_pos, &visuals);

        if self.visibility_changable || self.editable || self.removable {
            if ui.rect_contains_pointer(rect) {
                ui.put(rect, |ui: &mut Ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if self.removable {
                            let resp = ui.button(icon_to_char(Icon::Delete).to_string());

                            let resp = if !self.remove_hint.is_empty() {
                                resp.on_hover_text(self.remove_hint)
                            } else {
                                resp
                            };

                            if resp.clicked() {
                                if let Some(on_remove) = self.on_remove {
                                    on_remove(self.slot);
                                }
                            }
                        }

                        if self.editable {
                            let resp = ui.button(icon_to_char(Icon::Edit).to_string());

                            let resp = if !self.edit_hint.is_empty() {
                                resp.on_hover_text(self.edit_hint)
                            } else {
                                resp
                            };

                            if resp.clicked() {
                                if let Some(on_edit) = self.on_edit {
                                    on_edit(self.slot);
                                }
                            }
                        }

                        if self.visibility_changable {
                            let resp = ui.button(
                                icon_to_char(if self.visible {
                                    Icon::Visibility
                                } else {
                                    Icon::VisibilityOff
                                })
                                .to_string(),
                            );

                            let resp = if !self.visibility_hint.is_empty() {
                                resp.on_hover_text(self.visibility_hint)
                            } else {
                                resp
                            };

                            if resp.clicked() {
                                if let Some(on_visiblity_changed) = self.on_visiblity_changed {
                                    on_visiblity_changed(!self.visible);
                                }
                            }
                        }
                    })
                    .response
                });
            }
        }

        response
    }
}

impl<'a> SlotEntry<'a> {
    pub fn new(slot: &'a Slot) -> Self {
        Self {
            slot,
            actived: false,
            visible: false,
            visibility_changable: true,
            editable: true,
            removable: true,
            visibility_hint: WidgetText::default(),
            edit_hint: WidgetText::default(),
            remove_hint: WidgetText::default(),
            on_visiblity_changed: None,
            on_edit: None,
            on_remove: None,
        }
    }

    pub fn actived(mut self, actived: bool) -> Self {
        self.actived = actived;
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    pub fn edit_hint(mut self, edit_hint: impl Into<WidgetText>) -> Self {
        self.edit_hint = edit_hint.into();
        self
    }

    pub fn on_edit(mut self, on_edit: impl 'a + FnOnce(&Slot)) -> Self {
        self.on_edit = Some(Box::new(on_edit));
        self
    }

    pub fn on_remove(mut self, on_remove: impl 'a + FnOnce(&Slot)) -> Self {
        self.on_remove = Some(Box::new(on_remove));
        self
    }

    pub fn on_visiblity_changed(mut self, on_visiblity_changed: impl 'a + FnOnce(bool)) -> Self {
        self.on_visiblity_changed = Some(Box::new(on_visiblity_changed));
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

    pub fn visibility_changable(mut self, visibility_changable: bool) -> Self {
        self.visibility_changable = visibility_changable;
        self
    }

    pub fn visibility_hint(mut self, visibility_hint: impl Into<WidgetText>) -> Self {
        self.visibility_hint = visibility_hint.into();
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}
