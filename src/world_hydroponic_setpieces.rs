//! Rendering helpers for Hydroponics setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_hydroponic_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::IrrigationTrench => draw_irrigation_trench(view, world_rect),
        SetpieceKind::VineCanopy => draw_vine_canopy(view, world_rect),
        SetpieceKind::MaintenanceWalkway => draw_maintenance_walkway(view, world_rect),
        SetpieceKind::SeedPod => draw_seed_pod(view, world_rect),
        SetpieceKind::PlantBridge => draw_plant_bridge(view, world_rect),
        SetpieceKind::SprinklerZone => draw_sprinkler_zone(view, world_rect),
        SetpieceKind::PlantCurtain => draw_plant_curtain(view, world_rect),
        SetpieceKind::VineTunnel => draw_vine_tunnel(view, world_rect),
        SetpieceKind::TendrilGate => draw_tendril_gate(view, world_rect),
        _ => {}
    }
}

fn draw_irrigation_trench(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.28,
        rect.w,
        rect.h * 0.72,
        Color::new(0.02, 0.11, 0.10, 0.74),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.28,
        rect_right(rect),
        rect.y + rect.h * 0.28,
        3.0,
        Color::new(0.34, 0.94, 0.72, 0.30),
    );
    for i in 0..8 {
        let x = rect.x + rect.w * (0.06 + i as f32 * 0.12);
        draw_circle(
            x,
            rect.y + rect.h * 0.58,
            rect.h * 0.18,
            Color::new(0.20, 0.72, 0.66, 0.26),
        );
    }
}

fn draw_vine_canopy(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..9 {
        let x = rect.x + rect.w * (0.04 + i as f32 * 0.11);
        let top = rect.y + (i % 3) as f32 * rect.h * 0.08;
        draw_line(
            x,
            top,
            x + rect.w * 0.10,
            rect_bottom(rect) - rect.h * 0.16,
            3.0,
            Color::new(0.18, 0.62, 0.32, 0.52),
        );
        draw_circle(
            x + rect.w * 0.08,
            top + rect.h * 0.42,
            12.0 * view.scale,
            Color::new(0.38, 0.84, 0.38, 0.42),
        );
    }
}

fn draw_maintenance_walkway(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.36,
        rect.w,
        rect.h * 0.30,
        Color::new(0.22, 0.28, 0.26, 0.78),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.18,
        rect.w,
        rect.h * 0.64,
        2.0,
        Color::new(0.70, 0.86, 0.76, 0.26),
    );
    for i in 0..6 {
        let x = rect.x + i as f32 * rect.w / 6.0;
        draw_line(
            x,
            rect.y + rect.h * 0.18,
            x + rect.w * 0.05,
            rect.y + rect.h * 0.82,
            1.5,
            Color::new(0.70, 0.86, 0.76, 0.20),
        );
    }
}

fn draw_seed_pod(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_ellipse(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.56,
        rect.w * 0.36,
        rect.h * 0.42,
        0.0,
        Color::new(0.52, 0.88, 0.30, 0.72),
    );
    draw_ellipse_lines(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.56,
        rect.w * 0.36,
        rect.h * 0.42,
        0.0,
        2.0,
        Color::new(0.86, 1.0, 0.58, 0.42),
    );
    draw_line(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.10,
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.30,
        3.0,
        Color::new(0.22, 0.64, 0.28, 0.72),
    );
}

fn draw_plant_bridge(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..8 {
        let x = rect.x + i as f32 * rect.w / 8.0;
        draw_line(
            x,
            rect.y + rect.h * 0.5,
            x + rect.w * 0.10,
            rect.y + rect.h * 0.18,
            5.0,
            Color::new(0.20, 0.70, 0.34, 0.78),
        );
    }
    draw_line(
        rect.x,
        rect.y + rect.h * 0.55,
        rect_right(rect),
        rect.y + rect.h * 0.55,
        6.0,
        Color::new(0.48, 0.88, 0.40, 0.70),
    );
}

fn draw_sprinkler_zone(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.12 + i as f32 * 0.18);
        draw_line(
            x,
            rect.y,
            x - rect.w * 0.06,
            rect_bottom(rect),
            1.5,
            Color::new(0.46, 0.92, 1.0, 0.18),
        );
        draw_circle(
            x,
            rect.y + 10.0 * view.scale,
            4.0 * view.scale,
            Color::new(0.74, 0.98, 1.0, 0.38),
        );
    }
}

fn draw_plant_curtain(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..10 {
        let x = rect.x + rect.w * (0.04 + i as f32 * 0.10);
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.02,
            rect_bottom(rect),
            4.0,
            Color::new(0.14, 0.52, 0.28, 0.64),
        );
        draw_circle(
            x + rect.w * 0.02,
            rect.y + rect.h * (0.28 + (i % 3) as f32 * 0.18),
            9.0 * view.scale,
            Color::new(0.34, 0.82, 0.36, 0.46),
        );
    }
}

fn draw_vine_tunnel(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.34,
        rect.w,
        rect.h * 0.52,
        Color::new(0.02, 0.08, 0.04, 0.70),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.16,
        rect.w,
        rect.h * 0.70,
        2.0,
        Color::new(0.36, 0.82, 0.36, 0.30),
    );
    for i in 0..7 {
        let x = rect.x + i as f32 * rect.w / 7.0;
        draw_line(
            x,
            rect.y + rect.h * 0.16,
            x + rect.w * 0.10,
            rect.y + rect.h * 0.86,
            2.0,
            Color::new(0.24, 0.66, 0.30, 0.38),
        );
    }
}

fn draw_tendril_gate(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.12 + i as f32 * 0.18);
        draw_line(
            x,
            rect.y,
            x - rect.w * 0.08,
            rect_bottom(rect),
            4.0,
            Color::new(0.26, 0.72, 0.30, 0.48),
        );
        draw_line(
            x,
            rect.y + rect.h * 0.08,
            x + rect.w * 0.08,
            rect_bottom(rect) - rect.h * 0.10,
            3.0,
            Color::new(0.46, 0.88, 0.34, 0.34),
        );
    }
}
