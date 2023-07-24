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

#[derive(Default)]
pub struct DollAdapter {
    pub desc: String,
    pub width: u32,
    pub height: u32,
    pub offset: Point,
    pub path: String,
    pub image: ImageAdapter,
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

#[derive(Default)]
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
