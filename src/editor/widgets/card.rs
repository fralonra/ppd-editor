use eframe::{
    egui::{Response, Sense, TextStyle, Ui, Widget},
    epaint::{pos2, Color32, Rect, Stroke, Vec2},
};

use crate::common::{allocate_size_center_in_rect, TextureData};

pub struct Card<'a> {
    size: f32,
    rounding: f32,
    highlighted: bool,
    preview_auto_resize: bool,

    desc: &'a str,
    texture: Option<&'a TextureData>,

    placeholder_ui: Option<Box<dyn 'a + FnOnce(&mut Ui)>>,
}

impl<'a> Widget for Card<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.allocate_response(Vec2::splat(self.size), Sense::click());

        let rect = response.rect;

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
            }
            None => {
                if let Some(add_contents) = self.placeholder_ui {
                    add_contents(ui);
                }
            }
        }

        let padding = 4.0;

        let text = ui.painter().layout(
            self.desc.to_owned(),
            ui.style()
                .text_styles
                .get(&TextStyle::Button)
                .map(|font| font.clone())
                .unwrap_or_default(),
            ui.visuals().strong_text_color(),
            rect.size().x - padding - padding,
        );

        ui.painter().galley(rect.min + Vec2::splat(padding), text);

        ui.painter().rect_stroke(
            rect,
            self.rounding,
            Stroke::new(1.0, Color32::from_gray(128)),
        );

        if self.highlighted {
            ui.painter().rect_stroke(
                rect,
                self.rounding,
                Stroke::new(4.0, ui.visuals().selection.bg_fill),
            );
        }

        if !self.desc.is_empty() {
            response.on_hover_text(self.desc)
        } else {
            response
        }
    }
}

impl<'a> Card<'a> {
    pub fn new(texture: Option<&'a TextureData>) -> Self {
        Self {
            size: 50.0,
            rounding: 5.0,
            highlighted: false,
            preview_auto_resize: false,
            desc: "",
            texture,
            placeholder_ui: None,
        }
    }

    pub fn add_placeholder_ui(mut self, add_contents: impl 'a + FnOnce(&mut Ui)) -> Self {
        self.placeholder_ui = Some(Box::new(add_contents));
        self
    }

    pub fn desc(mut self, desc: &'a str) -> Self {
        self.desc = desc;
        self
    }

    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn preview_auto_resize(mut self, preview_auto_resize: bool) -> Self {
        self.preview_auto_resize = preview_auto_resize;
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
