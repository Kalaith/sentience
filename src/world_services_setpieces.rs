//! Rendering helpers for Crew Services setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_services_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::LaundryFloor => draw_laundry_floor(view, world_rect),
        SetpieceKind::LaundryTube => draw_laundry_tube(view, world_rect),
        SetpieceKind::SuctionBurst => draw_suction_burst(view, world_rect),
        SetpieceKind::TubeExit => draw_tube_exit(view, world_rect),
        SetpieceKind::RollingLaundryCart => draw_laundry_cart(view, world_rect),
        SetpieceKind::LaundryBin => draw_laundry_bin(view, world_rect),
        SetpieceKind::UniformRetrievalTube => draw_uniform_retrieval_tube(view, world_rect),
        _ => {}
    }
}

fn draw_laundry_floor(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.42,
        rect.w,
        rect.h * 0.34,
        Color::new(0.12, 0.15, 0.17, 0.68),
    );
    for i in 0..7 {
        let x = rect.x + rect.w * (0.06 + i as f32 * 0.13);
        draw_circle(
            x,
            rect.y + rect.h * 0.56,
            8.0 * view.scale,
            Color::new(0.78, 0.88, 0.92, 0.20),
        );
    }
}

fn draw_laundry_tube(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.5,
        rect_right(rect),
        rect.y + rect.h * 0.5,
        12.0,
        Color::new(0.42, 0.58, 0.64, 0.64),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.28,
        rect_right(rect),
        rect.y + rect.h * 0.28,
        2.0,
        Color::new(0.82, 0.94, 1.0, 0.24),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.72,
        rect_right(rect),
        rect.y + rect.h * 0.72,
        2.0,
        Color::new(0.82, 0.94, 1.0, 0.18),
    );
}

fn draw_suction_burst(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.18);
        draw_line(
            x,
            rect_bottom(rect),
            x + rect.w * 0.16,
            rect.y,
            3.0,
            Color::new(0.66, 0.94, 1.0, 0.26),
        );
    }
}

fn draw_tube_exit(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_circle_lines(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.5,
        rect.w * 0.35,
        3.0,
        Color::new(0.74, 0.96, 1.0, 0.34),
    );
    draw_rectangle(
        rect.x + rect.w * 0.30,
        rect.y + rect.h * 0.36,
        rect.w * 0.40,
        rect.h * 0.28,
        Color::new(0.12, 0.18, 0.20, 0.72),
    );
}

fn draw_laundry_cart(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.68,
        Color::new(0.46, 0.52, 0.56, 0.78),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.68,
        1.5,
        Color::new(0.86, 0.94, 0.98, 0.28),
    );
    for x in [rect.x + rect.w * 0.24, rect.x + rect.w * 0.76] {
        draw_circle(
            x,
            rect.y + rect.h * 0.78,
            5.0 * view.scale,
            Color::new(0.06, 0.07, 0.08, 0.90),
        );
    }
}

fn draw_laundry_bin(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.18,
        rect.w,
        rect.h * 0.62,
        Color::new(0.20, 0.26, 0.28, 0.80),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.18,
        rect.w,
        rect.h * 0.62,
        1.5,
        Color::new(0.84, 0.96, 1.0, 0.28),
    );
    draw_line(
        rect.x + rect.w * 0.12,
        rect.y + rect.h * 0.34,
        rect_right(rect) - rect.w * 0.12,
        rect.y + rect.h * 0.34,
        2.0,
        Color::new(0.84, 0.96, 1.0, 0.20),
    );
}

fn draw_uniform_retrieval_tube(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..6 {
        let x = rect.x + rect.w * (0.06 + i as f32 * 0.15);
        let y = rect.y + rect.h * (0.28 + (i % 2) as f32 * 0.24);
        draw_circle(x, y, 8.0 * view.scale, Color::new(1.0, 0.76, 0.34, 0.30));
        draw_line(
            x,
            y,
            x + rect.w * 0.08,
            y + rect.h * 0.12,
            2.0,
            Color::new(1.0, 0.76, 0.34, 0.28),
        );
    }
}
