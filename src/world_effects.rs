//! Shared world-space effect overlays.

use crate::state::Ambience;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_stars(view: &WorldView) {
    let window = view.rect(Rect::new(view.visible_left() + 34.0, 44.0, 1052.0, 96.0));
    draw_rectangle(
        window.x,
        window.y,
        window.w,
        window.h,
        Color::new(0.005, 0.008, 0.018, 0.88),
    );
    draw_rectangle_lines(
        window.x,
        window.y,
        window.w,
        window.h,
        view.scale * 5.0,
        Color::new(0.35, 0.48, 0.55, 0.35),
    );

    for i in 0..54 {
        let x = view.visible_left() + 52.0 + ((i * 83) % 1010) as f32;
        let y = 58.0 + ((i * 37) % 64) as f32;
        let p = view.point(vec2(x, y));
        let radius = if i % 7 == 0 { 1.7 } else { 1.0 };
        draw_circle(p.x, p.y, radius, Color::new(0.80, 0.92, 1.0, 0.70));
    }
}

pub(crate) fn draw_ambience_overlays(ambience: Ambience, rect: Rect, view: &WorldView) {
    if ambience.smoke {
        for i in 0..9 {
            let x = rect.x + 70.0 + i as f32 * 92.0 + (i % 3) as f32 * 16.0;
            let y = rect.y + 235.0 + (i % 4) as f32 * 21.0;
            draw_circle(x, y, 52.0, Color::new(0.68, 0.72, 0.70, 0.13));
        }
    }
    if ambience.darkness {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.0, 0.0, 0.0, 0.38),
        );
    }
    if ambience.sparks {
        for i in 0..12 {
            let start = view.point(vec2(
                190.0 + (i * 73 % 830) as f32,
                174.0 + (i * 31 % 240) as f32,
            ));
            draw_line(
                start.x,
                start.y,
                start.x + 8.0,
                start.y + 14.0,
                1.5,
                Color::new(1.0, 0.62, 0.22, 0.65),
            );
        }
    }
    if ambience.gravity_off {
        for i in 0..7 {
            let p = view.point(vec2(
                230.0 + i as f32 * 112.0,
                250.0 + (i % 2) as f32 * 44.0,
            ));
            draw_rectangle_ex(
                p.x,
                p.y,
                14.0,
                8.0,
                DrawRectangleParams {
                    rotation: 0.35 + i as f32 * 0.17,
                    color: Color::new(0.68, 0.76, 0.80, 0.58),
                    ..Default::default()
                },
            );
        }
    }
}
