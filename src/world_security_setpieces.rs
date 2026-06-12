//! Rendering helpers for security checkpoint setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_security_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::QueueScannerCover => draw_queue_cover(view, world_rect),
        SetpieceKind::BadgeGate => draw_badge_gate(view, world_rect),
        SetpieceKind::SecurityOffice => draw_security_office(view, world_rect),
        SetpieceKind::MaintenanceBypass => draw_maintenance_bypass(view, world_rect),
        SetpieceKind::BadgePrinter => draw_badge_printer(view, world_rect),
        SetpieceKind::GateJam => draw_gate_jam(view, world_rect),
        SetpieceKind::WrongBadgeLoop => draw_wrong_badge_loop(view, world_rect),
        SetpieceKind::DroneChargingPad => draw_drone_charging_pad(view, world_rect),
        SetpieceKind::DroneRail => draw_drone_rail(view, world_rect),
        SetpieceKind::DispatchTower => draw_dispatch_tower(view, world_rect),
        SetpieceKind::DroneServicedHatch => draw_drone_serviced_hatch(view, world_rect),
        SetpieceKind::RescueDronePath => draw_rescue_drone_path(view, world_rect),
        SetpieceKind::EnthusiasticDroneCarry => draw_enthusiastic_drone_carry(view, world_rect),
        SetpieceKind::BrigCellDoor => draw_brig_cell_door(view, world_rect),
        SetpieceKind::PrisonerWalkway => draw_prisoner_walkway(view, world_rect),
        SetpieceKind::DoorControlRoom => draw_door_control_room(view, world_rect),
        SetpieceKind::EvidenceLock => draw_evidence_lock(view, world_rect),
        SetpieceKind::OneWayDoor => draw_one_way_door(view, world_rect),
        SetpieceKind::WrongWaitingRoom => draw_wrong_waiting_room(view, world_rect),
        SetpieceKind::RevolvingDoorLoop => draw_revolving_door_loop(view, world_rect),
        SetpieceKind::WeaponLockerCorridor => draw_weapon_locker_corridor(view, world_rect),
        SetpieceKind::ArmoryCatwalk => draw_armory_catwalk(view, world_rect),
        SetpieceKind::FoamPit => draw_foam_pit(view, world_rect),
        SetpieceKind::StunTurretLane => draw_stun_turret_lane(view, world_rect),
        SetpieceKind::FoamLauncher => draw_foam_launcher(view, world_rect),
        SetpieceKind::TargetingConsole => draw_targeting_console(view, world_rect),
        SetpieceKind::FoamPile => draw_foam_pile(view, world_rect),
        _ => {}
    }
}

fn draw_queue_cover(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..4 {
        let x = rect.x + i as f32 * rect.w / 4.0;
        draw_rectangle(
            x,
            rect.y + rect.h * 0.36,
            rect.w * 0.18,
            rect.h * 0.52,
            Color::new(0.16, 0.22, 0.25, 0.78),
        );
        draw_line(
            x,
            rect.y + rect.h * 0.34,
            x + rect.w * 0.18,
            rect.y + rect.h * 0.34,
            2.0,
            Color::new(0.48, 0.92, 1.0, 0.32),
        );
    }
}

fn draw_badge_gate(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.13, 0.15, 0.76),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.62, 0.94, 1.0, 0.34),
    );
    for i in 0..3 {
        let y = rect.y + rect.h * (0.22 + i as f32 * 0.24);
        draw_line(
            rect.x + rect.w * 0.20,
            y,
            rect_right(rect) - rect.w * 0.20,
            y,
            2.0,
            Color::new(0.62, 0.94, 1.0, 0.22),
        );
    }
}

fn draw_security_office(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.11, 0.13, 0.66),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.78, 0.90, 0.96, 0.24),
    );
    draw_rectangle(
        rect.x + rect.w * 0.10,
        rect.y + rect.h * 0.56,
        rect.w * 0.80,
        rect.h * 0.20,
        Color::new(0.18, 0.24, 0.27, 0.74),
    );
}

fn draw_maintenance_bypass(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.40,
        rect.w,
        rect.h * 0.42,
        Color::new(0.025, 0.035, 0.04, 0.76),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.25,
        rect.w,
        rect.h * 0.57,
        2.0,
        Color::new(0.60, 0.76, 0.82, 0.26),
    );
    for i in 0..7 {
        let x = rect.x + i as f32 * rect.w / 7.0;
        draw_line(
            x,
            rect.y + rect.h * 0.25,
            x + rect.w * 0.05,
            rect.y + rect.h * 0.82,
            1.0,
            Color::new(0.60, 0.76, 0.82, 0.18),
        );
    }
}

fn draw_badge_printer(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x + rect.w * 0.10,
        rect.y + rect.h * 0.20,
        rect.w * 0.80,
        rect.h * 0.62,
        Color::new(0.17, 0.22, 0.25, 0.88),
    );
    draw_rectangle_lines(
        rect.x + rect.w * 0.10,
        rect.y + rect.h * 0.20,
        rect.w * 0.80,
        rect.h * 0.62,
        1.5,
        Color::new(0.76, 0.96, 1.0, 0.34),
    );
    draw_rectangle(
        rect.x + rect.w * 0.28,
        rect.y + rect.h * 0.58,
        rect.w * 0.44,
        rect.h * 0.18,
        Color::new(0.92, 0.86, 0.44, 0.58),
    );
}

fn draw_gate_jam(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.10 + i as f32 * 0.17);
        draw_rectangle(
            x,
            rect.y + (i % 2) as f32 * rect.h * 0.18,
            rect.w * 0.10,
            rect.h * 0.62,
            Color::new(0.88, 0.38, 0.22, 0.46),
        );
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.12,
            rect_bottom(rect),
            2.0,
            Color::new(1.0, 0.72, 0.32, 0.32),
        );
    }
}

fn draw_wrong_badge_loop(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..4 {
        let x = rect.x + rect.w * (0.12 + i as f32 * 0.21);
        let y = rect.y + rect.h * (0.28 + (i % 2) as f32 * 0.22);
        draw_circle_lines(x, y, rect.h * 0.20, 2.0, Color::new(1.0, 0.76, 0.30, 0.36));
        draw_line(
            x - rect.w * 0.04,
            y,
            x + rect.w * 0.04,
            y,
            2.0,
            Color::new(1.0, 0.76, 0.30, 0.36),
        );
    }
}

fn draw_drone_charging_pad(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.42,
        rect.w,
        rect.h * 0.36,
        Color::new(0.10, 0.16, 0.18, 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.42,
        rect.w,
        rect.h * 0.36,
        1.5,
        Color::new(0.46, 0.90, 1.0, 0.34),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.60,
        rect.h * 0.14,
        Color::new(0.48, 0.92, 1.0, 0.34),
    );
}

fn draw_drone_rail(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for y in [rect.y + rect.h * 0.28, rect.y + rect.h * 0.62] {
        draw_line(
            rect.x,
            y,
            rect_right(rect),
            y,
            3.0,
            Color::new(0.62, 0.86, 0.94, 0.30),
        );
    }
    for i in 0..6 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.16);
        draw_circle(
            x,
            rect.y + rect.h * 0.45,
            8.0 * view.scale,
            Color::new(0.68, 0.94, 1.0, 0.40),
        );
    }
}

fn draw_dispatch_tower(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x + rect.w * 0.34,
        rect.y,
        rect.w * 0.32,
        rect.h,
        Color::new(0.12, 0.18, 0.20, 0.80),
    );
    draw_rectangle_lines(
        rect.x + rect.w * 0.22,
        rect.y,
        rect.w * 0.56,
        rect.h,
        2.0,
        Color::new(0.72, 0.92, 1.0, 0.28),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.18,
        rect.w * 0.16,
        Color::new(0.42, 0.90, 1.0, 0.38),
    );
}

fn draw_drone_serviced_hatch(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.12, 0.14, 0.78),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.62, 0.94, 1.0, 0.34),
    );
    draw_line(
        rect.x,
        rect.y,
        rect_right(rect),
        rect_bottom(rect),
        2.0,
        Color::new(0.62, 0.94, 1.0, 0.20),
    );
    draw_line(
        rect_right(rect),
        rect.y,
        rect.x,
        rect_bottom(rect),
        2.0,
        Color::new(0.62, 0.94, 1.0, 0.20),
    );
}

fn draw_rescue_drone_path(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.18);
        let y = rect.y + rect.h * (0.34 + (i % 2) as f32 * 0.22);
        draw_circle(x, y, 9.0 * view.scale, Color::new(0.50, 0.94, 1.0, 0.44));
        draw_line(
            x - 16.0 * view.scale,
            y,
            x + 16.0 * view.scale,
            y,
            2.0,
            Color::new(0.80, 0.98, 1.0, 0.34),
        );
    }
}

fn draw_enthusiastic_drone_carry(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..4 {
        let x = rect.x + rect.w * (0.12 + i as f32 * 0.22);
        let y = rect.y + rect.h * (0.30 + (i % 2) as f32 * 0.26);
        draw_circle(x, y, 10.0 * view.scale, Color::new(1.0, 0.72, 0.30, 0.38));
        draw_line(
            x,
            y + 9.0 * view.scale,
            x,
            y + 32.0 * view.scale,
            2.0,
            Color::new(1.0, 0.86, 0.48, 0.32),
        );
        draw_rectangle(
            x - 12.0 * view.scale,
            y + 28.0 * view.scale,
            24.0 * view.scale,
            12.0 * view.scale,
            Color::new(0.84, 0.90, 0.92, 0.30),
        );
    }
}

fn draw_brig_cell_door(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.06, 0.09, 0.10, 0.78),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.72, 0.88, 0.92, 0.28),
    );
    for i in 0..4 {
        let x = rect.x + rect.w * (0.20 + i as f32 * 0.16);
        draw_line(
            x,
            rect.y + 6.0 * view.scale,
            x,
            rect_bottom(rect) - 6.0 * view.scale,
            1.5,
            Color::new(0.72, 0.88, 0.92, 0.24),
        );
    }
}

fn draw_prisoner_walkway(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.52,
        rect_right(rect),
        rect.y + rect.h * 0.52,
        6.0,
        Color::new(0.38, 0.48, 0.50, 0.72),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.18,
        rect.w,
        rect.h * 0.68,
        2.0,
        Color::new(0.62, 0.76, 0.78, 0.24),
    );
}

fn draw_door_control_room(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.09, 0.13, 0.14, 0.70),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.62, 0.94, 1.0, 0.28),
    );
    draw_rectangle(
        rect.x + rect.w * 0.22,
        rect.y + rect.h * 0.56,
        rect.w * 0.56,
        rect.h * 0.18,
        Color::new(0.42, 0.84, 1.0, 0.24),
    );
}

fn draw_evidence_lock(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.12, 0.14, 0.18, 0.80),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.96, 0.84, 0.38, 0.34),
    );
    draw_circle(
        rect.x + rect.w * 0.5,
        rect.y + rect.h * 0.45,
        rect.w * 0.18,
        Color::new(0.96, 0.84, 0.38, 0.28),
    );
}

fn draw_one_way_door(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.70, 0.92, 1.0, 0.30),
    );
    draw_triangle(
        vec2(rect.x + rect.w * 0.68, rect.y + rect.h * 0.50),
        vec2(rect.x + rect.w * 0.42, rect.y + rect.h * 0.30),
        vec2(rect.x + rect.w * 0.42, rect.y + rect.h * 0.70),
        Color::new(0.70, 0.92, 1.0, 0.30),
    );
}

fn draw_wrong_waiting_room(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.10, 0.12, 0.13, 0.56),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(1.0, 0.72, 0.32, 0.28),
    );
    for i in 0..3 {
        let x = rect.x + rect.w * (0.18 + i as f32 * 0.28);
        draw_circle(
            x,
            rect.y + rect.h * 0.56,
            8.0 * view.scale,
            Color::new(1.0, 0.72, 0.32, 0.26),
        );
    }
}

fn draw_revolving_door_loop(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    draw_circle_lines(
        center.x,
        center.y,
        rect.h * 0.36,
        2.0,
        Color::new(1.0, 0.62, 0.30, 0.34),
    );
    draw_line(
        center.x - rect.w * 0.22,
        center.y,
        center.x + rect.w * 0.22,
        center.y,
        2.0,
        Color::new(1.0, 0.62, 0.30, 0.30),
    );
    draw_line(
        center.x,
        center.y - rect.h * 0.28,
        center.x,
        center.y + rect.h * 0.28,
        2.0,
        Color::new(1.0, 0.62, 0.30, 0.30),
    );
}

fn draw_weapon_locker_corridor(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.09, 0.10, 0.11, 0.62),
    );
    for i in 0..7 {
        let x = rect.x + rect.w * (0.04 + i as f32 * 0.13);
        draw_rectangle_lines(
            x,
            rect.y + rect.h * 0.16,
            rect.w * 0.08,
            rect.h * 0.62,
            1.5,
            Color::new(0.72, 0.78, 0.80, 0.24),
        );
    }
}

fn draw_armory_catwalk(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.48,
        rect_right(rect),
        rect.y + rect.h * 0.48,
        6.0,
        Color::new(0.42, 0.48, 0.50, 0.72),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y + rect.h * 0.18,
        rect.w,
        rect.h * 0.62,
        2.0,
        Color::new(0.78, 0.86, 0.88, 0.24),
    );
}

fn draw_foam_pit(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.38,
        rect.w,
        rect.h * 0.46,
        Color::new(0.76, 0.92, 1.0, 0.18),
    );
    for i in 0..8 {
        let x = rect.x + rect.w * (0.06 + i as f32 * 0.11);
        let y = rect.y + rect.h * (0.54 + (i % 2) as f32 * 0.14);
        draw_circle(x, y, 12.0 * view.scale, Color::new(0.88, 0.98, 1.0, 0.28));
    }
}

fn draw_stun_turret_lane(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.18, 0.08, 0.08, 0.18),
    );
    for i in 0..4 {
        let x = rect.x + rect.w * (0.10 + i as f32 * 0.24);
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.18,
            rect_bottom(rect),
            2.0,
            Color::new(1.0, 0.28, 0.20, 0.22),
        );
    }
}

fn draw_foam_launcher(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x + rect.w * 0.18,
        rect.y + rect.h * 0.25,
        rect.w * 0.64,
        rect.h * 0.36,
        Color::new(0.18, 0.22, 0.24, 0.82),
    );
    draw_circle(
        rect.x + rect.w * 0.78,
        rect.y + rect.h * 0.42,
        rect.h * 0.18,
        Color::new(0.82, 0.96, 1.0, 0.34),
    );
}

fn draw_targeting_console(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x + rect.w * 0.18,
        rect.y + rect.h * 0.24,
        rect.w * 0.64,
        rect.h * 0.52,
        Color::new(0.12, 0.15, 0.16, 0.86),
    );
    draw_rectangle_lines(
        rect.x + rect.w * 0.18,
        rect.y + rect.h * 0.24,
        rect.w * 0.64,
        rect.h * 0.52,
        1.5,
        Color::new(1.0, 0.56, 0.36, 0.34),
    );
}

fn draw_foam_pile(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..7 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.12);
        let y = rect.y + rect.h * (0.36 + (i % 3) as f32 * 0.14);
        draw_circle(x, y, 16.0 * view.scale, Color::new(0.86, 0.98, 1.0, 0.34));
        draw_circle_lines(
            x,
            y,
            16.0 * view.scale,
            1.2,
            Color::new(0.96, 1.0, 1.0, 0.32),
        );
    }
}
