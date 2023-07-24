use eframe::egui::{Response, Ui, Widget, WidgetText};
use material_icons::{icon_to_char, Icon};

pub struct Tooltip {
    text: WidgetText,
}

impl Widget for Tooltip {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(icon_to_char(Icon::HelpOutline).to_string())
            .on_hover_text(self.text)
    }
}

impl Tooltip {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self { text: text.into() }
    }
}
