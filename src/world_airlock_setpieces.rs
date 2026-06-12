//! Rendering helpers for External Maintenance setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_airlock_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::AirlockStaging => draw_airlock_staging(view, world_rect),
        SetpieceKind::PressureChamber => draw_pressure_chamber(view, world_rect),
        SetpieceKind::PressureDoor => draw_pressure_door(view, world_rect),
        SetpieceKind::ExteriorMaintenanceStrip => draw_exterior_strip(view, world_rect),
        SetpieceKind::TetherAnchor => draw_tether_anchor(view, world_rect),
        SetpieceKind::SafetyNet => draw_safety_net(view, world_rect),
        SetpieceKind::PressureBurst => draw_pressure_burst(view, world_rect),
        _ => {}
    }
}

fn draw_airlock_staging(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.12, 0.15, 0.72),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.70, 0.86, 0.95, 0.26),
    );
    for i in 0..4 {
        let x = rect.x + rect.w * (0.15 + i as f32 * 0.2);
        draw_line(
            x,
            rect.y + 8.0 * view.scale,
            x,
            rect_bottom(rect) - 8.0 * view.scale,
            1.5,
            Color::new(0.72, 0.88, 0.96, 0.18),
        );
    }
}

fn draw_pressure_chamber(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.50, 0.78, 1.0, 0.30),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.50,
        rect_right(rect),
        rect.y + rect.h * 0.50,
        1.5,
        Color::new(0.50, 0.78, 1.0, 0.16),
    );
    for i in 0..3 {
        let x = rect.x + rect.w * (0.2 + i as f32 * 0.28);
        draw_circle_lines(
            x,
            rect.y + rect.h * 0.22,
            10.0 * view.scale,
            1.5,
            Color::new(0.58, 0.90, 1.0, 0.30),
        );
    }
}

fn draw_pressure_door(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x + rect.w * 0.18,
        rect.y,
        rect.w * 0.64,
        rect.h,
        Color::new(0.14, 0.18, 0.20, 0.84),
    );
    draw_rectangle_lines(
        rect.x + rect.w * 0.18,
        rect.y,
        rect.w * 0.64,
        rect.h,
        2.0,
        Color::new(0.78, 0.96, 1.0, 0.34),
    );
    draw_line(
        rect.x + rect.w * 0.5,
        rect.y,
        rect.x + rect.w * 0.5,
        rect_bottom(rect),
        3.0,
        Color::new(0.46, 0.86, 1.0, 0.25),
    );
}

fn draw_exterior_strip(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.38,
        rect.w,
        rect.h * 0.24,
        Color::new(0.18, 0.22, 0.26, 0.68),
    );
    for i in 0..8 {
        let x = rect.x + i as f32 * rect.w / 8.0;
        draw_line(
            x,
            rect.y + rect.h * 0.35,
            x + rect.w * 0.05,
            rect.y + rect.h * 0.65,
            1.5,
            Color::new(0.80, 0.90, 1.0, 0.20),
        );
    }
}

fn draw_tether_anchor(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    draw_circle(
        center.x,
        center.y,
        rect.h * 0.28,
        Color::new(0.18, 0.24, 0.28, 0.86),
    );
    draw_circle_lines(
        center.x,
        center.y,
        rect.h * 0.34,
        2.0,
        Color::new(0.72, 0.96, 1.0, 0.42),
    );
    draw_line(
        center.x - rect.w * 0.36,
        center.y,
        center.x + rect.w * 0.36,
        center.y,
        2.0,
        Color::new(0.72, 0.96, 1.0, 0.34),
    );
}

fn draw_safety_net(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.94, 0.98, 1.0, 0.38),
    );
    for i in 0..5 {
        let x = rect.x + i as f32 * rect.w / 4.0;
        draw_line(
            x,
            rect.y,
            rect_right(rect) - (x - rect.x),
            rect_bottom(rect),
            1.5,
            Color::new(0.94, 0.98, 1.0, 0.22),
        );
    }
    for i in 0..3 {
        let y = rect.y + i as f32 * rect.h / 2.0;
        draw_line(
            rect.x,
            y,
            rect_right(rect),
            y,
            1.0,
            Color::new(0.94, 0.98, 1.0, 0.18),
        );
    }
}

fn draw_pressure_burst(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.10 + i as f32 * 0.18);
        let y = rect.y + rect.h * (0.22 + (i % 2) as f32 * 0.24);
        draw_line(
            x,
            y,
            x + rect.w * 0.18,
            y - rect.h * 0.18,
            3.0,
            Color::new(0.62, 0.90, 1.0, 0.28),
        );
        draw_triangle(
            vec2(x + rect.w * 0.18, y - rect.h * 0.18),
            vec2(x + rect.w * 0.13, y - rect.h * 0.23),
            vec2(x + rect.w * 0.15, y - rect.h * 0.12),
            Color::new(0.62, 0.90, 1.0, 0.28),
        );
    }
}
