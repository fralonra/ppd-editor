use eframe::{
    egui::{Context, InnerResponse, Layout, RichText, WidgetText, Window},
    emath::{Align, Align2},
    epaint::vec2,
};

pub enum DialogResponse {
    Idle,
    Primary,
    Secondary,
    Tertiary,
}

impl DialogResponse {
    fn is_some(&self) -> bool {
        match self {
            Self::Idle => false,
            _ => true,
        }
    }
}

pub struct Dialog<'a> {
    id: WidgetText,

    text: RichText,
    primary_text: WidgetText,
    secondary_text: Option<WidgetText>,
    tertiary_text: Option<WidgetText>,
    open: Option<&'a mut bool>,
}

impl<'a> Dialog<'a> {
    pub fn new(id: impl Into<WidgetText>, text: impl Into<RichText>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            primary_text: "Ok".into(),
            secondary_text: None,
            tertiary_text: None,
            open: None,
        }
    }

    pub fn open(mut self, open: &'a mut bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn primary_text(mut self, primary_text: impl Into<WidgetText>) -> Self {
        self.primary_text = primary_text.into();
        self
    }

    pub fn secondary_text(mut self, secondary_text: impl Into<WidgetText>) -> Self {
        self.secondary_text = Some(secondary_text.into());
        self
    }

    pub fn show(self, ctx: &Context) -> Option<InnerResponse<Option<DialogResponse>>> {
        let is_explicitly_closed = matches!(self.open, Some(false));

        let is_open = !is_explicitly_closed || ctx.memory(|mem| mem.everything_is_visible());

        if !is_open {
            return None;
        }

        let inner_resp = Window::new(self.id)
            .pivot(Align2::CENTER_CENTER)
            .default_pos(ctx.screen_rect().center())
            .resizable(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.allocate_space(vec2(ui.available_width(), 10.0));

                ui.vertical_centered(|ui| {
                    ui.heading(self.text);
                });

                ui.allocate_space(vec2(ui.available_width(), 20.0));

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    let mut response = DialogResponse::Idle;

                    if let Some(tertiary_text) = self.tertiary_text {
                        if ui.button(tertiary_text).clicked() {
                            response = DialogResponse::Tertiary;
                        }
                    }

                    if let Some(secondary_text) = self.secondary_text {
                        if ui.button(secondary_text).clicked() {
                            response = DialogResponse::Secondary;
                        }
                    }

                    if ui.button(self.primary_text).clicked() {
                        response = DialogResponse::Primary;
                    }

                    if response.is_some() {
                        if let Some(open) = self.open {
                            *open = !*open;
                        }
                    }

                    response
                })
                .inner
            });

        inner_resp
    }

    pub fn tertiary_text(mut self, tertiary_text: impl Into<WidgetText>) -> Self {
        self.tertiary_text = Some(tertiary_text.into());
        self
    }
}
