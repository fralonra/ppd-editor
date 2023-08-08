use std::ops::RangeInclusive;

use eframe::{
    egui::{
        Button, CentralPanel, Context, DragValue, Frame, Grid, Layout, ScrollArea, Sense,
        SidePanel, TextEdit, TopBottomPanel, Ui, Window,
    },
    emath::{Align, Align2},
    epaint::{vec2, Color32, Vec2},
};
use material_icons::{icon_to_char, Icon};
use paperdoll_tar::paperdoll::common::Point;

use crate::common::TextureData;

use super::{
    actions::Action,
    canvas::CanvasState,
    widgets::{
        Card, Dialog, DialogResponse, FragmentEntry, ImageUpload, Modal, PivotSelect, SlotEntry,
        Tooltip,
    },
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

        self.ui_associated_slots_window(ctx);

        self.ui_about_window(ctx);

        self.ui_dialog(ctx);
    }

    fn ui_about_window(&mut self, ctx: &Context) {
        if !self.window_about_visible {
            return;
        }

        Modal::new("about_window").show(ctx, |ctx| {
            Window::new("About")
                .pivot(Align2::CENTER_CENTER)
                .default_pos(ctx.screen_rect().center())
                .collapsible(false)
                .resizable(false)
                .open(&mut self.window_about_visible)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Paperdoll Editor");

                        let hash = option_env!("GIT_COMMIT_HASH").unwrap_or_default();

                        ui.strong(format!(" v{} {}", env!("CARGO_PKG_VERSION"), hash));

                        if let Some(homepage) = option_env!("CARGO_PKG_HOMEPAGE") {
                            ui.hyperlink(homepage);
                        }
                    });

                    ui.allocate_space(vec2(ui.available_width(), 10.0));
                })
        });
    }

    fn ui_associated_slots_window(&mut self, ctx: &Context) {
        if !self.window_associated_slots_visible {
            return;
        }

        Modal::new("slots_window").show(ctx, |ctx| {
            Window::new("Manage Associated Slots")
                .pivot(Align2::CENTER_CENTER)
                .default_pos(ctx.screen_rect().center())
                .resizable(false)
                .open(&mut self.window_associated_slots_visible)
                .show(ctx, |ui| {
                    ui.label("Use this fragment in highlighted slots. Click to toggle.");

                    ui.horizontal(|ui| {
                        if ui.button("Select All").clicked() {
                            self.actions.push_back(Action::AssociatedSlotsSelectAll);
                        }

                        if ui.button("Unselect All").clicked() {
                            self.actions.push_back(Action::AssociatedSlotsUnselectAll);
                        }
                    });

                    ui.group(|ui| {
                        ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .max_height(300.0)
                            .show(ui, |ui| {
                                if self.ppd.slots().nth(0).is_none() {
                                    ui.allocate_space(vec2(1.0, 60.0));

                                    ui.vertical_centered(|ui| {
                                        ui.set_width(240.0);

                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(
                                                "No slots found.\n\n\
                                                You can add slots in the \
                                                left panel first.",
                                            );
                                        });
                                    });

                                    return;
                                }

                                ui.vertical(|ui| {
                                    ui.spacing_mut().item_spacing.y = 2.0;

                                    for (slot_id, slot) in self.ppd.slots() {
                                        let is_actived = self.associated_slots.contains(slot_id);

                                        if ui
                                            .add(SlotEntry::new(slot).actived(is_actived))
                                            .clicked()
                                        {
                                            if is_actived {
                                                self.associated_slots.remove(slot_id);
                                            } else {
                                                self.associated_slots.insert(*slot_id);
                                            }
                                        }
                                    }
                                });
                            });
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            self.actions
                                .push_back(Action::AssociatedSlotsConfirm(self.actived_fragment));

                            self.actions
                                .push_back(Action::WindowAssociatedSlotsVisible(false));
                        }

                        if ui.button("Cancel").clicked() {
                            self.actions
                                .push_back(Action::AssociatedSlotsCancel(self.actived_fragment));

                            self.actions
                                .push_back(Action::WindowAssociatedSlotsVisible(false));
                        }
                    });
                })
        });
    }

    fn ui_dialog(&mut self, ctx: &Context) {
        if !self.dialog_visible {
            return;
        }

        let inner_resp = Modal::new("doll_window").show(ctx, |ctx| {
            let mut dialog = Dialog::new("dialog", &self.dialog_option.text)
                .open(&mut self.dialog_visible)
                .primary_text(&self.dialog_option.primary_text);

            if let Some(text) = &self.dialog_option.secondary_text {
                dialog = dialog.secondary_text(text);
            }

            if let Some(text) = &self.dialog_option.tertiary_text {
                dialog = dialog.tertiary_text(text);
            }

            dialog.show(ctx)
        });

        if let Some(inner_resp) = inner_resp {
            if let Some(resp) = inner_resp.inner {
                match resp {
                    DialogResponse::Primary => {
                        if let Some(action) = self.dialog_option.primary_action.take() {
                            self.actions.push_back(action);
                        }
                    }
                    DialogResponse::Secondary => {
                        if let Some(action) = self.dialog_option.secondary_action.take() {
                            self.actions.push_back(action);
                        }
                    }
                    DialogResponse::Tertiary => {
                        if let Some(action) = self.dialog_option.tertiary_action.take() {
                            self.actions.push_back(action);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn ui_doll(&mut self, ui: &mut Ui) {
        let doll = self
            .actived_doll
            .map(|id| self.ppd.get_doll_mut(id))
            .flatten();

        if let Some(doll) = doll {
            let doll_id = doll.id();

            Grid::new("doll")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Description:");
                    ui.text_edit_singleline(&mut doll.desc);

                    ui.end_row();

                    ui.horizontal_centered(|ui| {
                        ui.label("Size:");
                        ui.add(Tooltip::new("The size of the doll."));
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.monospace("w");
                        if ui
                            .add(
                                DragValue::new(&mut doll.width)
                                    .clamp_range(RangeInclusive::new(1, u32::MAX))
                                    .speed(1),
                            )
                            .has_focus()
                        {
                            self.has_drag_value_focused = true;
                        }

                        ui.monospace("h");
                        if ui
                            .add(
                                DragValue::new(&mut doll.height)
                                    .clamp_range(RangeInclusive::new(1, u32::MAX))
                                    .speed(1),
                            )
                            .has_focus()
                        {
                            self.has_drag_value_focused = true;
                        }
                    });

                    ui.end_row();

                    ui.horizontal_centered(|ui| {
                        ui.label("Offset:");
                        ui.add(Tooltip::new(
                            "Offset pixels of the top left\
                            position of the background image, if any.",
                        ));
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.monospace("x");
                        if ui
                            .add(DragValue::new(&mut doll.offset.x).speed(1))
                            .has_focus()
                        {
                            self.has_drag_value_focused = true;
                        }

                        ui.monospace("y");
                        if ui
                            .add(DragValue::new(&mut doll.offset.y).speed(1))
                            .has_focus()
                        {
                            self.has_drag_value_focused = true;
                        }
                    });

                    ui.end_row();

                    ui.horizontal_centered(|ui| {
                        ui.label("Background:");
                        ui.add(Tooltip::new("The background of the doll. It's optional."));
                    });

                    let mut request_edit = false;
                    let mut request_remove = false;

                    if ui
                        .add(
                            ImageUpload::new(self.textures_doll.get(&doll_id))
                                .on_edit(|| {
                                    request_edit = true;
                                })
                                .on_remove(|| {
                                    request_remove = true;
                                }),
                        )
                        .clicked()
                    {
                        self.actions
                            .push_back(Action::DollBackgroundUpload(doll_id));
                    }

                    if request_edit {
                        self.actions
                            .push_back(Action::DollBackgroundUpload(doll_id));
                    }

                    if request_remove {
                        self.actions
                            .push_back(Action::DollBackgroundRemove(doll_id));
                    }
                });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Slots");
                ui.add(Tooltip::new(
                    "Areas where you can place fragments on top of them.",
                ));
            });

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(icon_to_char(Icon::Add).to_string())
                    .on_hover_text("New slot")
                    .clicked()
                {
                    self.actions.push_back(Action::SlotCreate);
                }

                ui.add_enabled_ui(self.actived_slot.is_some(), |ui| {
                    if ui
                        .button(icon_to_char(Icon::Edit).to_string())
                        .on_hover_text("Edit slot")
                        .clicked()
                    {
                        if let Some(slot_id) = self.actived_slot {
                            self.actions.push_back(Action::SlotEdit(slot_id));
                        }
                    }

                    if ui
                        .button(icon_to_char(Icon::Delete).to_string())
                        .on_hover_text("Delete slot")
                        .clicked()
                    {
                        if let Some(slot_id) = self.actived_slot {
                            self.actions.push_back(Action::SlotRemoveRequest(slot_id));
                        }
                    }

                    if ui
                        .add_enabled(
                            self.actived_slot.map_or(false, |slot_id| {
                                doll.slots
                                    .iter()
                                    .position(|id| *id == slot_id)
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
                                .push_back(if ui.input(|input| input.modifiers.shift) {
                                    Action::SlotRaiseTop(doll_id, slot_id)
                                } else {
                                    Action::SlotRaise(doll_id, slot_id)
                                });
                        }
                    }

                    if ui
                        .add_enabled(
                            self.actived_slot.map_or(false, |slot_id| {
                                doll.slots
                                    .iter()
                                    .position(|id| *id == slot_id)
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
                                .push_back(if ui.input(|input| input.modifiers.shift) {
                                    Action::SlotLowerBottom(doll_id, slot_id)
                                } else {
                                    Action::SlotLower(doll_id, slot_id)
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

                            for slot_id in slots {
                                if let Some(slot) = self.ppd.get_slot(slot_id) {
                                    let is_actived = self
                                        .actived_slot
                                        .map_or(false, |actived_slot| actived_slot == slot_id);

                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 0.0;

                                        let is_align_basis =
                                            self.align_basis_slots.contains(&slot_id);
                                        let is_locked = self.locked_slots.contains(&slot_id);
                                        let is_visible = self.visible_slots.contains(&slot_id);

                                        if ui
                                            .add(
                                                Button::new(
                                                    icon_to_char(if is_visible {
                                                        Icon::Visibility
                                                    } else {
                                                        Icon::VisibilityOff
                                                    })
                                                    .to_string(),
                                                )
                                                .frame(false),
                                            )
                                            .on_hover_text(
                                                "Change visibility of this slot in editor",
                                            )
                                            .clicked()
                                        {
                                            if is_visible {
                                                self.visible_slots.remove(&slot_id);
                                            } else {
                                                self.visible_slots.insert(slot_id);
                                            }
                                        }

                                        if ui
                                            .add(
                                                Button::new(
                                                    icon_to_char(if is_locked {
                                                        Icon::Lock
                                                    } else {
                                                        Icon::LockOpen
                                                    })
                                                    .to_string(),
                                                )
                                                .frame(false),
                                            )
                                            .on_hover_text(if is_locked {
                                                "Allow this slot to be dragged around"
                                            } else {
                                                "Lock this slot to prevent \
                                                    it from being dragged"
                                            })
                                            .clicked()
                                        {
                                            if is_locked {
                                                self.locked_slots.remove(&slot_id);
                                            } else {
                                                self.locked_slots.insert(slot_id);
                                            }
                                        }

                                        if ui
                                            .add(
                                                Button::new(
                                                    icon_to_char(if is_align_basis {
                                                        Icon::GridOn
                                                    } else {
                                                        Icon::GridOff
                                                    })
                                                    .to_string(),
                                                )
                                                .frame(false),
                                            )
                                            .on_hover_text(if is_align_basis {
                                                "Do not use this slot as \
                                                    a basis for aligning other slots"
                                            } else {
                                                "Use this slot as a basis for aligning other slots"
                                            })
                                            .clicked()
                                        {
                                            if is_align_basis {
                                                self.align_basis_slots.remove(&slot_id);
                                            } else {
                                                self.align_basis_slots.insert(slot_id);
                                            }
                                        }

                                        let resp = ui.add(SlotEntry::new(slot).actived(is_actived));

                                        if resp.clicked() {
                                            self.actived_slot = Some(slot_id);
                                        }

                                        if resp.double_clicked() {
                                            self.actions.push_back(Action::SlotEdit(slot_id));
                                        }

                                        resp
                                    })
                                    .inner
                                    .context_menu(|ui| {
                                        self.menu_slot(ui, Some(slot_id));
                                    });
                                }
                            }

                            if ui
                                .allocate_response(ui.available_size(), Sense::click())
                                .context_menu(|ui| self.menu_slot(ui, self.actived_slot))
                                .clicked()
                            {
                                self.actived_slot = None;
                            }
                        });
                    });
            });
        }
    }

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

                    Grid::new("doll")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            let adapter_doll = self.adapter_doll.as_mut().unwrap();

                            ui.label("Description:");
                            ui.text_edit_singleline(&mut adapter_doll.desc);

                            ui.end_row();

                            ui.horizontal_centered(|ui| {
                                ui.label("Size:");
                                ui.add(Tooltip::new("The size of the doll."));
                            });
                            ui.horizontal_wrapped(|ui| {
                                ui.monospace("w");
                                if ui
                                    .add(
                                        DragValue::new(&mut adapter_doll.width)
                                            .clamp_range(RangeInclusive::new(1, u32::MAX))
                                            .speed(1),
                                    )
                                    .has_focus()
                                {
                                    self.has_drag_value_focused = true;
                                }

                                ui.monospace("h");
                                if ui
                                    .add(
                                        DragValue::new(&mut adapter_doll.height)
                                            .clamp_range(RangeInclusive::new(1, u32::MAX))
                                            .speed(1),
                                    )
                                    .has_focus()
                                {
                                    self.has_drag_value_focused = true;
                                }
                            });

                            ui.end_row();

                            ui.horizontal_centered(|ui| {
                                ui.label("Offset:");
                                ui.add(Tooltip::new(
                                    "Offset pixels of the top left position\
                                    of the background image, if any.",
                                ));
                            });
                            ui.horizontal_wrapped(|ui| {
                                ui.monospace("x");
                                if ui
                                    .add(DragValue::new(&mut adapter_doll.offset.x).speed(1))
                                    .has_focus()
                                {
                                    self.has_drag_value_focused = true;
                                }

                                ui.monospace("y");
                                if ui
                                    .add(DragValue::new(&mut adapter_doll.offset.y).speed(1))
                                    .has_focus()
                                {
                                    self.has_drag_value_focused = true;
                                }
                            });

                            ui.end_row();

                            ui.horizontal_centered(|ui| {
                                ui.label("Background:");
                                ui.add(Tooltip::new("The background of the doll. It's optional."));
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
                                self.actions.push_back(Action::DollAdapterBackgroundUpload);
                            }

                            if request_edit {
                                self.actions.push_back(Action::DollAdapterBackgroundUpload);
                            }

                            if request_remove {
                                self.actions.push_back(Action::DollAdapterBackgroundRemove);
                            }
                        });

                    ui.add_visible_ui(self.window_doll_error.is_some(), |ui| {
                        ui.colored_label(
                            Color32::LIGHT_RED,
                            self.window_doll_error
                                .as_ref()
                                .map(|err| err.as_str())
                                .unwrap_or_default(),
                        );
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            self.actions.push_back(Action::DollEditConfirm(id));

                            self.actions.push_back(Action::WindowDollVisible(false));
                        }

                        if ui.button("Cancel").clicked() {
                            self.actions.push_back(Action::WindowDollVisible(false));
                        }
                    });
                })
        });
    }

    fn ui_dolls(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_height(100.0);

            ui.heading("Dolls");

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(icon_to_char(Icon::Add).to_string())
                    .on_hover_text("New doll")
                    .clicked()
                {
                    self.actions.push_back(Action::DollCreate);
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
                        self.actions.push_back(Action::DollRemoveRequest(id));
                    }
                }
            });

            ui.group(|ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let dolls: Vec<u32> = self.ppd.dolls().map(|(id, _)| *id).collect();

                        for id in dolls {
                            if let Some(doll) = self.ppd.get_doll(id) {
                                let is_actived_doll = self
                                    .actived_doll
                                    .map_or(false, |actived_doll| actived_doll == id);

                                ui.add(|ui: &mut Ui| {
                                    let desc = if doll.desc.is_empty() {
                                        format!("Doll - {}", id)
                                    } else {
                                        doll.desc.clone()
                                    };

                                    let resp = ui.add(
                                        Card::new(self.textures_doll.get(&id))
                                            .desc(&desc)
                                            .highlighted(is_actived_doll),
                                    );

                                    if resp.clicked() {
                                        self.actived_doll = Some(id);
                                    }

                                    if resp.double_clicked() {
                                        self.actions.push_back(Action::DollEdit(id));
                                    }

                                    resp
                                })
                                .context_menu(|ui| {
                                    self.menu_doll(ui, Some(id));
                                });
                            }
                        }
                    });

                    ui.allocate_response(ui.available_size(), Sense::click())
                        .context_menu(|ui| {
                            self.menu_doll(ui, self.actived_doll);
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

        let adapter_fragment = self.adapter_fragment.as_mut().unwrap();

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
                    let is_create_mode = id.is_none();

                    ui.heading("Fragment");

                    Grid::new("fragment")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            if is_create_mode {
                                ui_fragment_window_grid(
                                    &mut adapter_fragment.desc,
                                    adapter_fragment.image.width,
                                    adapter_fragment.image.height,
                                    &mut adapter_fragment.pivot,
                                    ui,
                                );
                            } else {
                                let fragment = id.map(|id| self.ppd.get_fragment_mut(id)).flatten();

                                if let Some(fragment) = fragment {
                                    ui_fragment_window_grid(
                                        &mut fragment.desc,
                                        fragment.image.width,
                                        fragment.image.height,
                                        &mut fragment.pivot,
                                        ui,
                                    );
                                }
                            }

                            ui.horizontal_centered(|ui| {
                                ui.label("Image:");
                                ui.add(Tooltip::new("It's required."));
                            });

                            let texture = if is_create_mode {
                                adapter_fragment
                                    .image
                                    .texture
                                    .as_ref()
                                    .map(|texture| TextureData {
                                        width: adapter_fragment.image.width,
                                        height: adapter_fragment.image.height,
                                        texture: texture.clone(),
                                    })
                            } else {
                                let fragment = id.map(|id| self.ppd.get_fragment_mut(id)).flatten();

                                fragment
                                    .map(|fragment| {
                                        self.textures_fragment.get(&fragment.id()).map(|texture| {
                                            TextureData {
                                                width: fragment.image.width,
                                                height: fragment.image.height,
                                                texture: texture.texture.clone(),
                                            }
                                        })
                                    })
                                    .flatten()
                            };

                            if ui
                                .add(ImageUpload::new(texture.as_ref()).removable(false).on_edit(
                                    || {
                                        if is_create_mode {
                                            self.actions
                                                .push_back(Action::FragmentAdapterBackgroundUpload);
                                        } else {
                                            if let Some(id) = id {
                                                self.actions.push_back(
                                                    Action::FragmentBackgroundUpload(id),
                                                );
                                            }
                                        }
                                    },
                                ))
                                .clicked()
                            {
                                if is_create_mode {
                                    self.actions
                                        .push_back(Action::FragmentAdapterBackgroundUpload);
                                } else {
                                    if let Some(id) = id {
                                        self.actions
                                            .push_back(Action::FragmentBackgroundUpload(id));
                                    }
                                }
                            }
                        });

                    ui.add_visible_ui(self.window_fragment_error.is_some(), |ui| {
                        ui.colored_label(
                            Color32::LIGHT_RED,
                            self.window_fragment_error
                                .as_ref()
                                .map(|err| err.as_str())
                                .unwrap_or_default(),
                        );
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            self.actions.push_back(Action::FragmentEditConfirm(id));

                            self.actions.push_back(Action::WindowFragmentVisible(false));
                        }

                        if ui.button("Cancel").clicked() {
                            self.actions.push_back(Action::FragmentEditCancel(id));

                            self.actions.push_back(Action::WindowFragmentVisible(false));
                        }
                    });
                })
        });

        fn ui_fragment_window_grid(
            desc: &mut String,
            width: u32,
            height: u32,
            pivot: &mut Point,
            ui: &mut Ui,
        ) {
            ui.label("Description:");
            ui.text_edit_singleline(desc);

            ui.end_row();

            ui.horizontal_centered(|ui| {
                ui.label("Pivot:");
                ui.add(Tooltip::new(
                    "The position where connects to the anchor point of a slot.",
                ));
            });
            ui.add(PivotSelect::new(
                &mut pivot.x,
                &mut pivot.y,
                width as f32,
                height as f32,
            ));

            ui.end_row();
        }
    }

    fn ui_left_panel(&mut self, ui: &mut Ui) {
        ui.heading("Project Info");

        Grid::new("meta").num_columns(2).show(ui, |ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.ppd.meta.name);
        });

        ui.separator();

        self.ui_dolls(ui);

        self.ui_doll(ui);
    }

    fn ui_right_panel(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Fragments");
                ui.add(Tooltip::new("Image assets which can be placed into slots."));
            });

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(icon_to_char(Icon::Add).to_string())
                    .on_hover_text("New fragment")
                    .clicked()
                {
                    self.actions.push_back(Action::FragmentCreate);
                }

                if ui
                    .button(icon_to_char(Icon::PlaylistAdd).to_string())
                    .on_hover_text("Add fragments from a batch of images")
                    .clicked()
                {
                    self.actions
                        .push_back(Action::FragmentCreateFromBatchImages);
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
                        self.actions.push_back(Action::FragmentEdit(id));
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
                        self.actions.push_back(Action::FragmentRemoveRequest(id));
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

                if ui
                    .add_enabled(
                        !self.fragments_filter_keyword.is_empty(),
                        Button::new(icon_to_char(Icon::Clear).to_string()),
                    )
                    .clicked()
                {
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
                                .map(|slot| slot.candidates.clone())
                                .unwrap_or_default();

                            let fragments: Vec<u32> =
                                self.ppd.fragments().map(|(id, _)| *id).collect();

                            let rounding = 5.0;

                            for id in fragments {
                                if let Some(fragment) = self.ppd.get_fragment(id) {
                                    if !self.fragments_filter_keyword.is_empty()
                                        && !fragment.desc.contains(&self.fragments_filter_keyword)
                                    {
                                        continue;
                                    }

                                    ui.add(|ui: &mut Ui| {
                                        let is_actived_fragment = self
                                            .actived_fragment
                                            .map_or(false, |actived_fragment| {
                                                actived_fragment == id
                                            });

                                        let resp = ui.add(
                                            Card::new(self.textures_fragment.get(&id))
                                                .desc(&fragment.desc)
                                                .rounding(rounding)
                                                .highlighted(is_actived_fragment),
                                        );

                                        if !actived_slot_candidates.contains(&id) {
                                            ui.painter().rect_filled(
                                                resp.rect,
                                                rounding,
                                                Color32::from_black_alpha(200),
                                            );
                                        }

                                        if resp.clicked() {
                                            self.actived_fragment = Some(id);
                                        }

                                        if resp.double_clicked() {
                                            self.actions.push_back(Action::FragmentEdit(id));
                                        }

                                        resp
                                    })
                                    .context_menu(|ui| {
                                        self.menu_fragment(ui, Some(id));
                                    });
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
                            .context_menu(|ui| {
                                self.menu_fragment(ui, self.actived_fragment);
                            })
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
                    ui.heading("Slot");

                    ui.horizontal(|ui| {
                        let left_resp = ui
                            .vertical(|ui| {
                                ui.set_max_width(250.0);

                                let slot_data = id
                                    .map(|id| self.ppd.get_slot_mut(id))
                                    .flatten()
                                    .map(|slot| {
                                        (
                                            &mut slot.desc,
                                            &mut slot.required,
                                            &mut slot.constrainted,
                                            &mut slot.positions,
                                            &mut slot.width,
                                            &mut slot.height,
                                            &mut slot.anchor,
                                        )
                                    })
                                    .unwrap_or((
                                        &mut adapter_slot.desc,
                                        &mut adapter_slot.required,
                                        &mut adapter_slot.constrainted,
                                        &mut adapter_slot.positions,
                                        &mut adapter_slot.width,
                                        &mut adapter_slot.height,
                                        &mut adapter_slot.anchor,
                                    ));

                                Grid::new("slot")
                                    .num_columns(2)
                                    .striped(true)
                                    .show(ui, |ui| {
                                        ui.label("Description:");
                                        ui.text_edit_singleline(slot_data.0);

                                        ui.end_row();

                                        ui.horizontal_centered(|ui| {
                                            ui.label("Required:");
                                            ui.add(Tooltip::new(
                                                "This slot always displays an image.",
                                            ));
                                        });
                                        ui.checkbox(slot_data.1, "");

                                        ui.end_row();

                                        ui.horizontal_centered(|ui| {
                                            ui.label("Constrained:");

                                            ui.add(Tooltip::new(
                                                "Resize image to fit the size of the slot,\
                                                no matter what the original size of the image is.",
                                            ));
                                        });
                                        ui.checkbox(slot_data.2, "");

                                        ui.end_row();

                                        ui.horizontal_centered(|ui| {
                                            ui.label("Positions:");
                                            ui.add(Tooltip::new(
                                                "The top left position of the slot.\
                                                One slot can have multiple positions.",
                                            ));
                                        });

                                        ui.vertical(|ui| {
                                            ui.horizontal_wrapped(|ui| {
                                                if ui
                                                    .button(icon_to_char(Icon::Add).to_string())
                                                    .on_hover_text("Add position")
                                                    .clicked()
                                                {
                                                    self.actions
                                                        .push_back(Action::SlotAddPosition(id));
                                                }

                                                if ui
                                                    .add_enabled(
                                                        adapter_slot.actived_position.is_some(),
                                                        Button::new(
                                                            icon_to_char(Icon::Delete).to_string(),
                                                        ),
                                                    )
                                                    .on_hover_text("Delete position")
                                                    .clicked()
                                                {
                                                    self.actions.push_back(
                                                        Action::SlotRemovePosition(
                                                            id,
                                                            adapter_slot.actived_position.unwrap(),
                                                        ),
                                                    );
                                                }
                                            });

                                            if ui_positions(
                                                slot_data.3,
                                                &mut adapter_slot.actived_position,
                                                ui,
                                            ) {
                                                self.has_drag_value_focused = true;
                                            }
                                        });

                                        ui.end_row();

                                        ui.horizontal_centered(|ui| {
                                            ui.label("Size:");
                                            ui.add(Tooltip::new(
                                                "The size of the slot.\
                                                The displayed image will resize\
                                                to this size if constrained is set.",
                                            ));
                                        });
                                        ui.horizontal_wrapped(|ui| {
                                            ui.monospace("w");

                                            let resp = ui.add(
                                                DragValue::new(slot_data.4)
                                                    .clamp_range(RangeInclusive::new(1, u32::MAX))
                                                    .speed(1),
                                            );

                                            if resp.has_focus() {
                                                self.has_drag_value_focused = true;
                                            }

                                            if resp.changed() {
                                                if adapter_slot.keep_aspect_ratio {
                                                    *slot_data.5 = (*slot_data.4 as f32
                                                        / adapter_slot.aspect_ratio)
                                                        as u32;
                                                }

                                                adapter_slot.aspect_ratio =
                                                    *slot_data.4 as f32 / *slot_data.5 as f32;
                                            }

                                            ui.monospace("h");

                                            let resp = ui.add(
                                                DragValue::new(slot_data.5)
                                                    .clamp_range(RangeInclusive::new(1, u32::MAX))
                                                    .speed(1),
                                            );

                                            if resp.has_focus() {
                                                self.has_drag_value_focused = true;
                                            }

                                            if resp.changed() {
                                                if adapter_slot.keep_aspect_ratio {
                                                    *slot_data.4 = (*slot_data.5 as f32
                                                        * adapter_slot.aspect_ratio)
                                                        as u32;
                                                }

                                                adapter_slot.aspect_ratio =
                                                    *slot_data.4 as f32 / *slot_data.5 as f32;
                                            }

                                            ui.spacing_mut().item_spacing.x = 0.0;

                                            if ui
                                                .add(
                                                    Button::new(
                                                        icon_to_char(
                                                            if adapter_slot.keep_aspect_ratio {
                                                                Icon::Link
                                                            } else {
                                                                Icon::LinkOff
                                                            },
                                                        )
                                                        .to_string(),
                                                    )
                                                    .frame(false),
                                                )
                                                .clicked()
                                            {
                                                adapter_slot.keep_aspect_ratio =
                                                    !adapter_slot.keep_aspect_ratio;
                                            }
                                        });

                                        ui.end_row();

                                        ui.horizontal_centered(|ui| {
                                            ui.label("Anchor:");
                                            ui.add(Tooltip::new(
                                                "If constrained is not set,\
                                                the position where the pivot\
                                                of the image placed to.",
                                            ));
                                        });
                                        ui.add_enabled(
                                            !*slot_data.2,
                                            PivotSelect::new(
                                                &mut slot_data.6.x,
                                                &mut slot_data.6.y,
                                                *slot_data.4 as f32,
                                                *slot_data.5 as f32,
                                            ),
                                        );
                                    });
                            })
                            .response;

                        ui.separator();

                        ui.vertical(|ui| {
                            ui.set_height(left_resp.rect.height());

                            ui.horizontal(|ui| {
                                ui.label("Candidates");
                                ui.add(Tooltip::new("Fragments those can be used in this slot."));
                            });

                            ui.horizontal_centered(|ui| {
                                let frame_width = 166.0;
                                let top_padding = 60.0;

                                let candidates = id
                                    .map(|id| self.ppd.get_slot(id))
                                    .flatten()
                                    .map(|slot| &slot.candidates)
                                    .unwrap_or_else(|| &adapter_slot.candidates);

                                let can_raise =
                                    adapter_slot.actived_candidate.map_or(false, |fragment_id| {
                                        candidates
                                            .iter()
                                            .position(|id| *id == fragment_id)
                                            .map_or(false, |i| i > 0)
                                    });

                                let can_lower =
                                    adapter_slot.actived_candidate.map_or(false, |fragment_id| {
                                        candidates
                                            .iter()
                                            .position(|id| *id == fragment_id)
                                            .map_or(false, |i| i < candidates.len() - 1)
                                    });

                                ui.vertical(|ui| {
                                    let actived_candidate = adapter_slot.actived_candidate;

                                    ui.horizontal(|ui| {
                                        ui.add_enabled_ui(actived_candidate.is_some(), |ui| {
                                            if ui
                                                .add_enabled(
                                                    can_raise,
                                                    Button::new(
                                                        icon_to_char(Icon::ExpandLess).to_string(),
                                                    ),
                                                )
                                                .on_hover_ui(|ui| {
                                                    ui.vertical(|ui| {
                                                        ui.label("Raise candidate");

                                                        ui.horizontal(|ui| {
                                                            ui.strong("Shift");

                                                            ui.label("Raise to the top");
                                                        });
                                                    });
                                                })
                                                .clicked()
                                            {
                                                if let Some(fragment_id) = actived_candidate {
                                                    self.actions.push_back(
                                                        if ui.input(|input| input.modifiers.shift) {
                                                            Action::CandidateRaiseTop(
                                                                id,
                                                                fragment_id,
                                                            )
                                                        } else {
                                                            Action::CandidateRaise(id, fragment_id)
                                                        },
                                                    );
                                                }
                                            }

                                            if ui
                                                .add_enabled(
                                                    can_lower,
                                                    Button::new(
                                                        icon_to_char(Icon::ExpandMore).to_string(),
                                                    ),
                                                )
                                                .on_hover_ui(|ui| {
                                                    ui.vertical(|ui| {
                                                        ui.label("Lower candidate");

                                                        ui.horizontal(|ui| {
                                                            ui.strong("Shift");

                                                            ui.label("Lower to the bottom");
                                                        });
                                                    });
                                                })
                                                .clicked()
                                            {
                                                if let Some(fragment_id) = actived_candidate {
                                                    self.actions.push_back(
                                                        if ui.input(|input| input.modifiers.shift) {
                                                            Action::CandidateLowerBottom(
                                                                id,
                                                                fragment_id,
                                                            )
                                                        } else {
                                                            Action::CandidateLower(id, fragment_id)
                                                        },
                                                    );
                                                }
                                            }
                                        });
                                    });

                                    ui.group(|ui| {
                                        ui.set_height(ui.available_height());

                                        ScrollArea::both()
                                            .auto_shrink([false, false])
                                            .max_width(frame_width)
                                            .show(ui, |ui| {
                                                ui.vertical(|ui| {
                                                    ui.spacing_mut().item_spacing.y = 0.0;

                                                    for candidate_id in candidates {
                                                        if let Some(candidate) =
                                                            self.ppd.get_fragment(*candidate_id)
                                                        {
                                                            let is_actived = adapter_slot
                                                                .actived_candidate
                                                                .map_or(
                                                                    false,
                                                                    |actived_candidate| {
                                                                        actived_candidate
                                                                            == *candidate_id
                                                                    },
                                                                );

                                                            let texture = self
                                                                .textures_fragment
                                                                .get(&candidate_id);

                                                            let resp = ui.add(
                                                                FragmentEntry::new(candidate)
                                                                    .actived(is_actived)
                                                                    .texture(texture),
                                                            );

                                                            if resp.clicked() {
                                                                adapter_slot.actived_candidate =
                                                                    Some(*candidate_id);
                                                            }

                                                            if resp.double_clicked() {
                                                                self.actions.push_back(
                                                                    Action::SlotRemoveCandidate(
                                                                        id,
                                                                        *candidate_id,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }

                                                    if ui
                                                        .allocate_response(
                                                            ui.available_size(),
                                                            Sense::click(),
                                                        )
                                                        .clicked()
                                                    {
                                                        adapter_slot.actived_candidate = None
                                                    }
                                                });
                                            });
                                    });
                                });

                                ui.vertical(|ui| {
                                    ui.allocate_space(vec2(1.0, top_padding));

                                    if ui
                                        .add_enabled(
                                            adapter_slot.filtered_fragments.len() > 0,
                                            Button::new(icon_to_char(Icon::FirstPage).to_string()),
                                        )
                                        .on_hover_text(
                                            "Add all fragments in the right to candidates",
                                        )
                                        .clicked()
                                    {
                                        self.actions.push_back(Action::SlotAddCandidates(
                                            id,
                                            adapter_slot.filtered_fragments.clone(),
                                        ));
                                    }

                                    if ui
                                        .add_enabled(
                                            adapter_slot.actived_fragments.len() > 0,
                                            Button::new(
                                                icon_to_char(Icon::ChevronLeft).to_string(),
                                            ),
                                        )
                                        .on_hover_text("Add selected fragments to candidates")
                                        .clicked()
                                    {
                                        self.actions.push_back(Action::SlotAddCandidates(
                                            id,
                                            adapter_slot
                                                .actived_fragments
                                                .iter()
                                                .map(|id| *id)
                                                .collect::<Vec<u32>>(),
                                        ));
                                    }

                                    if ui
                                        .add_enabled(
                                            adapter_slot.actived_candidate.is_some(),
                                            Button::new(
                                                icon_to_char(Icon::ChevronRight).to_string(),
                                            ),
                                        )
                                        .on_hover_text("Remove selected fragment from candidates")
                                        .clicked()
                                    {
                                        self.actions.push_back(Action::SlotRemoveCandidate(
                                            id,
                                            adapter_slot.actived_candidate.unwrap(),
                                        ))
                                    }

                                    if ui
                                        .add_enabled(
                                            candidates.len() > 0,
                                            Button::new(icon_to_char(Icon::LastPage).to_string()),
                                        )
                                        .on_hover_text("Remove all fragments from candidates")
                                        .clicked()
                                    {
                                        self.actions.push_back(Action::SlotRemoveCandidates(
                                            id,
                                            candidates.to_vec(),
                                        ))
                                    }
                                });

                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.set_width(frame_width - 8.0);

                                        if ui
                                            .add(
                                                TextEdit::singleline(
                                                    &mut adapter_slot.fragments_filter_keyword,
                                                )
                                                .hint_text("Filter by description"),
                                            )
                                            .changed()
                                        {
                                            self.actions
                                                .push_back(Action::SlotAdapterFragmentFilter);
                                        }

                                        if ui
                                            .add_enabled(
                                                !adapter_slot.fragments_filter_keyword.is_empty(),
                                                Button::new(icon_to_char(Icon::Clear).to_string()),
                                            )
                                            .clicked()
                                        {
                                            adapter_slot.fragments_filter_keyword.clear();

                                            self.actions
                                                .push_back(Action::SlotAdapterFragmentFilter);
                                        }
                                    });

                                    ui.group(|ui| {
                                        ScrollArea::both()
                                            .auto_shrink([false, false])
                                            .max_width(frame_width)
                                            .show(ui, |ui| {
                                                if self.ppd.fragments().nth(0).is_none() {
                                                    ui.allocate_space(vec2(1.0, top_padding));

                                                    ui.vertical_centered(|ui| {
                                                        ui.set_width(frame_width * 0.8);

                                                        ui.horizontal_wrapped(|ui| {
                                                            ui.label(
                                                                "No fragments found.\n\n\
                                                                    You can add fragments in the \
                                                                    right panel first.",
                                                            );
                                                        });
                                                    });

                                                    return;
                                                }

                                                if adapter_slot.filtered_fragments.is_empty() {
                                                    ui.allocate_space(vec2(1.0, top_padding));

                                                    ui.vertical_centered(|ui| {
                                                        ui.set_width(frame_width * 0.8);

                                                        ui.horizontal_wrapped(|ui| {
                                                            ui.label("No fragments found.");
                                                        });
                                                    });

                                                    return;
                                                }

                                                ui.horizontal_wrapped(|ui| {
                                                    for fragment_id in
                                                        &adapter_slot.filtered_fragments
                                                    {
                                                        if candidates.contains(fragment_id) {
                                                            continue;
                                                        }

                                                        let is_selected = adapter_slot
                                                            .actived_fragments
                                                            .contains(fragment_id);

                                                        if let Some(fragment) =
                                                            self.ppd.get_fragment(*fragment_id)
                                                        {
                                                            let resp = ui.add(
                                                                Card::new(
                                                                    self.textures_fragment
                                                                        .get(fragment_id),
                                                                )
                                                                .desc(&fragment.desc)
                                                                .highlighted(is_selected),
                                                            );

                                                            if resp.clicked() {
                                                                if is_selected {
                                                                    adapter_slot
                                                                        .actived_fragments
                                                                        .remove(fragment_id);
                                                                } else {
                                                                    adapter_slot
                                                                        .actived_fragments
                                                                        .insert(*fragment_id);
                                                                }
                                                            }

                                                            if resp.double_clicked() {
                                                                self.actions.push_back(
                                                                    Action::SlotAddCandidate(
                                                                        id,
                                                                        *fragment_id,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    }
                                                });
                                            });
                                    });
                                });
                            });
                        });
                    });

                    ui.add_visible_ui(self.window_slot_error.is_some(), |ui| {
                        ui.colored_label(
                            Color32::LIGHT_RED,
                            self.window_slot_error
                                .as_ref()
                                .map(|err| err.as_str())
                                .unwrap_or_default(),
                        );
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Confirm").clicked() {
                            self.actions.push_back(Action::SlotEditConfirm(id));

                            self.actions.push_back(Action::WindowSlotVisible(false));
                        }

                        if ui.button("Cancel").clicked() {
                            self.actions.push_back(Action::SlotEditCancel(id));

                            self.actions.push_back(Action::WindowSlotVisible(false));
                        }
                    });
                })
        });

        fn ui_positions(
            positions: &mut Vec<Point>,
            actived_position: &mut Option<usize>,
            ui: &mut Ui,
        ) -> bool {
            Frame::group(ui.style())
                .inner_margin(Vec2::splat(0.0))
                .show(ui, |ui| {
                    ScrollArea::both()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let mut has_drag_value_focused = false;

                            ui.spacing_mut().item_spacing.y = 1.0;

                            ui.vertical(|ui| {
                                for (position_index, position) in positions.iter_mut().enumerate() {
                                    let is_actived = actived_position
                                        .map(|actived_index| actived_index == position_index)
                                        .unwrap_or(false);

                                    let (rect, resp) = ui.allocate_at_least(
                                        vec2(ui.available_width(), 24.0),
                                        Sense::click(),
                                    );

                                    let visuals = ui.style().interact_selectable(&resp, is_actived);

                                    if is_actived {
                                        ui.painter().rect(
                                            rect,
                                            0.0,
                                            visuals.weak_bg_fill,
                                            visuals.bg_stroke,
                                        );
                                    }

                                    if resp.clicked() {
                                        if is_actived {
                                            *actived_position = None;
                                        } else {
                                            *actived_position = Some(position_index);
                                        }
                                    }

                                    ui.allocate_ui_at_rect(rect.shrink2(vec2(4.0, 0.0)), |ui| {
                                        ui.horizontal_centered(|ui| {
                                            ui.monospace("x");
                                            if ui
                                                .add(DragValue::new(&mut position.x).speed(1))
                                                .has_focus()
                                            {
                                                has_drag_value_focused = true;
                                            }

                                            ui.monospace("y");
                                            if ui
                                                .add(DragValue::new(&mut position.y).speed(1))
                                                .has_focus()
                                            {
                                                has_drag_value_focused = true;
                                            }
                                        });
                                    });
                                }
                            });

                            has_drag_value_focused
                        })
                })
                .inner
                .inner
        }
    }

    fn ui_status_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_visible_ui(self.cursor_position.is_some(), |ui| {
                ui.set_width(100.0);

                if let Some(position) = self.cursor_position {
                    ui.label(format!("{:.1},{:.1}", position.x, position.y));
                }
            });

            ui.add_visible_ui(true, |ui| {
                ui.set_width(80.0);

                ui.label(format!("{}%", self.viewport.scale * 100.0));
            });

            match self.canvas_state {
                CanvasState::Idle | CanvasState::Dragging => ui.horizontal_wrapped(|ui| {
                    ui.strong("Ctrl + Scroll");
                    ui.label("or");
                    ui.strong("+/-");
                    ui.label("to zoom in / out");

                    ui.strong("Left Click");
                    ui.label("to select a slot");

                    ui.strong("Right Drag");
                    ui.label("to drag around");

                    ui.strong("Arrow Keys");
                    ui.label("to move around");

                    ui.strong("Scroll");
                    ui.label("to move vertically");

                    ui.strong("Shift + Scroll");
                    ui.label("to move horizontally");
                }),
                CanvasState::ActivedSlotHover => ui.horizontal(|ui| {
                    ui.strong("Left Drag");
                    ui.label("to move the slot");

                    ui.strong("Right Click");
                    ui.label("to open context menu");
                }),
                CanvasState::DraggingAnchor => ui.horizontal(|ui| {
                    ui.strong("Ctrl");
                    ui.label("to restrict to horizontal / vertical");

                    ui.strong("Alt");
                    ui.label("to disable snapping");
                }),
                CanvasState::DraggingSlot => ui.horizontal(|ui| {
                    ui.strong("Ctrl");
                    ui.label("to restrict to horizontal / vertical");

                    ui.strong("Shift");
                    ui.label("to move all positions of the same slot at the same time");

                    ui.strong("Alt");
                    ui.label("to disable snapping");
                }),
                CanvasState::ResizingSlot => ui.horizontal(|ui| {
                    ui.strong("Ctrl");
                    ui.label("to lock ratio");

                    ui.strong("Alt");
                    ui.label("to disable snapping");
                }),
            }
        });
    }
}
