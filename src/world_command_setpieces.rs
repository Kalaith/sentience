//! Rendering helpers for Executive Deck setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_command_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::TrophyHall => draw_trophy_hall(view, world_rect),
        SetpieceKind::CommandDesk => draw_command_desk(view, world_rect),
        SetpieceKind::EvidenceSafe => draw_evidence_safe(view, world_rect),
        SetpieceKind::PrivateEscapeCorridor => draw_private_escape_corridor(view, world_rect),
        SetpieceKind::BriefingRoom => draw_briefing_room(view, world_rect),
        SetpieceKind::CommandLock => draw_command_lock(view, world_rect),
        SetpieceKind::FalseOrderTrail => draw_false_order_trail(view, world_rect),
        _ => {}
    }
}

fn draw_trophy_hall(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.12, 0.12, 0.14, 0.58),
    );
    for i in 0..5 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.18);
        draw_rectangle_lines(
            x,
            rect.y + rect.h * 0.18,
            rect.w * 0.10,
            rect.h * 0.58,
            1.5,
            Color::new(0.94, 0.82, 0.42, 0.26),
        );
        draw_circle(
            x + rect.w * 0.05,
            rect.y + rect.h * 0.36,
            8.0 * view.scale,
            Color::new(0.94, 0.82, 0.42, 0.28),
        );
    }
}

fn draw_command_desk(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.35,
        rect.w,
        rect.h * 0.42,
        Color::new(0.18, 0.21, 0.24, 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.35,
        rect.w,
        rect.h * 0.42,
        1.5,
        Color::new(0.72, 0.90, 1.0, 0.28),
    );
}

fn draw_evidence_safe(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.10, 0.12, 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.96, 0.82, 0.38, 0.34),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.48,
        rect.w * 0.18,
        Color::new(0.96, 0.82, 0.38, 0.26),
    );
}

fn draw_private_escape_corridor(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.36,
        rect.w,
        rect.h * 0.38,
        Color::new(0.04, 0.07, 0.08, 0.72),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.20,
        rect.w,
        rect.h * 0.54,
        2.0,
        Color::new(0.64, 0.84, 0.90, 0.22),
    );
}

fn draw_briefing_room(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.10, 0.13, 0.15, 0.60),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.74, 0.86, 0.90, 0.24),
    );
    draw_rectangle(
        rect.x + rect.w * 0.18,
        rect.y + rect.h * 0.54,
        rect.w * 0.64,
        rect.h * 0.16,
        Color::new(0.24, 0.30, 0.32, 0.74),
    );
}

fn draw_command_lock(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.16, 0.10, 0.12, 0.78),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(1.0, 0.62, 0.36, 0.34),
    );
    draw_line(
        rect.x + rect.w * 0.24,
        rect.y + rect.h * 0.5,
        rect_right(rect) - rect.w * 0.24,
        rect.y + rect.h * 0.5,
        2.0,
        Color::new(1.0, 0.62, 0.36, 0.26),
    );
}

fn draw_false_order_trail(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..6 {
        let x = rect.x + rect.w * (0.06 + i as f32 * 0.15);
        let y = rect.y + rect.h * (0.24 + (i % 2) as f32 * 0.28);
        draw_rectangle(
            x,
            y,
            18.0 * view.scale,
            12.0 * view.scale,
            Color::new(1.0, 0.82, 0.34, 0.32),
        );
        draw_line(
            x,
            y,
            x + rect.w * 0.08,
            rect_bottom(rect),
            1.5,
            Color::new(1.0, 0.82, 0.34, 0.22),
        );
    }
}
