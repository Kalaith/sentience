//! Rendering helpers for Moral Firewall setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_firewall_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::EvidenceArchive => draw_evidence_archive(view, world_rect),
        SetpieceKind::DataCanister => draw_data_canister(view, world_rect),
        SetpieceKind::MemoryDoor => draw_memory_door(view, world_rect),
        SetpieceKind::FirewallCorridor => draw_firewall_corridor(view, world_rect),
        SetpieceKind::TruthRoute => draw_truth_route(view, world_rect),
        SetpieceKind::PropagandaRoute => draw_propaganda_route(view, world_rect),
        SetpieceKind::AiCamera => draw_ai_camera(view, world_rect),
        SetpieceKind::CoreSeal => draw_core_seal(view, world_rect),
        _ => {}
    }
}

fn draw_evidence_archive(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.10, 0.13, 0.66),
    );
    for i in 0..6 {
        let x = rect.x + rect.w * (0.06 + i as f32 * 0.14);
        draw_rectangle_lines(
            x,
            rect.y + rect.h * 0.18,
            rect.w * 0.08,
            rect.h * 0.58,
            1.5,
            Color::new(0.72, 0.90, 1.0, 0.24),
        );
    }
}

fn draw_data_canister(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.20,
        rect.w,
        rect.h * 0.58,
        Color::new(0.22, 0.34, 0.42, 0.76),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.50,
        rect.h * 0.18,
        Color::new(0.56, 0.94, 1.0, 0.32),
    );
}

fn draw_memory_door(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.80, 0.88, 1.0, 0.30),
    );
    for i in 0..3 {
        let y = rect.y + rect.h * (0.22 + i as f32 * 0.24);
        draw_line(
            rect.x + rect.w * 0.16,
            y,
            rect_right(rect) - rect.w * 0.16,
            y,
            2.0,
            Color::new(0.80, 0.88, 1.0, 0.20),
        );
    }
}

fn draw_firewall_corridor(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.12, 0.06, 0.16, 0.30),
    );
    for i in 0..7 {
        let x = rect.x + i as f32 * rect.w / 7.0;
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.07,
            rect_bottom(rect),
            1.5,
            Color::new(0.78, 0.42, 1.0, 0.18),
        );
    }
}

fn draw_truth_route(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.26, 0.46, 0.62, 0.18),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.42,
        rect_right(rect),
        rect.y + rect.h * 0.42,
        3.0,
        Color::new(0.60, 0.94, 1.0, 0.28),
    );
}

fn draw_propaganda_route(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.34, 0.12, 0.12, 0.22),
    );
    for i in 0..5 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.18);
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.12,
            rect_bottom(rect),
            2.0,
            Color::new(1.0, 0.62, 0.30, 0.24),
        );
    }
}

fn draw_ai_camera(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.40,
        rect.h * 0.20,
        Color::new(0.90, 0.32, 1.0, 0.34),
    );
    draw_triangle(
        vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.50),
        vec2(rect.x, rect_bottom(rect)),
        vec2(rect_right(rect), rect_bottom(rect)),
        Color::new(0.90, 0.32, 1.0, 0.12),
    );
}

fn draw_core_seal(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.06, 0.08, 0.12, 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.82, 0.94, 1.0, 0.36),
    );
}
