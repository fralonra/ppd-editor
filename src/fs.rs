use std::{
    fs::write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use paperdoll_tar::{
    paperdoll::image::{ColorType, ImageData},
    EXTENSION_NAME,
};
use rfd::FileDialog;

pub fn create_file(filename: &str) -> Option<PathBuf> {
    FileDialog::new()
        .set_directory("~")
        .add_filter("Paperdoll File", &[EXTENSION_NAME])
        .set_file_name(filename)
        .save_file()
}

pub fn open_image_rgba<P>(path: P) -> Result<ImageData>
where
    P: AsRef<Path>,
{
    let image = image::open(path)?;

    let image = image.into_rgba8();

    Ok(ImageData {
        width: image.width(),
        height: image.height(),
        color_type: ColorType::Rgba,
        pixels: image.into_vec(),
    })
}

pub fn select_file() -> Option<PathBuf> {
    FileDialog::new()
        .set_directory("~")
        .add_filter("Paperdoll File", &[EXTENSION_NAME])
        .pick_file()
}

pub fn select_texture() -> Option<PathBuf> {
    FileDialog::new()
        .set_directory("~")
        .add_filter("Images", &["png", "jpg", "webp"])
        .pick_file()
}

pub fn select_textures() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .set_directory("~")
        .add_filter("Images", &["png", "jpg", "webp"])
        .pick_files()
}

pub fn write_file<P, C>(path: P, contents: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    Ok(write(path, contents)?)
}
