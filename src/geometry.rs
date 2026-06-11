//! Shared world dimensions and rectangle helpers.

use macroquad::prelude::*;

pub const WORLD_WIDTH: f32 = 1120.0;
pub const WORLD_HEIGHT: f32 = 560.0;
pub const FLOOR_Y: f32 = 500.0;
pub const PLAYER_W: f32 = 28.0;
pub const PLAYER_H: f32 = 46.0;

pub fn rect_right(rect: Rect) -> f32 {
    rect.x + rect.w
}

pub fn rect_bottom(rect: Rect) -> f32 {
    rect.y + rect.h
}

pub fn rect_center(rect: Rect) -> Vec2 {
    vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5)
}

pub fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x < rect_right(b) && rect_right(a) > b.x && a.y < rect_bottom(b) && rect_bottom(a) > b.y
}

pub fn inflate(rect: Rect, amount: f32) -> Rect {
    Rect::new(
        rect.x - amount,
        rect.y - amount,
        rect.w + amount * 2.0,
        rect.h + amount * 2.0,
    )
}

pub fn horizontal_overlap(a: Rect, b: Rect) -> bool {
    a.x < rect_right(b) && rect_right(a) > b.x
}

pub fn line_intersects_rect(start: Vec2, end: Vec2, rect: Rect) -> bool {
    let steps = 18;
    for i in 1..steps {
        let t = i as f32 / steps as f32;
        let point = start.lerp(end, t);
        if point.x >= rect.x
            && point.x <= rect_right(rect)
            && point.y >= rect.y
            && point.y <= rect_bottom(rect)
        {
            return true;
        }
    }
    false
}
