mod actions;
mod shortcut;
mod ui;

use std::collections::{HashMap, VecDeque};

use eframe::{egui::Context, App, CreationContext, Frame};
use paperdoll_tar::paperdoll::factory::PaperdollFactory;

use crate::common::{load_fonts, setup_style, TextureData};

use self::{actions::Action, shortcut::Shortcut};

pub const APP_TITLE: &'static str = "Paperdoll Viewer";

struct ViewerApp {
    actions: VecDeque<Action>,
    shortcut: Shortcut,

    ppd: Option<PaperdollFactory>,

    textures_doll: HashMap<u32, TextureData>,
    textures_fragment: HashMap<u32, TextureData>,

    actived_doll: u32,
    slot_map: HashMap<u32, u32>,
    slot_index_map: HashMap<u32, isize>,
}

impl App for ViewerApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.ui(ctx);

        if let Err(err) = self.handle_actions(ctx, frame) {
            log::error!("{}", err);
        }
    }
}

impl ViewerApp {
    pub fn new(_cc: &CreationContext<'_>, ppd: Option<PaperdollFactory>) -> Self {
        Self {
            actions: VecDeque::from([Action::PpdChanged(ppd)]),
            shortcut: Shortcut::default(),

            ppd: None,

            textures_doll: HashMap::new(),
            textures_fragment: HashMap::new(),

            actived_doll: 0,
            slot_map: HashMap::new(),
            slot_index_map: HashMap::new(),
        }
    }
}

pub fn setup_eframe(cc: &CreationContext<'_>, ppd: Option<PaperdollFactory>) -> Box<dyn App> {
    load_fonts(&cc.egui_ctx);

    setup_style(&cc.egui_ctx);

    Box::new(ViewerApp::new(cc, ppd))
}
