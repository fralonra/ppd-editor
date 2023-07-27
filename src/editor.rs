mod actions;
mod canvas;
mod config;
mod menu;
mod shortcut;
mod ui;
mod widgets;

use std::collections::{HashMap, VecDeque};

use eframe::{
    egui::Context,
    epaint::{
        ahash::{HashSet, HashSetExt},
        Pos2,
    },
    App, CreationContext, Frame,
};
use paperdoll_tar::paperdoll::factory::PaperdollFactory;

use crate::{
    adapter::{DollAdapter, FragmentAdapter, SlotAdapter},
    common::{load_fonts, setup_style, TextureData},
};

use self::{actions::Action, config::Config, shortcut::Shortcut};

pub const APP_TITLE: &'static str = "Paperdoll Editor";

struct DialogOption {
    pub text: String,
    pub primary_text: String,
    pub secondary_text: Option<String>,
    pub tertiary_text: Option<String>,

    pub primary_action: Option<Action>,
    pub secondary_action: Option<Action>,
    pub tertiary_action: Option<Action>,
}

impl Default for DialogOption {
    fn default() -> Self {
        Self {
            text: "Close this".to_owned(),
            primary_text: "Close".to_owned(),
            secondary_text: None,
            tertiary_text: None,
            primary_action: None,
            secondary_action: None,
            tertiary_action: None,
        }
    }
}

impl DialogOption {
    fn confirm(text: &str) -> Self {
        Self::default()
            .text(text)
            .primary_text("Confirm")
            .secondary_text("Cancel")
    }

    fn primary_action(mut self, primary_action: Action) -> Self {
        self.primary_action = Some(primary_action);
        self
    }

    fn primary_text(mut self, primary_text: impl Into<String>) -> Self {
        self.primary_text = primary_text.into();
        self
    }

    fn secondary_action(mut self, secondary_action: Action) -> Self {
        self.secondary_action = Some(secondary_action);
        self
    }

    fn secondary_text(mut self, secondary_text: impl Into<String>) -> Self {
        self.secondary_text = Some(secondary_text.into());
        self
    }

    fn tertiary_action(mut self, tertiary_action: Action) -> Self {
        self.tertiary_action = Some(tertiary_action);
        self
    }

    fn tertiary_text(mut self, tertiary_text: impl Into<String>) -> Self {
        self.tertiary_text = Some(tertiary_text.into());
        self
    }

    fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

#[derive(Default)]
struct EditorApp {
    // editor core
    actions: VecDeque<Action>,
    config: Config,
    shortcut: Shortcut,

    // project core
    ppd: PaperdollFactory,

    // textures
    textures_doll: HashMap<u32, TextureData>,
    textures_fragment: HashMap<u32, TextureData>,

    // editor helpers
    fragments_filter_keyword: String,
    cursor_position: Option<Pos2>,
    associated_slots: HashSet<u32>,
    locked_slots: HashSet<u32>,
    visible_slots: HashSet<u32>,
    slot_copy: Option<u32>,

    // adapters
    adapter_doll: Option<DollAdapter>,
    adapter_fragment: Option<FragmentAdapter>,
    adapter_slot: Option<SlotAdapter>,

    // selected ids
    actived_doll: Option<u32>,
    actived_fragment: Option<u32>,
    actived_slot: Option<u32>,

    // dialog
    dialog_visible: bool,
    dialog_option: DialogOption,

    // window visible
    window_associated_slots_visible: bool,
    window_doll_visible: bool,
    window_fragment_visible: bool,
    window_slot_visible: bool,

    // window status
    window_doll_error: Option<String>,
    window_fragment_error: Option<String>,
    window_slot_error: Option<String>,
}

impl App for EditorApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.ui(ctx);

        if let Err(err) = self.handle_actions(ctx, frame) {
            log::error!("{}", err);
        }
    }
}

impl EditorApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut ppd = PaperdollFactory::default();

        if let Some(doll) = ppd.get_doll_mut(0) {
            doll.width = 400;
            doll.height = 400;
        }

        Self::from_ppd(cc, ppd)
    }

    pub fn from_ppd(_cc: &CreationContext<'_>, ppd: PaperdollFactory) -> Self {
        Self {
            actions: VecDeque::from([Action::PpdChanged, Action::AppTitleChanged(None)]),
            config: Config::default(),
            shortcut: Shortcut::default(),

            ppd,

            textures_doll: HashMap::new(),
            textures_fragment: HashMap::new(),

            fragments_filter_keyword: String::default(),
            cursor_position: None,
            associated_slots: HashSet::new(),
            locked_slots: HashSet::new(),
            visible_slots: HashSet::new(),
            slot_copy: None,

            adapter_doll: None,
            adapter_fragment: None,
            adapter_slot: None,

            actived_doll: None,
            actived_fragment: None,
            actived_slot: None,

            dialog_visible: false,
            dialog_option: DialogOption::default(),

            window_associated_slots_visible: false,
            window_doll_visible: false,
            window_fragment_visible: false,
            window_slot_visible: false,

            window_doll_error: None,
            window_fragment_error: None,
            window_slot_error: None,
        }
    }
}

pub fn setup_eframe(cc: &CreationContext<'_>) -> Box<dyn App> {
    load_fonts(&cc.egui_ctx);

    setup_style(&cc.egui_ctx);

    Box::new(EditorApp::new(cc))
}
