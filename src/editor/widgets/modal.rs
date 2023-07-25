use eframe::{
    egui::{Area, Context, Id, InnerResponse, Order, Sense},
    epaint::{Color32, Pos2, Rect},
};

pub struct Modal<'a> {
    id: Id,

    position: Pos2,
    area: Option<Rect>,
    modal_color: Color32,
    open: Option<&'a mut bool>,
}

impl<'a> Modal<'a> {
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            position: Pos2::ZERO,
            area: None,
            modal_color: Color32::from_black_alpha(90),
            open: None,
        }
    }

    pub fn area(mut self, area: Rect) -> Self {
        self.area = Some(area);
        self
    }

    pub fn modal_color(mut self, modal_color: Color32) -> Self {
        self.modal_color = modal_color;
        self
    }

    pub fn open(mut self, open: &'a mut bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn position(mut self, position: impl Into<Pos2>) -> Self {
        self.position = position.into();
        self
    }

    pub fn show<R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&Context) -> Option<InnerResponse<Option<R>>>,
    ) -> Option<InnerResponse<Option<R>>> {
        let is_explicitly_closed = matches!(self.open, Some(false));

        let is_open = !is_explicitly_closed || ctx.memory(|mem| mem.everything_is_visible());

        if !is_open {
            return None;
        }

        let inner_resp = Area::new(self.id)
            .fixed_pos(self.position)
            .order(Order::Background)
            .show(ctx, |ui| {
                let area = self.area.unwrap_or(ui.input(|input| input.screen_rect));

                let resp = ui.allocate_response(area.size(), Sense::hover());

                ui.painter_at(resp.rect)
                    .rect_filled(area, 0.0, self.modal_color);

                let inner_resp = add_contents(ctx);

                inner_resp
            });

        inner_resp.inner
    }
}
