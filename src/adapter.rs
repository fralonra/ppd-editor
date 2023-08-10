use std::collections::HashSet;

use eframe::epaint::TextureHandle;
use paperdoll_tar::paperdoll::{Doll, Fragment, ImageData, Point, Slot};

pub const DOLL_DEFAULT_SIZE: u32 = 400;

pub struct DollAdapter {
    pub desc: String,
    pub width: u32,
    pub height: u32,
    pub offset: Point,
    pub path: String,
    pub image: ImageAdapter,
}

impl Default for DollAdapter {
    fn default() -> Self {
        Self {
            desc: String::default(),
            width: DOLL_DEFAULT_SIZE,
            height: DOLL_DEFAULT_SIZE,
            offset: Point::default(),
            path: String::default(),
            image: ImageAdapter::default(),
        }
    }
}

impl From<&Doll> for DollAdapter {
    fn from(doll: &Doll) -> Self {
        Self {
            desc: doll.desc.clone(),
            width: doll.width,
            height: doll.height,
            offset: doll.offset,
            path: doll.path.clone(),
            image: (&doll.image).into(),
        }
    }
}

#[derive(Default)]
pub struct FragmentAdapter {
    pub desc: String,
    pub pivot: Point,

    pub path: String,
    pub image: ImageAdapter,
}

impl From<&Fragment> for FragmentAdapter {
    fn from(fragment: &Fragment) -> Self {
        Self {
            desc: fragment.desc.clone(),
            pivot: fragment.pivot,
            path: fragment.path.clone(),
            image: (&fragment.image).into(),
        }
    }
}

#[derive(Default)]
pub struct ImageAdapter {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub texture: Option<TextureHandle>,
}

impl From<&ImageData> for ImageAdapter {
    fn from(image: &ImageData) -> Self {
        Self {
            width: image.width,
            height: image.height,
            pixels: vec![],
            texture: None,
        }
    }
}

pub struct SlotAdapter {
    pub desc: String,
    pub required: bool,
    pub constrainted: bool,

    pub positions: Vec<Point>,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub keep_aspect_ratio: bool,

    pub anchor: Point,

    pub candidates: Vec<u32>,

    pub actived_position: Option<usize>,
    pub actived_candidate: Option<u32>,
    pub actived_fragments: HashSet<u32>,
    pub filtered_fragments: Vec<u32>,
    pub fragments_filter_keyword: String,
}

impl Default for SlotAdapter {
    fn default() -> Self {
        Self {
            desc: String::default(),
            required: false,
            constrainted: false,
            positions: vec![Point::default()],
            width: 50,
            height: 50,
            aspect_ratio: 1.0,
            keep_aspect_ratio: false,
            anchor: Point::default(),
            candidates: vec![],
            actived_position: None,
            actived_candidate: None,
            actived_fragments: HashSet::new(),
            filtered_fragments: vec![],
            fragments_filter_keyword: String::default(),
        }
    }
}

impl From<&Slot> for SlotAdapter {
    fn from(slot: &Slot) -> Self {
        Self {
            desc: slot.desc.clone(),
            required: slot.required,
            constrainted: slot.constrainted,
            positions: slot.positions.clone(),
            width: slot.width,
            height: slot.height,
            aspect_ratio: slot.width as f32 / slot.height as f32,
            anchor: slot.anchor,
            candidates: slot.candidates.clone(),
            ..Default::default()
        }
    }
}
