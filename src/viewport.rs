use eframe::epaint::{Rect, Vec2};

pub struct Viewport {
    pub offset: Vec2,
    pub rect: Rect,
    pub scale: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            rect: Rect::NOTHING,
            scale: 1.0,
        }
    }
}
