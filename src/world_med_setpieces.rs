//! Rendering helpers for Med-Bay setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_med_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::ReceptionDesk => draw_reception_desk(view, world_rect),
        SetpieceKind::TreatmentRoom => draw_treatment_room(view, world_rect),
        SetpieceKind::AutoDoc => draw_auto_doc(view, world_rect),
        SetpieceKind::BedLift => draw_bed_lift(view, world_rect),
        SetpieceKind::RecoveryHallway => draw_recovery_hallway(view, world_rect),
        SetpieceKind::TriageDoor => draw_triage_door(view, world_rect),
        SetpieceKind::RollingBed => draw_rolling_bed(view, world_rect),
        SetpieceKind::BandageCocoon => draw_bandage_cocoon(view, world_rect),
        SetpieceKind::MedicDrone => draw_medic_drone(view, world_rect),
        _ => {}
    }
}

fn draw_reception_desk(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.18, 0.27, 0.30, 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.72, 1.0, 0.92, 0.32),
    );
    draw_rectangle(
        rect.x + rect.w * 0.08,
        rect.y + rect.h * 0.18,
        rect.w * 0.26,
        rect.h * 0.16,
        Color::new(0.74, 1.0, 0.92, 0.28),
    );
}

fn draw_treatment_room(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.18, 0.17, 0.34),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.72, 1.0, 0.92, 0.32),
    );
    for i in 0..3 {
        let x = rect.x + rect.w * (0.22 + i as f32 * 0.25);
        draw_line(
            x,
            rect.y,
            x,
            rect_bottom(rect),
            1.5,
            Color::new(0.72, 1.0, 0.92, 0.18),
        );
    }
}

fn draw_auto_doc(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.22, 0.30, 0.32, 0.76),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.62, 1.0, 0.88, 0.46),
    );
    let c = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.42);
    draw_rectangle(
        c.x - 4.0 * view.scale,
        c.y - 20.0 * view.scale,
        8.0 * view.scale,
        40.0 * view.scale,
        Color::new(0.78, 1.0, 0.92, 0.52),
    );
    draw_rectangle(
        c.x - 20.0 * view.scale,
        c.y - 4.0 * view.scale,
        40.0 * view.scale,
        8.0 * view.scale,
        Color::new(0.78, 1.0, 0.92, 0.52),
    );
}

fn draw_bed_lift(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.64, 0.86, 0.90, 0.34),
    );
    draw_line(
        rect.x,
        rect.y,
        rect_right(rect),
        rect_bottom(rect),
        1.5,
        Color::new(0.64, 0.86, 0.90, 0.24),
    );
    draw_line(
        rect_right(rect),
        rect.y,
        rect.x,
        rect_bottom(rect),
        1.5,
        Color::new(0.64, 0.86, 0.90, 0.24),
    );
}

fn draw_recovery_hallway(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.10, 0.20, 0.21, 0.50),
    );
    draw_line(
        rect.x,
        rect.y,
        rect_right(rect),
        rect.y,
        3.0,
        Color::new(0.74, 1.0, 0.92, 0.24),
    );
}

fn draw_triage_door(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.11, 0.18, 0.18, 0.72),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.64, 1.0, 0.88, 0.48),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.28,
        6.0 * view.scale,
        Color::new(0.64, 1.0, 0.88, 0.52),
    );
}

fn draw_rolling_bed(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.65,
        Color::new(0.80, 0.90, 0.88, 0.72),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.65,
        1.5,
        Color::new(0.98, 1.0, 1.0, 0.32),
    );
    for x in [rect.x + rect.w * 0.22, rect.x + rect.w * 0.78] {
        draw_circle(
            x,
            rect.y + rect.h * 0.82,
            5.0 * view.scale,
            Color::new(0.10, 0.12, 0.13, 0.9),
        );
    }
}

fn draw_bandage_cocoon(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_ellipse(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.52,
        rect.w * 0.45,
        rect.h * 0.34,
        0.0,
        Color::new(0.92, 0.90, 0.82, 0.74),
    );
    for i in 0..4 {
        let y = rect.y + rect.h * (0.28 + i as f32 * 0.12);
        draw_line(
            rect.x + rect.w * 0.18,
            y,
            rect.x + rect.w * 0.82,
            y + 6.0 * view.scale,
            1.5,
            Color::new(0.72, 0.70, 0.62, 0.45),
        );
    }
}

fn draw_medic_drone(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.5,
        rect.h * 0.42,
        Color::new(0.58, 1.0, 0.88, 0.34),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.5,
        rect_right(rect),
        rect.y + rect.h * 0.5,
        2.0,
        Color::new(0.82, 1.0, 0.92, 0.42),
    );
}
