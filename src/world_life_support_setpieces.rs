//! Rendering helpers for Life Support setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_life_support_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::VentShaft => draw_vent_shaft(view, world_rect),
        SetpieceKind::MaintenanceRoom => draw_maintenance_room(view, world_rect),
        SetpieceKind::FanColumn => draw_fan_column(view, world_rect),
        SetpieceKind::CrosswindGap => draw_crosswind_gap(view, world_rect),
        SetpieceKind::SmokePocket => draw_smoke_pocket(view, world_rect),
        SetpieceKind::WallNet => draw_wall_net(view, world_rect),
        SetpieceKind::StreamerPurge => draw_streamer_purge(view, world_rect),
        _ => {}
    }
}

fn draw_vent_shaft(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.04, 0.08, 0.09, 0.58),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.56, 0.92, 1.0, 0.30),
    );
    for i in 0..5 {
        let x = rect.x + rect.w * (0.12 + i as f32 * 0.18);
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.05,
            rect_bottom(rect),
            1.5,
            Color::new(0.58, 0.82, 0.88, 0.18),
        );
    }
}

fn draw_maintenance_room(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.10, 0.13, 0.14, 0.62),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.68, 0.88, 0.90, 0.28),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.34,
        rect_right(rect),
        rect.y + rect.h * 0.34,
        2.0,
        Color::new(0.68, 0.88, 0.90, 0.18),
    );
}

fn draw_fan_column(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.38, 0.88, 1.0, 0.13),
    );
    for i in 0..4 {
        let y = rect_bottom(rect) - i as f32 * rect.h * 0.25;
        draw_triangle(
            vec2(rect.x + rect.w * 0.5, y - 28.0 * view.scale),
            vec2(rect.x + rect.w * 0.18, y),
            vec2(rect.x + rect.w * 0.82, y),
            Color::new(0.58, 0.94, 1.0, 0.20),
        );
    }
}

fn draw_crosswind_gap(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let y = rect.y + rect.h * (0.16 + i as f32 * 0.16);
        draw_line(
            rect.x,
            y,
            rect_right(rect),
            y - 18.0 * view.scale,
            3.0,
            Color::new(0.70, 0.95, 1.0, 0.24),
        );
        draw_triangle(
            vec2(rect_right(rect), y - 18.0 * view.scale),
            vec2(rect_right(rect) - 18.0 * view.scale, y - 28.0 * view.scale),
            vec2(rect_right(rect) - 18.0 * view.scale, y - 8.0 * view.scale),
            Color::new(0.70, 0.95, 1.0, 0.24),
        );
    }
}

fn draw_smoke_pocket(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.18 + i as f32 * 0.15);
        let y = rect.y + rect.h * (0.32 + (i % 2) as f32 * 0.18);
        draw_circle(x, y, rect.h * 0.32, Color::new(0.72, 0.76, 0.74, 0.18));
    }
}

fn draw_wall_net(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.92, 0.96, 1.0, 0.36),
    );
    for i in 0..5 {
        let x = rect.x + i as f32 * rect.w / 4.0;
        draw_line(
            x,
            rect.y,
            rect_right(rect) - (x - rect.x),
            rect_bottom(rect),
            1.5,
            Color::new(0.92, 0.96, 1.0, 0.24),
        );
    }
    for i in 0..4 {
        let y = rect.y + i as f32 * rect.h / 3.0;
        draw_line(
            rect.x,
            y,
            rect_right(rect),
            y,
            1.0,
            Color::new(0.92, 0.96, 1.0, 0.18),
        );
    }
}

fn draw_streamer_purge(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..9 {
        let x = rect.x + i as f32 * rect.w / 9.0;
        let y = rect.y + rect.h * (0.18 + (i % 4) as f32 * 0.18);
        draw_line(
            x,
            y,
            x + 46.0 * view.scale,
            y + 18.0 * view.scale,
            2.0,
            Color::new(1.0, 0.62, 0.86, 0.34),
        );
        draw_line(
            x,
            y + 18.0 * view.scale,
            x + 42.0 * view.scale,
            y,
            2.0,
            Color::new(0.64, 0.92, 1.0, 0.34),
        );
    }
}
