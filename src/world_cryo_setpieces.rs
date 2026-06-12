//! Rendering helpers for Cryonics setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_cryo_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::FrozenAisle => draw_frozen_aisle(view, world_rect),
        SetpieceKind::CryoPodRoom => draw_cryo_pod_room(view, world_rect),
        SetpieceKind::ThawSwitch => draw_thaw_switch(view, world_rect),
        SetpieceKind::ColdServicePipe => draw_cold_service_pipe(view, world_rect),
        SetpieceKind::SleeperPod => draw_sleeper_pod(view, world_rect),
        SetpieceKind::ThermalBlanketCover => draw_thermal_blanket_cover(view, world_rect),
        _ => {}
    }
}

fn draw_frozen_aisle(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.58, 0.90, 1.0, 0.16),
    );
    for i in 0..12 {
        let x = rect.x + i as f32 * rect.w / 12.0;
        draw_line(
            x,
            rect.y + rect.h * 0.25,
            x + rect.w * 0.08,
            rect_bottom(rect) - rect.h * 0.12,
            1.2,
            Color::new(0.86, 0.98, 1.0, 0.28),
        );
    }
}

fn draw_cryo_pod_room(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.06, 0.12, 0.16, 0.62),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.58, 0.92, 1.0, 0.30),
    );
    for i in 0..3 {
        let x = rect.x + rect.w * (0.18 + i as f32 * 0.28);
        draw_rectangle_lines(
            x,
            rect.y + rect.h * 0.22,
            rect.w * 0.17,
            rect.h * 0.56,
            1.5,
            Color::new(0.74, 0.96, 1.0, 0.24),
        );
    }
}

fn draw_thaw_switch(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x + rect.w * 0.30,
        rect.y + rect.h * 0.12,
        rect.w * 0.40,
        rect.h * 0.76,
        Color::new(0.12, 0.18, 0.20, 0.86),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.36,
        rect.w * 0.15,
        Color::new(1.0, 0.70, 0.32, 0.58),
    );
    draw_line(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.54,
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.76,
        3.0,
        Color::new(1.0, 0.82, 0.42, 0.48),
    );
}

fn draw_cold_service_pipe(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.48,
        rect_right(rect),
        rect.y + rect.h * 0.48,
        8.0,
        Color::new(0.48, 0.70, 0.78, 0.72),
    );
    for i in 0..7 {
        let x = rect.x + i as f32 * rect.w / 7.0;
        draw_circle(
            x,
            rect.y + rect.h * 0.48,
            9.0 * view.scale,
            Color::new(0.78, 0.96, 1.0, 0.30),
        );
    }
}

fn draw_sleeper_pod(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.20,
        rect.w,
        rect.h * 0.60,
        Color::new(0.15, 0.24, 0.28, 0.78),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.20,
        rect.w,
        rect.h * 0.60,
        1.5,
        Color::new(0.70, 0.96, 1.0, 0.34),
    );
    draw_circle(
        rect.x + rect.w * 0.30,
        rect.y + rect.h * 0.50,
        rect.h * 0.16,
        Color::new(0.90, 0.96, 1.0, 0.28),
    );
}

fn draw_thermal_blanket_cover(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.18);
        draw_rectangle(
            x,
            rect.y + (i % 2) as f32 * rect.h * 0.12,
            rect.w * 0.12,
            rect.h * 0.66,
            Color::new(0.94, 0.72, 0.28, 0.44),
        );
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.10,
            rect_bottom(rect),
            1.5,
            Color::new(1.0, 0.92, 0.62, 0.24),
        );
    }
}
