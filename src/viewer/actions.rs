use anyhow::Result;
use eframe::{egui::Context, epaint::Vec2, Frame};
use paperdoll_tar::paperdoll::factory::PaperdollFactory;

use crate::{
    common::{allocate_size_fit_in_rect, upload_image_to_texture},
    fs::select_file,
};

use super::ViewerApp;

pub enum Action {
    AppQuit,
    FileOpen,
    PpdChanged(Option<PaperdollFactory>),
    SlotFragmentChanged(u32, isize),
    ViewportCenter,
    ViewportFit,
    ViewportMove(Vec2),
    ViewportZoomReset,
    ViewportZoomTo(f32),
    WindowAboutVisible(bool),
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

                    self.paperdoll.doll = 0;
                    self.paperdoll.slot_map.clear();

                    self.slot_index_map.clear();

                    for (id, slot) in ppd.slots() {
                        self.slot_index_map.insert(*id, -1);

                        if !slot.required {
                            continue;
                        }

                        if let Some(fragment_id) = slot.candidates.first() {
                            self.paperdoll.slot_map.insert(*id, *fragment_id);
                            self.slot_index_map.insert(*id, 0);
                        }
                    }

                    if let Ok(image) = ppd.render_paperdoll(&self.paperdoll) {
                        self.texture = Some(upload_image_to_texture(&image, &ppd.meta.name, ctx));
                    }

                    self.ppd = Some(ppd);
                }
                Action::SlotFragmentChanged(slot_id, candidate_index) => {
                    if let Some(ppd) = &self.ppd {
                        if candidate_index >= 0 {
                            if let Some(slot) = ppd.get_slot(slot_id) {
                                if let Some(fragment_id) =
                                    slot.candidates.iter().nth(candidate_index as usize)
                                {
                                    self.paperdoll.slot_map.insert(slot_id, *fragment_id);
                                }
                            }
                        } else {
                            self.paperdoll.slot_map.remove(&slot_id);
                        }

                        if let Ok(image) = ppd.render_paperdoll(&self.paperdoll) {
                            self.texture =
                                Some(upload_image_to_texture(&image, &ppd.meta.name, ctx));
                        }
                    }
                }
                Action::ViewportCenter => {
                    self.viewport.offset = Vec2::ZERO;
                }
                Action::ViewportFit => {
                    let doll = self
                        .ppd
                        .as_ref()
                        .map(|ppd| ppd.get_doll(self.paperdoll.doll))
                        .flatten();

                    if let Some(doll) = doll {
                        let doll_rect = allocate_size_fit_in_rect(
                            doll.width as f32,
                            doll.height as f32,
                            &self.viewport.rect,
                        );

                        self.viewport.offset = Vec2::ZERO;

                        self.viewport.scale = doll_rect.width() / doll.width as f32;
                    }
                }
                Action::ViewportMove(offset) => self.viewport.offset += offset,
                Action::ViewportZoomReset => self.viewport.scale = 1.0,
                Action::ViewportZoomTo(scale) => {
                    if scale > 0.1 && scale < 10.0 {
                        self.viewport.scale = scale;
                    }
                }
                Action::WindowAboutVisible(visible) => {
                    self.window_about_visible = visible;
                }
            }
        }

        Ok(())
    }
}
