//! Texture drawing helpers shared by immediate-mode renderers.

use crate::geometry::rect_bottom;
use crate::ui::UiContext;
use macroquad::prelude::*;

pub(crate) fn draw_texture_fill(ctx: &UiContext<'_>, key: &str, rect: Rect, tint: Color) -> bool {
    let Some(texture) = ctx.assets.get_texture(key) else {
        return false;
    };
    draw_texture_ex(
        texture,
        rect.x,
        rect.y,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(rect.w, rect.h)),
            ..Default::default()
        },
    );
    true
}

pub(crate) fn draw_texture_fit(
    ctx: &UiContext<'_>,
    key: &str,
    rect: Rect,
    tint: Color,
    flip_x: bool,
    anchor_bottom: bool,
) -> bool {
    let Some(texture) = ctx.assets.get_texture(key) else {
        return false;
    };

    let texture_w = texture.width().max(1.0);
    let texture_h = texture.height().max(1.0);
    let aspect = texture_w / texture_h;
    let mut width = rect.w;
    let mut height = width / aspect;
    if height > rect.h {
        height = rect.h;
        width = height * aspect;
    }

    let x = rect.x + (rect.w - width) * 0.5;
    let y = if anchor_bottom {
        rect_bottom(rect) - height
    } else {
        rect.y + (rect.h - height) * 0.5
    };
    let source = flip_x.then_some(Rect::new(texture_w, 0.0, -texture_w, texture_h));

    draw_texture_ex(
        texture,
        x,
        y,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(width, height)),
            source,
            ..Default::default()
        },
    );
    true
}
