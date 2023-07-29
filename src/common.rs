use std::{collections::HashMap, fs::read};

use anyhow::anyhow;
use eframe::{
    egui::{
        self, widget_text::WidgetTextGalley, Context, FontData, FontDefinitions, Style, TextStyle,
        TextureOptions, Ui, WidgetText,
    },
    epaint::{vec2, ColorImage, FontFamily, Rect, TextureHandle, Vec2},
};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use paperdoll_tar::paperdoll::{factory::PaperdollFactory, image::ImageData};

pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub texture: TextureHandle,
}

pub(crate) fn allocate_size_center_in_rect(width: f32, height: f32, container_rect: &Rect) -> Rect {
    let origin_aspect_ratio = width / height;

    let mut width = width.min(container_rect.width());
    let mut height = height.min(container_rect.height());

    let new_aspect_ratio = width / height;

    if new_aspect_ratio < origin_aspect_ratio {
        height = width / origin_aspect_ratio;
    }

    if new_aspect_ratio > origin_aspect_ratio {
        width = height * origin_aspect_ratio;
    }

    let min = container_rect.center() - vec2(width * 0.5, height * 0.5);
    let max = min + vec2(width, height);

    Rect::from([min, max])
}

pub(crate) fn layout_text_widget(
    ui: &mut Ui,
    text: impl Into<WidgetText>,
    padding: Vec2,
) -> (WidgetTextGalley, Vec2) {
    let total_padding = padding + padding;

    let text: WidgetText = text.into();
    let text = text.into_galley(
        ui,
        None,
        ui.available_width() - total_padding.x,
        TextStyle::Button,
    );

    let widget_size = vec2(
        text.size().x + total_padding.x,
        (text.size().y + total_padding.y).max(ui.spacing().interact_size.y),
    );

    (text, widget_size)
}

pub(crate) fn load_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    add_font(
        &mut fonts,
        FontData::from_static(material_icons::FONT),
        "MaterialIcons-Regular",
    );

    let handle =
        SystemSource::new().select_best_match(&[FamilyName::SansSerif], &Properties::new());

    let buffer = match handle {
        Ok(handle) => match handle {
            Handle::Memory { bytes, .. } => Ok(bytes.to_vec()),
            Handle::Path { path, .. } => read(path).map_err(|e| anyhow!(e)),
        },
        Err(err) => Err(anyhow!(err)),
    };

    match buffer {
        Ok(buffer) => {
            add_font(
                &mut fonts,
                FontData::from_owned(buffer),
                "System Sans Serif",
            );
        }
        Err(err) => {
            log::warn!("Failed to load system fonts: {}.", err);
        }
    }

    ctx.set_fonts(fonts);
}

pub(crate) fn setup_style(ctx: &Context) {
    let mut style = Style::default();

    style.spacing.item_spacing.y = 8.0;

    // style.debug.debug_on_hover = true;

    ctx.set_style(style);
}

pub(crate) fn upload_image_to_texture(
    image: &ImageData,
    name: impl Into<String>,
    ctx: &Context,
) -> TextureData {
    let texture = ctx.load_texture(
        name,
        egui::ImageData::Color(ColorImage::from_rgba_unmultiplied(
            [image.width as usize, image.height as usize],
            &image.pixels,
        )),
        TextureOptions::NEAREST,
    );

    TextureData {
        width: image.width,
        height: image.height,
        texture,
    }
}

pub(crate) fn upload_ppd_textures(
    ppd: &PaperdollFactory,
    ctx: &Context,
) -> (HashMap<u32, TextureData>, HashMap<u32, TextureData>) {
    let mut textures_doll = HashMap::new();

    for (id, doll) in ppd.dolls() {
        if doll.image.is_empty() {
            continue;
        }

        let texture = upload_image_to_texture(&doll.image, "doll", ctx);

        textures_doll.insert(*id, texture);
    }

    let mut textures_fragment = HashMap::new();

    for (id, fragment) in ppd.fragments() {
        if fragment.image.is_empty() {
            continue;
        }

        let texture = upload_image_to_texture(&fragment.image, "fragment", ctx);

        textures_fragment.insert(*id, texture);
    }

    (textures_doll, textures_fragment)
}

fn add_font(fonts: &mut FontDefinitions, font: FontData, font_name: &str) {
    fonts.font_data.insert(font_name.to_owned(), font);

    if let Some(vec) = fonts.families.get_mut(&FontFamily::Proportional) {
        vec.push(font_name.to_owned());
    }

    if let Some(vec) = fonts.families.get_mut(&FontFamily::Monospace) {
        vec.push(font_name.to_owned());
    }
}
