mod actions;
mod config;
mod menu;
mod shortcut;
mod storage;
mod ui;

use std::collections::{HashMap, VecDeque};

use eframe::{egui::Context, App, CreationContext, Frame};
use paperdoll_tar::paperdoll::{Paperdoll, PaperdollFactory};
use which::which;

use crate::{
    common::{load_fonts, setup_style, TextureData},
    viewport::Viewport,
};

use self::{actions::Action, config::Config, shortcut::Shortcut, storage::Storage};

pub const APP_CMD: &'static str = "ppd-viewer";
pub const APP_ID: &'static str = "io.github.fralonra.PpdViewer";
pub const APP_TITLE: &'static str = "Paperdoll Viewer";

struct ViewerApp {
    actions: VecDeque<Action>,
    config: Config,
    shortcut: Shortcut,
    storage: Storage,
    viewport: Viewport,

    ppd: Option<PaperdollFactory>,
    paperdoll: Paperdoll,

    texture: Option<TextureData>,

    slot_index_map: HashMap<u32, isize>,

    window_about_visible: bool,

    has_editor_installed: bool,
}

impl App for ViewerApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.ui(ctx);

        self.handle_shortcut(ctx);

        if let Err(err) = self.handle_actions(ctx, frame) {
            log::error!("{}", err);
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        match self.storage.save(storage) {
            Ok(()) => log::info!("Saving app data successfully."),
            Err(err) => log::error!("Saving app data failed: {}", err),
        }
    }
}

impl ViewerApp {
    pub fn new(
        cc: &CreationContext<'_>,
        ppd: Option<PaperdollFactory>,
        path: Option<String>,
    ) -> Self {
        let mut storage = Storage::default();

        if let Some(s) = cc.storage {
            match storage.restore(s) {
                Ok(()) => log::info!("Restoring app data successfully."),
                Err(err) => log::error!("Restoring app data failed: {}", err),
            }
        }

        let mut config = Config::default();
        if let Some(path) = &path {
            config.file_path = Some(path.into());
        }

        let has_editor_installed = which(crate::editor::APP_CMD).is_ok();

        Self {
            actions: VecDeque::from([Action::PpdChanged(ppd), Action::AppTitleChanged(path)]),
            config,
            shortcut: Shortcut::default(),
            storage,
            viewport: Viewport::default(),

            ppd: None,
            paperdoll: Paperdoll {
                doll: 0,
                slot_map: HashMap::new(),
            },

            texture: None,

            slot_index_map: HashMap::new(),

            window_about_visible: false,

            has_editor_installed,
        }
    }
}

pub fn setup_eframe(
    cc: &CreationContext<'_>,
    ppd: Option<PaperdollFactory>,
    path: Option<String>,
) -> Box<dyn App> {
    load_fonts(&cc.egui_ctx);

    setup_style(&cc.egui_ctx);

    Box::new(ViewerApp::new(cc, ppd, path))
}
