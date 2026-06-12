//! Rendering helpers for Observatory setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_observatory_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::ObservationDeck => draw_observation_deck(view, world_rect),
        SetpieceKind::TelescopeGantry => draw_telescope_gantry(view, world_rect),
        SetpieceKind::ShutterZone => draw_shutter_zone(view, world_rect),
        SetpieceKind::SearchlightBeam => draw_searchlight_beam(view, world_rect),
        SetpieceKind::RadiationLock => draw_radiation_lock(view, world_rect),
        SetpieceKind::ShadowLane => draw_shadow_lane(view, world_rect),
        SetpieceKind::GlareLane => draw_glare_lane(view, world_rect),
        _ => {}
    }
}

fn draw_observation_deck(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.45,
        rect.w,
        rect.h * 0.32,
        Color::new(0.10, 0.13, 0.17, 0.72),
    );
    for i in 0..8 {
        let x = rect.x + i as f32 * rect.w / 8.0;
        draw_line(
            x,
            rect.y + rect.h * 0.38,
            x + rect.w * 0.08,
            rect.y + rect.h * 0.66,
            1.5,
            Color::new(0.70, 0.82, 0.92, 0.20),
        );
    }
}

fn draw_telescope_gantry(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.72, 0.82, 0.88, 0.26),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.5,
        rect_right(rect),
        rect.y + rect.h * 0.5,
        5.0,
        Color::new(0.38, 0.46, 0.52, 0.74),
    );
    draw_circle(
        rect.x + rect.w * 0.28,
        rect.y + rect.h * 0.35,
        rect.h * 0.16,
        Color::new(0.56, 0.70, 0.78, 0.42),
    );
}

fn draw_shutter_zone(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.86, 0.94, 1.0, 0.24),
    );
    for i in 0..5 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.18);
        draw_rectangle(
            x,
            rect.y,
            rect.w * 0.08,
            rect.h,
            Color::new(0.20, 0.24, 0.28, 0.56),
        );
    }
}

fn draw_searchlight_beam(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_triangle(
        vec2(rect.x + rect.w * 0.15, rect.y),
        vec2(rect_right(rect), rect.y + rect.h * 0.35),
        vec2(rect_right(rect), rect.y + rect.h * 0.82),
        Color::new(0.92, 0.98, 1.0, 0.14),
    );
}

fn draw_radiation_lock(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.13, 0.10, 0.18, 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(1.0, 0.86, 0.36, 0.38),
    );
    draw_circle_lines(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.45,
        rect.w * 0.22,
        2.0,
        Color::new(1.0, 0.86, 0.36, 0.32),
    );
}

fn draw_shadow_lane(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.02, 0.025, 0.05, 0.46),
    );
    for i in 0..4 {
        let x = rect.x + i as f32 * rect.w / 4.0;
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.12,
            rect_bottom(rect),
            2.0,
            Color::new(0.18, 0.24, 0.36, 0.28),
        );
    }
}

fn draw_glare_lane(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.18);
        draw_line(
            x,
            rect_bottom(rect),
            x + rect.w * 0.10,
            rect.y,
            3.0,
            Color::new(1.0, 0.88, 0.42, 0.30),
        );
    }
}
