use eframe::{
    egui::{Response, Sense, TextStyle, Ui, Widget, WidgetText},
    epaint::{pos2, vec2, Color32, Rect, Vec2},
};
use material_icons::{icon_to_char, Icon};
use paperdoll_tar::paperdoll::Fragment;

use crate::common::{allocate_size_center_in_rect, layout_text_widget, TextureData};

pub struct FragmentEntry<'a> {
    fragment: &'a Fragment,
    actived: bool,
    texture: Option<&'a TextureData>,
}

impl<'a> Widget for FragmentEntry<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let text = if self.fragment.desc.is_empty() {
                WidgetText::from(format!("Unnamed Fragment - {}", self.fragment.id()))
            } else {
                WidgetText::from(&self.fragment.desc)
            };

            let preview_rect_size = Vec2::splat(ui.available_height());

            let (rect_preview, response_preview) =
                ui.allocate_at_least(preview_rect_size, Sense::click());

            let padding = ui.spacing().button_padding;

            let (text, desired_size) = layout_text_widget(ui, text, padding);

            let (rect_text, response_text) = ui.allocate_at_least(
                vec2(ui.available_width().max(desired_size.x), desired_size.y),
                Sense::click(),
            );

            let response = response_text.union(response_preview);
            let rect = response.rect;

            let visuals = ui.style().interact_selectable(&response, self.actived);

            if self.actived {
                let rect = rect.expand(visuals.expansion);

                ui.painter()
                    .rect(rect, 0.0, visuals.weak_bg_fill, visuals.bg_stroke);
            }

            if let Some(texture) = self.texture {
                let rect_preview = rect_preview.shrink(2.0);

                ui.painter_at(rect_preview).image(
                    texture.texture.id(),
                    allocate_size_center_in_rect(
                        texture.width as f32,
                        texture.height as f32,
                        &rect_preview,
                    ),
                    Rect::from([pos2(0.0, 0.0), pos2(1.0, 1.0)]),
                    Color32::WHITE,
                )
            } else {
                let padding = 4.0;

                let text = ui.painter().layout(
                    icon_to_char(Icon::BrokenImage).to_string(),
                    ui.style()
                        .text_styles
                        .get(&TextStyle::Button)
                        .map(|font| font.clone())
                        .unwrap_or_default(),
                    ui.visuals().text_color(),
                    preview_rect_size.x - padding - padding,
                );

                ui.painter()
                    .galley(rect_preview.min + Vec2::splat(padding), text);
            }

            text.paint_with_visuals(&ui.painter_at(rect_text), rect_text.min + padding, &visuals);

            response
        })
        .inner
    }
}

impl<'a> FragmentEntry<'a> {
    pub fn new(fragment: &'a Fragment) -> Self {
        Self {
            fragment,
            actived: false,
            texture: None,
        }
    }

    pub fn actived(mut self, actived: bool) -> Self {
        self.actived = actived;
        self
    }

    pub fn texture(mut self, texture: Option<&'a TextureData>) -> Self {
        self.texture = texture;
        self
    }
}
