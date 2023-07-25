use anyhow::Result;
use eframe::{egui::Context, Frame};
use paperdoll_tar::paperdoll::factory::PaperdollFactory;

use crate::{common::upload_ppd_textures, fs::select_file};

use super::ViewerApp;

pub enum Action {
    AppQuit,
    FileOpen,
    PpdChanged(Option<PaperdollFactory>),
    SlotFragmentChanged(u32, isize),
}

impl ViewerApp {
    pub(super) fn handle_actions(&mut self, ctx: &Context, frame: &mut Frame) -> Result<()> {
        while let Some(action) = self.actions.pop_front() {
            match action {
                Action::AppQuit => frame.close(),
                Action::FileOpen => {
                    if let Some(path) = select_file() {
                        let ppd = paperdoll_tar::load(&path)?;

                        self.actions.push_back(Action::PpdChanged(Some(ppd)));
                    }
                }
                Action::PpdChanged(ppd) => {
                    if ppd.is_none() {
                        self.ppd = None;

                        continue;
                    }

                    let ppd = ppd.unwrap();

                    self.actived_doll = 0;

                    self.slot_map.clear();
                    self.slot_index_map.clear();

                    for (id, slot) in ppd.slots() {
                        self.slot_index_map.insert(*id, -1);

                        if !slot.required {
                            continue;
                        }

                        if let Some(fragment_id) = slot.candidates.first() {
                            self.slot_map.insert(*id, *fragment_id);
                            self.slot_index_map.insert(*id, 0);
                        }
                    }

                    let (textures_doll, textures_fragment) = upload_ppd_textures(&ppd, ctx);

                    self.textures_doll = textures_doll;
                    self.textures_fragment = textures_fragment;

                    self.ppd = Some(ppd);
                }
                Action::SlotFragmentChanged(slot_id, candidate_index) => {
                    if let Some(ppd) = &self.ppd {
                        if candidate_index >= 0 {
                            if let Some(slot) = ppd.get_slot(slot_id) {
                                if let Some(fragment_id) =
                                    slot.candidates.iter().nth(candidate_index as usize)
                                {
                                    self.slot_map.insert(slot_id, *fragment_id);

                                    continue;
                                }
                            }
                        }

                        self.slot_map.remove(&slot_id);
                    }
                }
            }
        }

        Ok(())
    }
}
