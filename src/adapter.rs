use eframe::epaint::TextureHandle;
use paperdoll_tar::paperdoll::{
    common::Point, doll::Doll, fragment::Fragment, image::ImageData, slot::Slot,
};

#[derive(Default, PartialEq)]
pub enum FragmentFilter {
    #[default]
    All,
    IsCandidate,
    IsNotCandidate,
}

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
            width: 400,
            height: 400,
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

    pub position: Point,
    pub width: u32,
    pub height: u32,

    pub anchor: Point,

    pub candidates: Vec<u32>,

    pub actived_fragments: Vec<u32>,
    pub fragments_filter: FragmentFilter,
    pub fragments_filter_keyword: String,
}

impl Default for SlotAdapter {
    fn default() -> Self {
        Self {
            desc: String::default(),
            required: false,
            constrainted: false,
            position: Point::default(),
            width: 50,
            height: 50,
            anchor: Point::default(),
            candidates: vec![],
            actived_fragments: vec![],
            fragments_filter: FragmentFilter::default(),
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
            position: slot.position,
            width: slot.width,
            height: slot.height,
            anchor: slot.anchor,
            candidates: slot.candidates.clone(),
            actived_fragments: vec![],
            fragments_filter: FragmentFilter::default(),
            fragments_filter_keyword: String::default(),
        }
    }
}
