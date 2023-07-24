mod actions;
mod canvas;
mod config;
mod menu;
mod shortcut;
mod ui;
mod widgets;

use std::collections::HashMap;

use eframe::{
    egui::Context,
    epaint::ahash::{HashSet, HashSetExt},
    App, CreationContext, Frame,
};
use paperdoll_tar::paperdoll::factory::PaperdollFactory;

use crate::{
    adapter::{DollAdapter, FragmentAdapter, SlotAdapter},
    common::{load_fonts, setup_style, TextureData},
};

use self::{actions::Action, config::Config, shortcut::Shortcut};

pub const APP_TITLE: &'static str = "Paperdoll Editor";

#[derive(Default)]
struct EditorApp {
    actions: Vec<Action>,
    config: Config,
    shortcut: Shortcut,

    ppd: PaperdollFactory,

    textures_doll: HashMap<u32, TextureData>,
    textures_fragment: HashMap<u32, TextureData>,

    fragments_filter_keyword: String,
    locked_slots: HashSet<u32>,
    visible_slots: HashSet<u32>,

    adapter_doll: Option<DollAdapter>,
    adapter_fragment: Option<FragmentAdapter>,
    adapter_slot: Option<SlotAdapter>,

    actived_doll: Option<u32>,
    actived_fragment: Option<u32>,
    actived_slot: Option<u32>,

    window_doll_visible: bool,
    window_fragment_visible: bool,
    window_slot_visible: bool,
}

impl App for EditorApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.ui(ctx);

        self.handle_actions(ctx, frame);
    }
}

impl EditorApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let ppd = PaperdollFactory::default();

        Self::from_ppd(cc, ppd)
    }

    pub fn from_ppd(_cc: &CreationContext<'_>, ppd: PaperdollFactory) -> Self {
        Self {
            actions: vec![Action::PpdChanged, Action::AppTitleChanged(None)],
            config: Config::default(),
            shortcut: Shortcut::default(),

            ppd,

            textures_doll: HashMap::new(),
            textures_fragment: HashMap::new(),

            fragments_filter_keyword: String::default(),
            locked_slots: HashSet::new(),
            visible_slots: HashSet::new(),

            adapter_doll: None,
            adapter_fragment: None,
            adapter_slot: None,

            actived_doll: None,
            actived_fragment: None,
            actived_slot: None,

            window_doll_visible: false,
            window_fragment_visible: false,
            window_slot_visible: false,
        }
    }
}

pub fn setup_eframe(cc: &CreationContext<'_>) -> Box<dyn App> {
    load_fonts(&cc.egui_ctx);

    setup_style(&cc.egui_ctx);

    Box::new(EditorApp::new(cc))
}
