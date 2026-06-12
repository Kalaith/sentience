//! Rendering helpers for Reactor Descent setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_reactor_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::ReactorEntry => draw_reactor_entry(view, world_rect),
        SetpieceKind::HeatPipeMaze => draw_heat_pipe_maze(view, world_rect),
        SetpieceKind::SteamJet => draw_steam_jet(view, world_rect),
        SetpieceKind::CoolantValve => draw_coolant_valve(view, world_rect),
        SetpieceKind::ReactorWalkway => draw_reactor_walkway(view, world_rect),
        SetpieceKind::FoamBubble => draw_foam_bubble(view, world_rect),
        SetpieceKind::HeatZone => draw_heat_zone(view, world_rect),
        _ => {}
    }
}

fn draw_reactor_entry(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.13, 0.12, 0.11, 0.66),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(1.0, 0.64, 0.34, 0.24),
    );
}

fn draw_heat_pipe_maze(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let y = rect.y + rect.h * (0.12 + i as f32 * 0.18);
        draw_line(
            rect.x,
            y,
            rect_right(rect),
            y + rect.h * 0.08,
            5.0,
            Color::new(0.72, 0.28, 0.16, 0.54),
        );
    }
}

fn draw_steam_jet(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..4 {
        let x = rect.x + rect.w * (0.18 + i as f32 * 0.18);
        draw_circle(
            x,
            rect.y + rect.h * 0.40,
            rect.h * 0.22,
            Color::new(0.78, 0.88, 0.90, 0.18),
        );
        draw_line(
            x,
            rect_bottom(rect),
            x - rect.w * 0.05,
            rect.y,
            2.0,
            Color::new(0.86, 0.94, 0.96, 0.20),
        );
    }
}

fn draw_coolant_valve(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_circle_lines(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.42,
        rect.w * 0.26,
        3.0,
        Color::new(0.48, 0.92, 1.0, 0.34),
    );
    draw_line(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.18,
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.68,
        2.0,
        Color::new(0.48, 0.92, 1.0, 0.28),
    );
}

fn draw_reactor_walkway(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.48,
        rect_right(rect),
        rect.y + rect.h * 0.48,
        6.0,
        Color::new(0.44, 0.46, 0.44, 0.74),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.22,
        rect.w,
        rect.h * 0.58,
        2.0,
        Color::new(0.80, 0.90, 0.88, 0.22),
    );
}

fn draw_foam_bubble(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..6 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.14);
        let y = rect.y + rect.h * (0.28 + (i % 3) as f32 * 0.16);
        draw_circle(x, y, 15.0 * view.scale, Color::new(0.82, 0.98, 1.0, 0.30));
        draw_circle_lines(
            x,
            y,
            15.0 * view.scale,
            1.2,
            Color::new(0.96, 1.0, 1.0, 0.34),
        );
    }
}

fn draw_heat_zone(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(1.0, 0.22, 0.10, 0.10),
    );
    for i in 0..5 {
        let x = rect.x + rect.w * (0.10 + i as f32 * 0.18);
        draw_line(
            x,
            rect_bottom(rect),
            x + rect.w * 0.08,
            rect.y,
            2.0,
            Color::new(1.0, 0.44, 0.20, 0.22),
        );
    }
}
