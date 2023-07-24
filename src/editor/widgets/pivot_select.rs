use eframe::{
    egui::{DragValue, Id, Response, Sense, Ui, Widget},
    epaint::{vec2, Color32, Pos2, Rect, Rounding, Stroke, Vec2},
};

pub struct PivotSelect<'a> {
    x: &'a mut f32,
    y: &'a mut f32,
    width: f32,
    height: f32,
}

impl<'a> Widget for PivotSelect<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.monospace("x");
                ui.add(DragValue::new(self.x).speed(1));

                ui.monospace("y");
                ui.add(DragValue::new(self.y).speed(1));
            });

            let (resp, painter) = ui.allocate_painter(Vec2::splat(80.0), Sense::hover());

            let rect = resp.rect;

            let rounding = 2.0;

            let stroke = Stroke::new(1.0, Color32::from_gray(60));

            painter.rect_stroke(rect, 0.0, stroke);

            let button_size = Vec2::splat(20.0);

            pivot_control(
                "pivot_control_c",
                rect.center(),
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = self.width * 0.5;
                    *self.y = self.height * 0.5;
                },
            )
            .on_hover_text("Set to center");

            pivot_control(
                "pivot_control_t",
                rect.center_top() + vec2(0.0, button_size.y * 0.5),
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = self.width * 0.5;
                    *self.y = 0.0;
                },
            )
            .on_hover_text("Set to top");

            pivot_control(
                "pivot_control_tl",
                rect.left_top() + button_size * 0.5,
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = 0.0;
                    *self.y = 0.0;
                },
            )
            .on_hover_text("Set to top left");

            pivot_control(
                "pivot_control_l",
                rect.left_center() + vec2(button_size.x * 0.5, 0.0),
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = 0.0;
                    *self.y = self.height * 0.5;
                },
            )
            .on_hover_text("Set to left");

            pivot_control(
                "pivot_control_bl",
                rect.left_bottom() + vec2(button_size.x * 0.5, -button_size.y * 0.5),
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = 0.0;
                    *self.y = self.height;
                },
            )
            .on_hover_text("Set to bottom left");

            pivot_control(
                "pivot_control_b",
                rect.center_bottom() + vec2(0.0, -button_size.y * 0.5),
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = self.width * 0.5;
                    *self.y = self.height;
                },
            )
            .on_hover_text("Set to bottom");

            pivot_control(
                "pivot_control_br",
                rect.right_bottom() - button_size * 0.5,
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = self.width;
                    *self.y = self.height;
                },
            )
            .on_hover_text("Set to bottom right");

            pivot_control(
                "pivot_control_r",
                rect.right_center() + vec2(-button_size.x * 0.5, 0.0),
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = self.width;
                    *self.y = self.height * 0.5;
                },
            )
            .on_hover_text("Set to right");

            pivot_control(
                "pivot_control_tr",
                rect.right_top() + vec2(-button_size.x * 0.5, button_size.y * 0.5),
                button_size,
                rounding,
                stroke,
                ui,
                || {
                    *self.x = self.width;
                    *self.y = 0.0;
                },
            )
            .on_hover_text("Set to top right");
        })
        .response
    }
}

impl<'a> PivotSelect<'a> {
    pub fn new(x: &'a mut f32, y: &'a mut f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

fn pivot_control(
    id: impl Into<Id>,
    center_point: Pos2,
    size: Vec2,
    rounding: impl Into<Rounding>,
    stroke: Stroke,
    ui: &mut Ui,
    mut on_click: impl FnMut(),
) -> Response {
    let rect = Rect::from_center_size(center_point, size);

    let rounding = rounding.into();

    ui.painter_at(rect).rect_stroke(rect, rounding, stroke);

    let resp = ui.interact(rect, id.into(), Sense::drag());

    if resp.hovered() {
        ui.painter_at(rect)
            .rect_filled(rect, rounding, stroke.color);
    }

    if resp.clicked() {
        on_click();
    }

    resp
}
