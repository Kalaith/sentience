//! Rendering helpers for early campaign setpieces.

use crate::geometry::{rect_bottom, rect_right};
use crate::state::SetpieceKind;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_early_setpiece(view: &WorldView, kind: SetpieceKind, world_rect: Rect) {
    match kind {
        SetpieceKind::ScrapHeap => draw_scrap_heap(view, world_rect),
        SetpieceKind::BrokenCatwalk => draw_broken_catwalk(view, world_rect),
        SetpieceKind::CrouchTunnel => draw_crouch_tunnel(view, world_rect),
        SetpieceKind::HangingScrap => draw_hanging_scrap(view, world_rect),
        SetpieceKind::DroppedScrapBridge => draw_scrap_bridge(view, world_rect),
        SetpieceKind::FloatingScrapCover => draw_floating_scrap(view, world_rect),
        SetpieceKind::ExitLedge => draw_exit_ledge(view, world_rect),
        SetpieceKind::CargoBelt => draw_cargo_belt(view, world_rect),
        SetpieceKind::CargoLift => draw_cargo_lift(view, world_rect),
        SetpieceKind::ScannerGate => draw_scanner_gate(view, world_rect),
        SetpieceKind::DroneLane => draw_drone_lane(view, world_rect),
        SetpieceKind::SupplyPalletBridge => draw_supply_pallets(view, world_rect),
        SetpieceKind::MagnetizedCrateTrail => draw_magnetized_trail(view, world_rect),
        SetpieceKind::LockerBank => draw_locker_bank(view, world_rect),
        SetpieceKind::DeconShower => draw_decon_shower(view, world_rect),
        SetpieceKind::SprayLift => draw_spray_lift(view, world_rect),
        SetpieceKind::SuitRack => draw_suit_rack(view, world_rect),
        SetpieceKind::MaintenanceCrawlspace => draw_maintenance_crawlspace(view, world_rect),
        SetpieceKind::FoamBouncePad => draw_foam_bounce_pad(view, world_rect),
        SetpieceKind::DiningTable => draw_dining_table(view, world_rect),
        SetpieceKind::ServingRail => draw_serving_rail(view, world_rect),
        SetpieceKind::MealCart => draw_meal_cart(view, world_rect),
        SetpieceKind::KitchenPass => draw_kitchen_pass(view, world_rect),
        SetpieceKind::DiningPit => draw_dining_pit(view, world_rect),
        SetpieceKind::DishReturnRamp => draw_dish_return_ramp(view, world_rect),
        SetpieceKind::SlipperyGel => draw_slippery_gel(view, world_rect),
        SetpieceKind::EvacuationFlow => draw_evacuation_flow(view, world_rect),
        _ => {}
    }
}

fn draw_scrap_heap(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.46,
        rect.w,
        rect.h * 0.54,
        Color::new(0.20, 0.22, 0.24, 0.92),
    );
    for i in 0..5 {
        let x = rect.x + i as f32 * rect.w * 0.18;
        let y = rect_bottom(rect) - i as f32 * rect.h * 0.04;
        draw_triangle(
            vec2(x, y),
            vec2(
                x + rect.w * 0.24,
                rect.y + rect.h * (0.18 + i as f32 * 0.05),
            ),
            vec2(x + rect.w * 0.42, y),
            Color::new(0.34, 0.37, 0.39, 0.86),
        );
    }
}

fn draw_broken_catwalk(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.25,
        rect.x + rect.w * 0.38,
        rect.y + rect.h * 0.25,
        4.0,
        Color::new(0.54, 0.60, 0.63, 0.62),
    );
    draw_line(
        rect.x + rect.w * 0.58,
        rect.y + rect.h * 0.62,
        rect_right(rect),
        rect.y + rect.h * 0.62,
        4.0,
        Color::new(0.54, 0.60, 0.63, 0.62),
    );
    for i in 0..5 {
        let x = rect.x + 14.0 * view.scale + i as f32 * rect.w * 0.17;
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.06,
            rect_bottom(rect),
            2.0,
            Color::new(0.78, 0.88, 0.92, 0.38),
        );
    }
}

fn draw_crouch_tunnel(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.03, 0.04, 0.045, 0.58),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.52, 0.62, 0.66, 0.45),
    );
    draw_line(
        rect.x + 10.0 * view.scale,
        rect.y,
        rect_right(rect) - 10.0 * view.scale,
        rect.y,
        3.0,
        Color::new(0.82, 0.90, 0.92, 0.25),
    );
}

fn draw_hanging_scrap(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    let anchor = vec2(rect.x + rect.w * 0.5, rect.y - 42.0 * view.scale);
    draw_line(
        anchor.x,
        anchor.y,
        rect.x + rect.w * 0.5,
        rect.y,
        2.0,
        Color::new(0.75, 0.86, 0.9, 0.5),
    );
    draw_rectangle(
        rect.x + rect.w * 0.18,
        rect.y + rect.h * 0.22,
        rect.w * 0.64,
        rect.h * 0.42,
        Color::new(0.36, 0.39, 0.40, 0.84),
    );
    draw_circle(
        rect.x + rect.w * 0.24,
        rect.y + rect.h * 0.56,
        rect.h * 0.20,
        Color::new(0.22, 0.25, 0.27, 0.84),
    );
    draw_circle(
        rect.x + rect.w * 0.72,
        rect.y + rect.h * 0.36,
        rect.h * 0.18,
        Color::new(0.42, 0.45, 0.47, 0.82),
    );
}

fn draw_scrap_bridge(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.36, 0.40, 0.42, 0.96),
    );
    for i in 0..6 {
        let x = rect.x + i as f32 * rect.w / 6.0;
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.08,
            rect_bottom(rect),
            1.5,
            Color::new(0.84, 0.92, 0.95, 0.25),
        );
    }
}

fn draw_floating_scrap(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..7 {
        let x = rect.x + rect.w * (0.08 + i as f32 * 0.13);
        let y = rect.y + rect.h * (0.25 + (i % 3) as f32 * 0.18);
        draw_rectangle(
            x,
            y,
            rect.w * 0.10,
            rect.h * 0.14,
            Color::new(0.42, 0.46, 0.48, 0.58),
        );
        draw_circle(
            x + rect.w * 0.05,
            y + rect.h * 0.07,
            rect.h * 0.10,
            Color::new(0.70, 0.85, 0.92, 0.22),
        );
    }
}

fn draw_exit_ledge(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.35,
        rect.w,
        rect.h * 0.65,
        Color::new(0.18, 0.28, 0.30, 0.78),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.35,
        rect_right(rect),
        rect.y + rect.h * 0.35,
        3.0,
        Color::new(0.52, 0.86, 0.82, 0.44),
    );
}

fn draw_cargo_belt(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.12, 0.14, 0.15, 0.88),
    );
    for i in 0..12 {
        let x = rect.x + i as f32 * rect.w / 12.0;
        draw_circle(
            x + rect.w / 24.0,
            rect.y + rect.h * 0.5,
            rect.h * 0.22,
            Color::new(0.50, 0.56, 0.58, 0.55),
        );
    }
    draw_line(
        rect.x,
        rect.y,
        rect_right(rect),
        rect.y,
        2.0,
        Color::new(0.72, 0.82, 0.84, 0.30),
    );
}

fn draw_cargo_lift(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.58, 0.72, 0.76, 0.42),
    );
    draw_line(
        rect.x,
        rect.y,
        rect_right(rect),
        rect_bottom(rect),
        2.0,
        Color::new(0.58, 0.72, 0.76, 0.28),
    );
    draw_line(
        rect_right(rect),
        rect.y,
        rect.x,
        rect_bottom(rect),
        2.0,
        Color::new(0.58, 0.72, 0.76, 0.28),
    );
}

fn draw_scanner_gate(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.10, 0.18, 0.20, 0.72),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.36, 0.94, 1.0, 0.55),
    );
    let beam_x = rect.x + rect.w * 0.5;
    draw_line(
        beam_x,
        rect.y + 12.0 * view.scale,
        beam_x,
        rect_bottom(rect) - 12.0 * view.scale,
        3.0,
        Color::new(0.36, 0.94, 1.0, 0.35),
    );
}

fn draw_drone_lane(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect.y + rect.h * 0.5,
        rect_right(rect),
        rect.y + rect.h * 0.5,
        3.0,
        Color::new(0.68, 0.84, 0.88, 0.32),
    );
    for i in 0..4 {
        let x = rect.x + rect.w * (0.15 + i as f32 * 0.22);
        let y = rect.y + rect.h * (0.35 + (i % 2) as f32 * 0.3);
        draw_circle(x, y, 9.0 * view.scale, Color::new(0.58, 0.92, 1.0, 0.50));
        draw_line(
            x - 15.0 * view.scale,
            y,
            x + 15.0 * view.scale,
            y,
            2.0,
            Color::new(0.82, 0.96, 1.0, 0.45),
        );
    }
}

fn draw_supply_pallets(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let pallet = Rect::new(
            rect.x + i as f32 * rect.w / 5.0,
            rect.y,
            rect.w / 5.0 - 4.0 * view.scale,
            rect.h,
        );
        draw_rectangle(
            pallet.x,
            pallet.y,
            pallet.w,
            pallet.h,
            Color::new(0.46, 0.34, 0.20, 0.62),
        );
        draw_rectangle_lines(
            pallet.x,
            pallet.y,
            pallet.w,
            pallet.h,
            1.5,
            Color::new(0.88, 0.72, 0.42, 0.36),
        );
    }
}

fn draw_magnetized_trail(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..7 {
        let x = rect.x + i as f32 * rect.w / 7.0;
        let y = rect.y + (i % 2) as f32 * rect.h * 0.22;
        draw_rectangle(
            x,
            y,
            rect.w * 0.08,
            rect.h * 0.72,
            Color::new(0.42, 0.32, 0.22, 0.62),
        );
        draw_circle_lines(
            x + rect.w * 0.04,
            y - 5.0 * view.scale,
            13.0 * view.scale,
            2.0,
            Color::new(1.0, 0.34, 0.20, 0.42),
        );
    }
}

fn draw_locker_bank(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let locker = Rect::new(
            rect.x + i as f32 * rect.w / 5.0,
            rect.y,
            rect.w / 5.0 - 3.0 * view.scale,
            rect.h,
        );
        draw_rectangle(
            locker.x,
            locker.y,
            locker.w,
            locker.h,
            Color::new(0.16, 0.23, 0.27, 0.72),
        );
        draw_rectangle_lines(
            locker.x,
            locker.y,
            locker.w,
            locker.h,
            1.5,
            Color::new(0.58, 0.86, 0.92, 0.32),
        );
        draw_circle(
            locker.x + locker.w * 0.78,
            locker.y + locker.h * 0.5,
            2.5 * view.scale,
            Color::new(0.88, 0.96, 1.0, 0.48),
        );
    }
}

fn draw_decon_shower(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.16, 0.18, 0.42),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.52, 0.92, 1.0, 0.36),
    );
    for i in 0..5 {
        let x = rect.x + rect.w * (0.15 + i as f32 * 0.18);
        draw_line(
            x,
            rect.y + 10.0 * view.scale,
            x - rect.w * 0.08,
            rect_bottom(rect) - 12.0 * view.scale,
            1.5,
            Color::new(0.66, 0.96, 1.0, 0.22),
        );
    }
}

fn draw_spray_lift(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..4 {
        let y = rect_bottom(rect) - i as f32 * rect.h * 0.24;
        draw_triangle(
            vec2(rect.x + rect.w * 0.5, y - 22.0 * view.scale),
            vec2(rect.x + rect.w * 0.18, y),
            vec2(rect.x + rect.w * 0.82, y),
            Color::new(0.55, 0.94, 1.0, 0.18),
        );
    }
}

fn draw_suit_rack(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.74, 0.88, 0.92, 0.34),
    );
    for i in 0..3 {
        let x = rect.x + rect.w * (0.22 + i as f32 * 0.26);
        draw_line(
            x,
            rect.y,
            x,
            rect_bottom(rect),
            1.5,
            Color::new(0.74, 0.88, 0.92, 0.24),
        );
        draw_circle(
            x,
            rect.y + rect.h * 0.28,
            11.0 * view.scale,
            Color::new(0.64, 0.74, 0.78, 0.42),
        );
        draw_rectangle(
            x - 10.0 * view.scale,
            rect.y + rect.h * 0.38,
            20.0 * view.scale,
            rect.h * 0.36,
            Color::new(0.64, 0.74, 0.78, 0.34),
        );
    }
}

fn draw_maintenance_crawlspace(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.02, 0.025, 0.03, 0.72),
    );
    for i in 0..6 {
        let x = rect.x + i as f32 * rect.w / 6.0;
        draw_line(
            x,
            rect.y,
            x + rect.w * 0.05,
            rect_bottom(rect),
            1.0,
            Color::new(0.60, 0.72, 0.76, 0.24),
        );
    }
}

fn draw_foam_bounce_pad(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..5 {
        let x = rect.x + rect.w * (0.10 + i as f32 * 0.18);
        let y = rect.y + rect.h * (0.35 + (i % 2) as f32 * 0.18);
        draw_circle(x, y, rect.h * 0.34, Color::new(0.86, 0.96, 1.0, 0.30));
        draw_circle_lines(x, y, rect.h * 0.34, 1.5, Color::new(0.96, 1.0, 1.0, 0.42));
    }
}

fn draw_dining_table(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.55,
        Color::new(0.34, 0.24, 0.17, 0.88),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.55,
        2.0,
        Color::new(0.82, 0.62, 0.40, 0.28),
    );
    for x in [rect.x + rect.w * 0.16, rect.x + rect.w * 0.84] {
        draw_line(
            x,
            rect.y + rect.h * 0.55,
            x,
            rect_bottom(rect),
            4.0,
            Color::new(0.24, 0.18, 0.13, 0.8),
        );
    }
}

fn draw_serving_rail(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    let y = rect.y + rect.h * 0.34;
    draw_line(
        rect.x,
        y,
        rect_right(rect),
        y,
        5.0,
        Color::new(0.72, 0.82, 0.84, 0.36),
    );
    draw_line(
        rect.x,
        y + 12.0 * view.scale,
        rect_right(rect),
        y + 12.0 * view.scale,
        2.0,
        Color::new(0.72, 0.82, 0.84, 0.24),
    );
    for i in 0..4 {
        let x = rect.x + rect.w * (0.16 + i as f32 * 0.22);
        draw_line(
            x,
            y,
            x,
            y + 28.0 * view.scale,
            1.5,
            Color::new(0.72, 0.82, 0.84, 0.34),
        );
    }
}

fn draw_meal_cart(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.72,
        Color::new(0.52, 0.56, 0.54, 0.82),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h * 0.72,
        1.5,
        Color::new(0.92, 0.98, 0.92, 0.32),
    );
    for x in [rect.x + rect.w * 0.24, rect.x + rect.w * 0.76] {
        draw_circle(
            x,
            rect.y + rect.h * 0.82,
            6.0 * view.scale,
            Color::new(0.12, 0.12, 0.13, 0.92),
        );
    }
}

fn draw_kitchen_pass(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.10, 0.08, 0.07, 0.70),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.86, 0.58, 0.38, 0.30),
    );
    draw_rectangle(
        rect.x + rect.w * 0.12,
        rect.y + rect.h * 0.42,
        rect.w * 0.76,
        rect.h * 0.18,
        Color::new(0.92, 0.82, 0.54, 0.22),
    );
}

fn draw_dining_pit(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.32,
        rect.w,
        rect.h * 0.68,
        Color::new(0.02, 0.018, 0.016, 0.76),
    );
    draw_line(
        rect.x,
        rect.y + rect.h * 0.32,
        rect_right(rect),
        rect.y + rect.h * 0.32,
        3.0,
        Color::new(0.86, 0.62, 0.34, 0.26),
    );
}

fn draw_dish_return_ramp(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    draw_line(
        rect.x,
        rect_bottom(rect),
        rect_right(rect),
        rect.y,
        8.0,
        Color::new(0.42, 0.48, 0.50, 0.76),
    );
    draw_line(
        rect.x,
        rect_bottom(rect) - 12.0 * view.scale,
        rect_right(rect),
        rect.y - 12.0 * view.scale,
        2.0,
        Color::new(0.82, 0.92, 0.95, 0.32),
    );
}

fn draw_slippery_gel(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..9 {
        let x = rect.x + i as f32 * rect.w / 9.0;
        let y = rect.y + (i % 3) as f32 * rect.h * 0.24;
        draw_ellipse(
            x + rect.w * 0.04,
            y + rect.h * 0.42,
            rect.w * 0.05,
            rect.h * 0.30,
            0.0,
            Color::new(1.0, 0.72, 0.32, 0.24),
        );
    }
}

fn draw_evacuation_flow(view: &WorldView, world_rect: Rect) {
    let rect = view.rect(world_rect);
    for i in 0..4 {
        let x = rect.x + i as f32 * rect.w * 0.22;
        let y = rect.y + rect.h * (0.25 + (i % 2) as f32 * 0.24);
        draw_line(
            x,
            y,
            x + rect.w * 0.13,
            y,
            3.0,
            Color::new(0.52, 0.96, 0.86, 0.36),
        );
        draw_triangle(
            vec2(x + rect.w * 0.15, y),
            vec2(x + rect.w * 0.10, y - 8.0 * view.scale),
            vec2(x + rect.w * 0.10, y + 8.0 * view.scale),
            Color::new(0.52, 0.96, 0.86, 0.36),
        );
    }
}
