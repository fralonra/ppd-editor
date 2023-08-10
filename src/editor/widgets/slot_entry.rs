use eframe::{
    egui::{Response, Sense, Ui, Widget, WidgetText},
    epaint::vec2,
};
use paperdoll_tar::paperdoll::Slot;

use crate::common::layout_text_widget;

pub struct SlotEntry<'a> {
    slot: &'a Slot,
    actived: bool,
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

        let (rect, response) = ui.allocate_at_least(
            vec2(ui.available_width().max(desired_size.x), desired_size.y),
            Sense::click(),
        );

        let visuals = ui.style().interact_selectable(&response, self.actived);

        if self.actived {
            let rect = rect.expand(visuals.expansion);

            ui.painter()
                .rect(rect, 0.0, visuals.weak_bg_fill, visuals.bg_stroke);
        }

        text.paint_with_visuals(&ui.painter_at(rect), rect.min + padding, &visuals);

        response
    }
}

impl<'a> SlotEntry<'a> {
    pub fn new(slot: &'a Slot) -> Self {
        Self {
            slot,
            actived: false,
        }
    }

    pub fn actived(mut self, actived: bool) -> Self {
        self.actived = actived;
        self
    }
}
