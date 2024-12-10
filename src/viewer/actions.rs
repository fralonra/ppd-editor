use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, Result};
use eframe::{egui::Context, epaint::Vec2, Frame};
use paperdoll_tar::paperdoll::PaperdollFactory;

use crate::{
    common::{allocate_size_fit_in_rect, upload_image_to_texture},
    fs::{export_texture, select_file},
};

use super::{ViewerApp, APP_TITLE};

pub enum Action {
    AppQuit,
    AppTitleChanged(Option<String>),
    DollChanged,
    Export,
    FileOpen,
    FileOpenPath(PathBuf),
    OpenEditor,
    PpdChanged(Option<PaperdollFactory>),
    RecentFilesClean,
    SlotFragmentChanged(u32, isize),
    TextureUpdate,
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
                Action::AppTitleChanged(title) => {
                    let title = title
                        .map(|path| format!("{} - {}", APP_TITLE, path))
                        .unwrap_or(APP_TITLE.to_owned());

                    frame.set_window_title(&title)
                }
                Action::DollChanged => {
                    self.actions.push_back(Action::TextureUpdate);
                }
                Action::Export => {
                    let Some(ppd) = &self.ppd else {
                        continue;
                    };

                    if let Some(path) = export_texture(&format!("{}.png", ppd.meta.name)) {
                        let image = ppd.render_paperdoll(&self.paperdoll)?;
                        image::save_buffer(
                            path,
                            &image.pixels,
                            image.width,
                            image.height,
                            image::ColorType::Rgba8,
                        )?;
                    }
                }
                Action::FileOpen => {
                    if let Some(path) = select_file() {
                        self.load_ppd_from_path(&path)?;

                        self.actions.push_back(Action::AppTitleChanged(Some(
                            path.to_string_lossy().to_string(),
                        )));

                        self.storage.recent_files.push(path);
                    }
                }
                Action::FileOpenPath(path) => {
                    self.load_ppd_from_path(&path)?;

                    self.actions.push_back(Action::AppTitleChanged(Some(
                        path.to_string_lossy().to_string(),
                    )));

                    self.storage.recent_files.push(path);
                }
                Action::OpenEditor => {
                    if let Some(path) = &self.config.file_path {
                        #[cfg(not(feature = "flatpak"))]
                        Command::new(crate::editor::APP_CMD)
                            .args(&["-o", &path.to_string_lossy().to_string()])
                            .spawn()
                            .map_err(|e| anyhow!(e))?;

                        #[cfg(feature = "flatpak")]
                        Command::new("flatpak")
                            .args(&["run", crate::editor::APP_ID])
                            .spawn()
                            .map_err(|e| anyhow!(e))?;
                    }
                }
                Action::PpdChanged(ppd) => {
                    let Some(ppd) = ppd else {
                        self.ppd = None;

                        continue;
                    };

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

                    self.actions.push_back(Action::TextureUpdate);

                    self.ppd = Some(ppd);
                }
                Action::RecentFilesClean => {
                    self.storage.recent_files.clear();
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

                        self.actions.push_back(Action::TextureUpdate);
                    }
                }
                Action::TextureUpdate => {
                    if let Some(ppd) = &self.ppd {
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

    fn load_ppd_from_path<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let ppd = paperdoll_tar::load(&path)?;

        self.actions.push_back(Action::PpdChanged(Some(ppd)));

        self.config.file_path = Some(path.as_ref().to_path_buf());

        Ok(())
    }
}
