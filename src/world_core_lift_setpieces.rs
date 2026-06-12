//! Rendering helpers for Core Lift setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_core_lift_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::LiftLanding => draw_lift_landing(view, world_rect),
        SetpieceKind::CentralElevator => draw_central_elevator(view, world_rect),
        SetpieceKind::ServiceLadder => draw_service_ladder(view, world_rect),
        SetpieceKind::LiftScheduler => draw_lift_scheduler(view, world_rect),
        SetpieceKind::WrongFloorDoor => draw_wrong_floor_door(view, world_rect),
        SetpieceKind::ElevatorSecurityUnit => draw_elevator_security_unit(view, world_rect),
        SetpieceKind::EmptyLiftWindow => draw_empty_lift_window(view, world_rect),
        _ => {}
    }
}

fn draw_lift_landing(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.5,
        rect_right(rect),
        rect.y + rect.h * 0.5,
        6.0,
        Color::new(0.42, 0.52, 0.56, 0.74),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.18,
        rect.w,
        rect.h * 0.64,
        2.0,
        Color::new(0.72, 0.90, 1.0, 0.24),
    );
}

fn draw_central_elevator(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.11, 0.13, 0.64),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.54, 0.90, 1.0, 0.30),
    );
    draw_line(
        rect.x + rect.w * 0.5,
        rect.y,
        rect.x + rect.w * 0.5,
        rect_bottom(rect),
        2.0,
        Color::new(0.54, 0.90, 1.0, 0.18),
    );
}

fn draw_service_ladder(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for x in [rect.x + rect.w * 0.35, rect.x + rect.w * 0.65] {
        draw_line(
            x,
            rect.y,
            x,
            rect_bottom(rect),
            2.0,
            Color::new(0.76, 0.88, 0.90, 0.30),
        );
    }
    for i in 0..7 {
        let y = rect.y + i as f32 * rect.h / 7.0;
        draw_line(
            rect.x + rect.w * 0.32,
            y,
            rect.x + rect.w * 0.68,
            y,
            1.5,
            Color::new(0.76, 0.88, 0.90, 0.24),
        );
    }
}

fn draw_lift_scheduler(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x + rect.w * 0.15,
        rect.y + rect.h * 0.18,
        rect.w * 0.70,
        rect.h * 0.62,
        Color::new(0.12, 0.16, 0.18, 0.84),
    );
    for i in 0..4 {
        draw_circle(
            rect.x + rect.w * (0.28 + i as f32 * 0.14),
            rect.y + rect.h * 0.48,
            4.0 * view.scale,
            Color::new(0.44, 0.92, 1.0, 0.34),
        );
    }
}

fn draw_wrong_floor_door(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.12, 0.08, 0.10, 0.74),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(1.0, 0.62, 0.30, 0.34),
    );
}

fn draw_elevator_security_unit(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.42,
        rect.h * 0.22,
        Color::new(0.50, 0.86, 1.0, 0.34),
    );
    draw_line(
        rect.x + rect.w * 0.25,
        rect.y + rect.h * 0.56,
        rect.x + rect.w * 0.75,
        rect.y + rect.h * 0.56,
        2.0,
        Color::new(0.50, 0.86, 1.0, 0.30),
    );
}

fn draw_empty_lift_window(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.80, 0.94, 1.0, 0.28),
    );
    draw_rectangle(
        rect.x + rect.w * 0.16,
        rect.y + rect.h * 0.16,
        rect.w * 0.68,
        rect.h * 0.68,
        Color::new(0.02, 0.03, 0.04, 0.54),
    );
}
