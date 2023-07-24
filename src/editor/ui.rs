use eframe::{
    egui::{
        Button, CentralPanel, Context, DragValue, Grid, Layout, ScrollArea, Sense, SidePanel,
        TextEdit, TopBottomPanel, Ui, Window,
    },
    emath::{Align, Align2},
    epaint::Color32,
};
use material_icons::{icon_to_char, Icon};
use paperdoll_tar::paperdoll::common::Point;

use crate::{adapter::FragmentFilter, common::TextureData};

use super::{
    actions::Action,
    widgets::{Card, ImageUpload, Modal, PivotSelect, SlotEntry, Tooltip},
    EditorApp,
};

impl EditorApp {
    pub(super) fn ui(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            self.ui_menu_bar(ui);
        });

        TopBottomPanel::bottom("status").show(ctx, |ui| {
            self.ui_status_bar(ui);
        });

        SidePanel::left("left").min_width(240.0).show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.ui_left_panel(ui);
                });
        });

        SidePanel::right("right").min_width(300.0).show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.ui_right_panel(ui);
                });
        });

        CentralPanel::default().show(ctx, |ui| {
            self.ui_canvas(ui);
        });

        self.ui_doll_window(ctx);

        self.ui_slot_window(ctx);

        self.ui_fragment_window(ctx);
    }

    fn ui_doll(&mut self, ui: &mut Ui) {
        let doll = self
            .actived_doll
            .map(|id| self.ppd.get_doll_mut(id))
            .flatten();

        if let Some(doll) = doll {
            let id = doll.id();

            ui.heading(format!("Doll_{:?}", id));

            Grid::new("doll")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Description:");
                    ui.text_edit_singleline(&mut doll.desc);

                    ui.end_row();

                    ui.horizontal_centered(|ui| {
                        ui.label("Size:");
                        ui.add(Tooltip::new("The size of the doll."))
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.monospace("w");
                        ui.add(DragValue::new(&mut doll.width).speed(1));

                        ui.monospace("h");
                        ui.add(DragValue::new(&mut doll.height).speed(1));
                    });

                    ui.end_row();

                    ui.horizontal_centered(|ui| {
                        ui.label("Offset:");
                        ui.add(Tooltip::new(
                        "Offset pixels of the top left position of the background image, if any.",
                    ))
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.monospace("x");
                        ui.add(DragValue::new(&mut doll.offset.x).speed(1));

                        ui.monospace("y");
                        ui.add(DragValue::new(&mut doll.offset.y).speed(1));
                    });

                    ui.end_row();

                    ui.horizontal_centered(|ui| {
                        ui.label("Background:");
                        ui.add(Tooltip::new("The background of the doll. It's optional."))
                    });

                    let mut request_edit = false;
                    let mut request_remove = false;

                    if ui
                        .add(
                            ImageUpload::new(self.textures_doll.get(&id))
                                .on_edit(|| {
                                    request_edit = true;
                                })
                                .on_remove(|| {
                                    request_remove = true;
                                }),
                        )
                        .clicked()
                    {
                        self.actions.push(Action::DollBackgroundUpload(id));
                    }

                    if request_edit {
                        self.actions.push(Action::DollBackgroundUpload(id));
                    }

                    if request_remove {
                        self.actions.push(Action::DollBackgroundRemove(id));
                    }
                });

            ui.separator();

            ui.label("Slots");

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(icon_to_char(Icon::Add).to_string())
                    .on_hover_text("New slot")
                    .clicked()
                {
                    self.actions.push(Action::SlotCreate);
                }

                ui.add_enabled_ui(self.actived_slot.is_some(), |ui| {
                    if ui
                        .button(icon_to_char(Icon::Edit).to_string())
                        .on_hover_text("Edit slot")
                        .clicked()
                    {
                        if let Some(slot_id) = self.actived_slot {
                            self.actions.push(Action::SlotEdit(slot_id));
                        }
                    }

                    if ui
                        .button(icon_to_char(Icon::Delete).to_string())
                        .on_hover_text("Delete slot")
                        .clicked()
                    {
                        if let Some(slot_id) = self.actived_slot {
                            self.actions.push(Action::SlotRemove(slot_id));
                        }
                    }

                    if ui
                        .add_enabled(
                            self.actived_slot.map_or(false, |slot_id| {
                                doll.slots
                                    .iter()
                                    .position(|v| *v == slot_id)
                                    .map_or(false, |i| i > 0)
                            }),
                            Button::new(icon_to_char(Icon::ExpandLess).to_string()),
                        )
                        .on_hover_ui(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Raise slot");

                                ui.horizontal(|ui| {
                                    ui.strong("Shift");

                                    ui.label("Raise to the top");
                                });
                            });
                        })
                        .clicked()
                    {
                        if let Some(slot_id) = self.actived_slot {
                            self.actions
                                .push(if ui.input(|input| input.modifiers.shift) {
                                    Action::SlotRaiseTop(id, slot_id)
                                } else {
                                    Action::SlotRaise(id, slot_id)
                                });
                        }
                    }

                    if ui
                        .add_enabled(
                            self.actived_slot.map_or(false, |slot_id| {
                                doll.slots
                                    .iter()
                                    .position(|v| *v == slot_id)
                                    .map_or(false, |i| i < doll.slots.len() - 1)
                            }),
                            Button::new(icon_to_char(Icon::ExpandMore).to_string()),
                        )
                        .on_hover_ui(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Lower slot");

                                ui.horizontal(|ui| {
                                    ui.strong("Shift");

                                    ui.label("Lower to the bottom");
                                });
                            });
                        })
                        .clicked()
                    {
                        if let Some(slot_id) = self.actived_slot {
                            self.actions
                                .push(if ui.input(|input| input.modifiers.shift) {
                                    Action::SlotLowerBottom(id, slot_id)
                                } else {
                                    Action::SlotLower(id, slot_id)
                                });
                        }
                    }
                });
            });

            let slots = doll.slots.clone();

            ui.group(|ui| {
                ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            ui.spacing_mut().button_padding.y = 4.0;

                            for id in slots {
                                if let Some(slot) = self.ppd.get_slot(id) {
                                    let is_actived = self
                                        .actived_slot
                                        .map_or(false, |actived_slot| actived_slot == id);

                                    ui.horizontal(|ui| {
                                        let is_visible = self.visible_slots.contains(&id);
                                        let is_locked = self.locked_slots.contains(&id);

                                        if ui
                                            .button(
                                                icon_to_char(if is_visible {
                                                    Icon::Visibility
                                                } else {
                                                    Icon::VisibilityOff
                                                })
                                                .to_string(),
                                            )
                                            .on_hover_text(
                                                "Change visibility of the slot in editor",
                                            )
                                            .clicked()
                                        {
                                            if is_visible {
                                                self.visible_slots.remove(&id);
                                            } else {
                                                self.visible_slots.insert(id);
                                            }
                                        }

                                        if ui
                                            .button(
                                                icon_to_char(if is_locked {
                                                    Icon::Lock
                                                } else {
                                                    Icon::LockOpen
                                                })
                                                .to_string(),
                                            )
                                            .on_hover_text(
                                                "Lock the slot to prevent it from being dragged",
                                            )
                                            .clicked()
                                        {
                                            if is_locked {
                                                self.locked_slots.remove(&id);
                                            } else {
                                                self.locked_slots.insert(id);
                                            }
                                        }

                                        let resp = ui
                                            .add(SlotEntry::new(slot).actived(is_actived))
                                            .context_menu(|ui| {
                                                if ui.button("Edit slot").clicked() {
                                                    self.actions.push(Action::SlotEdit(id));

                                                    ui.close_menu();
                                                }

                                                if ui.button("Delete slot").clicked() {
                                                    self.actions.push(Action::SlotRemove(id));

                                                    ui.close_menu();
                                                }
                                            });

                                        if resp.clicked() {
                                            self.actived_slot = Some(id);
                                        }

                                        if resp.double_clicked() {
                                            self.actions.push(Action::SlotEdit(id));
                                        }
                                    });
                                }
                            }

                            if ui
                                .allocate_response(ui.available_size(), Sense::click())
                                .clicked()
                            {
                                self.actived_slot = None;
                            }
                        });
                    });
            });
        } else {
            self.ui_doll_empty(ui);
        }
    }

    fn ui_doll_empty(&mut self, ui: &mut Ui) {}

    fn ui_doll_window(&mut self, ctx: &Context) {
        if !self.window_doll_visible {
            return;
        }

        if self.adapter_doll.is_none() {
            return;
        }

        let id = self.actived_doll;

        let title = id.map_or("Create New Doll".to_owned(), |id| format!("Doll - {}", id));

        Modal::new("doll_window").show(ctx, |ctx| {
            Window::new(title)
                .pivot(Align2::CENTER_CENTER)
                .default_pos(ctx.screen_rect().center())
                .resizable(false)
                .open(&mut self.window_doll_visible)
                .show(ctx, |ui| {
                    ui.heading("Doll");

                    Grid::new("doll").num_columns(2).striped(true).show(ui, |ui| {
                        let adapter_doll = self.adapter_doll.as_mut().unwrap();

                        ui.label("Description:");
                        ui.text_edit_singleline(&mut adapter_doll.desc);

                        ui.end_row();

                        ui.horizontal_centered(|ui| {
                            ui.label("Size:");
                            ui.add(Tooltip::new("The size of the doll."))
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.monospace("w");
                            ui.add(DragValue::new(&mut adapter_doll.width).speed(1));

                            ui.monospace("h");
                            ui.add(DragValue::new(&mut adapter_doll.height).speed(1));
                        });

                        ui.end_row();

                        ui.horizontal_centered(|ui| {
                            ui.label("Offset:");
                            ui.add(Tooltip::new("Offset pixels of the top left position of the background image, if any."))
                        });
                        ui.horizontal_wrapped(|ui| {
                            ui.monospace("x");
                            ui.add(DragValue::new(&mut adapter_doll.offset.x).speed(1));

                            ui.monospace("y");
                            ui.add(DragValue::new(&mut adapter_doll.offset.y).speed(1));
                        });

                        ui.end_row();

                        ui.horizontal_centered(|ui| {
                            ui.label("Background:");
                            ui.add(Tooltip::new("The background of the doll. It's optional."))
                        });

                        let texture =
                            adapter_doll
                                .image
                                .texture
                                .as_ref()
                                .map(|texture| TextureData {
                                    width: adapter_doll.image.width,
                                    height: adapter_doll.image.height,
                                    texture: texture.clone(),
                                });

                        let mut request_edit = false;
                        let mut request_remove = false;

                        if ui
                            .add(
                                ImageUpload::new(texture.as_ref())
                                    .on_edit(|| {
                                        request_edit = true;
                                    })
                                    .on_remove(|| {
                                        request_remove = true;
                                    }),
                            )
                            .clicked()
                        {
                            self.actions.push(Action::DollAdapterBackgroundUpload);
                        }

                        if request_edit {
                            self.actions.push(Action::DollAdapterBackgroundUpload);
                        }

                        if request_remove {
                            self.actions.push(Action::DollAdapterBackgroundRemove);
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            self.actions.push(Action::DollEditConfirm(id));

                            self.actions.push(Action::WindowDollVisible(false));
                        }

                        if ui.button("Cancel").clicked() {
                            self.actions.push(Action::WindowDollVisible(false));
                        }
                    });
                })
        });
    }

    fn ui_dolls(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_height(ui.available_height() * 0.3);

            ui.heading("Dolls");

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(icon_to_char(Icon::Add).to_string())
                    .on_hover_text("New doll")
                    .clicked()
                {
                    self.actions.push(Action::DollCreate);
                }

                if ui
                    .add_enabled(
                        self.ppd.dolls().len() > 1 && self.actived_doll.is_some(),
                        Button::new(icon_to_char(Icon::Delete).to_string()),
                    )
                    .on_hover_text("Delete doll")
                    .clicked()
                {
                    if let Some(id) = self.actived_doll {
                        self.actions.push(Action::DollRemove(id));
                    }
                }
            });

            ui.group(|ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for (id, doll) in self.ppd.dolls() {
                                let is_actived_doll = self
                                    .actived_doll
                                    .map_or(false, |actived_doll| actived_doll == *id);

                                let resp = ui
                                    .add(
                                        Card::new(self.textures_doll.get(&id))
                                            .desc(&doll.desc)
                                            .highlighted(is_actived_doll),
                                    )
                                    .context_menu(|ui| {
                                        if ui.button("Edit doll").clicked() {
                                            self.actions.push(Action::DollEdit(*id));

                                            ui.close_menu();
                                        }

                                        if ui
                                            .add_enabled(
                                                self.ppd.dolls().len() > 1,
                                                Button::new("Delete doll"),
                                            )
                                            .clicked()
                                        {
                                            self.actions.push(Action::DollRemove(*id));

                                            ui.close_menu();
                                        }
                                    });

                                if resp.clicked() {
                                    self.actived_doll = Some(*id);
                                }

                                if resp.double_clicked() {
                                    self.actions.push(Action::DollEdit(*id));
                                }
                            }
                        });
                    });
            });
        });
    }

    fn ui_fragment_window(&mut self, ctx: &Context) {
        if !self.window_fragment_visible {
            return;
        }

        if self.adapter_fragment.is_none() {
            return;
        }

        let id = self.actived_fragment;

        let title = id.map_or("Create New Fragment".to_owned(), |id| {
            format!("Fragment - {}", id)
        });

        Modal::new("fragment_window").show(ctx, |ctx| {
            Window::new(title)
                .pivot(Align2::CENTER_CENTER)
                .default_pos(ctx.screen_rect().center())
                .resizable(false)
                .open(&mut self.window_fragment_visible)
                .show(ctx, |ui| {
                    ui.heading("Fragment");

                    Grid::new("fragment")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            let adapter_fragment = self.adapter_fragment.as_mut().unwrap();

                            ui.label("Description:");
                            ui.text_edit_singleline(&mut adapter_fragment.desc);

                            ui.end_row();

                            ui.horizontal_centered(|ui| {
                                ui.label("Pivot:");
                                ui.add(Tooltip::new(
                                    "The position where connects to the anchor point of a slot.",
                                ))
                            });
                            ui.add(PivotSelect::new(
                                &mut adapter_fragment.pivot.x,
                                &mut adapter_fragment.pivot.y,
                                adapter_fragment.image.width as f32,
                                adapter_fragment.image.height as f32,
                            ));

                            ui.end_row();

                            ui.horizontal_centered(|ui| {
                                ui.label("Image:");
                                ui.add(Tooltip::new("It's required."))
                            });

                            let texture = adapter_fragment.image.texture.as_ref().map(|texture| {
                                TextureData {
                                    width: adapter_fragment.image.width,
                                    height: adapter_fragment.image.height,
                                    texture: texture.clone(),
                                }
                            });

                            if ui
                                .add(ImageUpload::new(texture.as_ref()).removable(false).on_edit(
                                    || {
                                        self.actions.push(Action::FragmentAdapterBackgroundUpload);
                                    },
                                ))
                                .clicked()
                            {
                                self.actions.push(Action::FragmentAdapterBackgroundUpload);
                            }
                        });

                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            self.actions.push(Action::FragmentEditConfirm(id));

                            self.actions.push(Action::WindowFragmentVisible(false));
                        }

                        if ui.button("Cancel").clicked() {
                            self.actions.push(Action::WindowFragmentVisible(false));
                        }
                    });
                })
        });
    }

    fn ui_left_panel(&mut self, ui: &mut Ui) {
        ui.heading("Meta Info");

        Grid::new("meta").num_columns(2).show(ui, |ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.ppd.meta.name);
        });

        ui.separator();

        self.ui_dolls(ui);

        ui.separator();

        self.ui_doll(ui);
    }

    fn ui_right_panel(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Fragments");

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(icon_to_char(Icon::Add).to_string())
                    .on_hover_text("New fragment")
                    .clicked()
                {
                    self.actions.push(Action::FragmentCreate);
                }

                if ui
                    .add_enabled(
                        self.actived_fragment.is_some(),
                        Button::new(icon_to_char(Icon::Edit).to_string()),
                    )
                    .on_hover_text("Edit fragment")
                    .clicked()
                {
                    if let Some(id) = self.actived_fragment {
                        self.actions.push(Action::FragmentEdit(id));
                    }
                }

                if ui
                    .add_enabled(
                        self.actived_fragment.is_some(),
                        Button::new(icon_to_char(Icon::Delete).to_string()),
                    )
                    .on_hover_text("Delete fragment")
                    .clicked()
                {
                    if let Some(id) = self.actived_fragment {
                        self.actions.push(Action::FragmentRemove(id));
                    }
                }
            });

            ui.horizontal_wrapped(|ui| {
                ui.label("Filter:");

                ui.add(
                    TextEdit::singleline(&mut self.fragments_filter_keyword)
                        .hint_text("description")
                        .desired_width(120.0),
                );

                if ui.button(icon_to_char(Icon::Clear).to_string()).clicked() {
                    self.fragments_filter_keyword.clear();
                }
            });

            ui.group(|ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let actived_slot_candidates = self
                                .actived_slot
                                .map(|id| self.ppd.get_slot(id))
                                .flatten()
                                .map(|slot| &slot.candidates);

                            let rounding = 5.0;

                            for (id, fragment) in self.ppd.fragments() {
                                if !self.fragments_filter_keyword.is_empty()
                                    && !fragment.desc.contains(&self.fragments_filter_keyword)
                                {
                                    continue;
                                }

                                let is_actived_fragment = self
                                    .actived_fragment
                                    .map_or(false, |actived_fragment| actived_fragment == *id);

                                let resp = ui
                                    .add(
                                        Card::new(self.textures_fragment.get(&id))
                                            .desc(&fragment.desc)
                                            .rounding(rounding)
                                            .highlighted(is_actived_fragment),
                                    )
                                    .context_menu(|ui| {
                                        if ui.button("Edit fragment").clicked() {
                                            self.actions.push(Action::FragmentEdit(*id));

                                            ui.close_menu();
                                        }

                                        if ui.button("Delete fragment").clicked() {
                                            self.actions.push(Action::FragmentRemove(*id));

                                            ui.close_menu();
                                        }

                                        ui.separator();

                                        if ui.button("Add to slot").clicked() {}
                                    });

                                if resp.clicked() {
                                    self.actived_fragment = Some(*id);
                                }

                                if resp.double_clicked() {
                                    self.actions.push(Action::FragmentEdit(*id));
                                }

                                if let Some(candidates) = actived_slot_candidates {
                                    if !candidates.contains(id) {
                                        ui.painter().rect_filled(
                                            resp.rect,
                                            rounding,
                                            Color32::from_black_alpha(200),
                                        );
                                    }
                                }
                            }

                            if ui
                                .allocate_response(ui.available_size_before_wrap(), Sense::click())
                                .clicked()
                            {
                                self.actived_fragment = None;
                            }
                        });

                        if ui
                            .allocate_response(ui.available_size(), Sense::click())
                            .clicked()
                        {
                            self.actived_fragment = None;
                        }
                    });
            });
        });
    }

    fn ui_slot_window(&mut self, ctx: &Context) {
        if !self.window_slot_visible {
            return;
        }

        if self.adapter_slot.is_none() {
            return;
        }

        let adapter_slot = self.adapter_slot.as_mut().unwrap();

        let id = self.actived_slot;

        let title = id.map_or("Create New Slot".to_owned(), |id| format!("Slot - {}", id));

        Modal::new("slot_window").show(ctx, |ctx| {
            Window::new(title)
                .pivot(Align2::CENTER_CENTER)
                .default_pos(ctx.screen_rect().center())
                .resizable(false)
                .open(&mut self.window_slot_visible)
                .show(ctx, |ui| {
                    let is_create_mode = id.is_none();

                    ui.heading("Slot");

                    if is_create_mode {
                        ui_slot_window_grid(
                            &mut adapter_slot.desc,
                            &mut adapter_slot.required,
                            &mut adapter_slot.constrainted,
                            &mut adapter_slot.position,
                            &mut adapter_slot.width,
                            &mut adapter_slot.height,
                            &mut adapter_slot.anchor,
                            ui,
                        );
                    } else {
                        let slot = id.map(|id| self.ppd.get_slot_mut(id)).flatten();

                        if let Some(slot) = slot {
                            ui_slot_window_grid(
                                &mut slot.desc,
                                &mut slot.required,
                                &mut slot.constrainted,
                                &mut slot.position,
                                &mut slot.width,
                                &mut slot.height,
                                &mut slot.anchor,
                                ui,
                            );
                        }
                    }

                    ui.separator();

                    ui.label("Candidates");

                    ui.collapsing("Filter", |ui| {
                        ui.horizontal_wrapped(|ui| {
                            if ui
                                .radio_value(
                                    &mut adapter_slot.fragments_filter,
                                    FragmentFilter::All,
                                    "All",
                                )
                                .changed()
                            {
                                self.actions.push(Action::SlotAdapterFragmentFilter);
                            }

                            if ui
                                .radio_value(
                                    &mut adapter_slot.fragments_filter,
                                    FragmentFilter::IsCandidate,
                                    "Is candidate",
                                )
                                .changed()
                            {
                                self.actions.push(Action::SlotAdapterFragmentFilter);
                            }

                            if ui
                                .radio_value(
                                    &mut adapter_slot.fragments_filter,
                                    FragmentFilter::IsNotCandidate,
                                    "Is not candidate",
                                )
                                .changed()
                            {
                                self.actions.push(Action::SlotAdapterFragmentFilter);
                            }
                        });

                        ui.horizontal_wrapped(|ui| {
                            if ui
                                .add(
                                    TextEdit::singleline(
                                        &mut adapter_slot.fragments_filter_keyword,
                                    )
                                    .hint_text("description"),
                                )
                                .changed()
                            {
                                self.actions.push(Action::SlotAdapterFragmentFilter);
                            }

                            if ui.button(icon_to_char(Icon::Clear).to_string()).clicked() {
                                adapter_slot.fragments_filter_keyword.clear();

                                self.actions.push(Action::SlotAdapterFragmentFilter);
                            }
                        });
                    });

                    ui.group(|ui| {
                        ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .max_height(300.0)
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    if self.ppd.fragments().len() == 0
                                        || adapter_slot.actived_fragments.is_empty()
                                    {
                                        ui.horizontal_centered(|ui| {
                                            ui.vertical_centered(|ui| {
                                                if self.ppd.fragments().len() == 0 {
                                                    ui.label(
                                                        "The project doesn't have any fragments.",
                                                    );
                                                    ui.label("Please add a fragment first.");
                                                } else if adapter_slot.actived_fragments.is_empty()
                                                {
                                                    ui.label("No fragments found.");
                                                }
                                            });
                                        });
                                    }

                                    for fragment_id in &adapter_slot.actived_fragments {
                                        if let Some(fragment) = self.ppd.get_fragment(*fragment_id)
                                        {
                                            let is_candidate = if is_create_mode {
                                                adapter_slot.candidates.contains(fragment_id)
                                            } else {
                                                id.map_or(false, |id| {
                                                    self.ppd.get_slot(id).map_or(false, |slot| {
                                                        slot.candidates.contains(fragment_id)
                                                    })
                                                })
                                            };

                                            let resp = ui.add(
                                                Card::new(self.textures_fragment.get(fragment_id))
                                                    .desc(&fragment.desc)
                                                    .highlighted(is_candidate),
                                            );

                                            if resp.clicked() {
                                                if is_create_mode {
                                                    if is_candidate {
                                                        if let Some(index) = adapter_slot
                                                            .candidates
                                                            .iter()
                                                            .position(|v| v == fragment_id)
                                                        {
                                                            adapter_slot.candidates.remove(index);
                                                        }
                                                    } else {
                                                        adapter_slot.candidates.push(*fragment_id);
                                                    }
                                                } else {
                                                    let slot = id
                                                        .map(|id| self.ppd.get_slot_mut(id))
                                                        .flatten();

                                                    if let Some(slot) = slot {
                                                        if is_candidate {
                                                            if let Some(index) = adapter_slot
                                                                .candidates
                                                                .iter()
                                                                .position(|v| v == fragment_id)
                                                            {
                                                                adapter_slot
                                                                    .candidates
                                                                    .remove(index);
                                                            }
                                                        } else {
                                                            slot.candidates.push(*fragment_id);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            self.actions.push(Action::SlotEditConfirm(id));

                            self.actions.push(Action::WindowSlotVisible(false));
                        }

                        if ui.button("Cancel").clicked() {
                            if let Some(id) = id {
                                self.actions.push(Action::SlotEditCancel(id));
                            }

                            self.actions.push(Action::WindowSlotVisible(false));
                        }
                    });
                })
        });
    }

    fn ui_status_bar(&mut self, ui: &mut Ui) {}
}

fn ui_slot_window_grid(
    desc: &mut String,
    required: &mut bool,
    constrainted: &mut bool,
    position: &mut Point,
    width: &mut u32,
    height: &mut u32,
    anchor: &mut Point,
    ui: &mut Ui,
) {
    Grid::new("slot").num_columns(2).striped(true).show(ui, |ui| {
        ui.label("Description:");
        ui.text_edit_singleline(desc);

        ui.end_row();

        ui.horizontal_centered(|ui| {
            ui.label("Required:");
            ui.add(Tooltip::new("This slot always displays an image."))
        });
        ui.checkbox(required, "");

        ui.end_row();

        ui.horizontal_centered(|ui| {
            ui.label("Constrained:");
            ui.add(Tooltip::new("Resize image to fit the size of the slot, no matter what the original size of the image is."))
        });
        ui.checkbox(constrainted, "");

        ui.end_row();

        ui.horizontal_centered(|ui| {
            ui.label("Position:");
            ui.add(Tooltip::new("The top left position of the slot."))
        });
        ui.horizontal_wrapped(|ui| {
            ui.monospace("x");
            ui.add(DragValue::new(&mut position.x).speed(1));

            ui.monospace("y");
            ui.add(DragValue::new(&mut position.y).speed(1));
        });

        ui.end_row();

        ui.horizontal_centered(|ui| {
            ui.label("Size:");
            ui.add(Tooltip::new("The size of the slot. The displayed image will resize to this size if constrained is set."))
        });
        ui.horizontal_wrapped(|ui| {
            ui.monospace("w");
            ui.add(DragValue::new(width).speed(1));

            ui.monospace("h");
            ui.add(DragValue::new(height).speed(1));
        });

        ui.end_row();

        ui.horizontal_centered(|ui| {
            ui.label("Anchor:");
            ui.add(Tooltip::new("If constrained is not set, the position where the pivot of the image placed to."))
        });
        ui.add_enabled(!*constrainted,
            PivotSelect::new(&mut anchor.x, &mut anchor.y, *width as f32, *height as f32)
        );
    });
}
