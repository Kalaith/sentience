//! Level-specific background art and mechanic decals.

use crate::geometry::FLOOR_Y;
use crate::render_textures::draw_texture_fill;
use crate::state::{LevelPhase, MoralChoice};
use crate::ui::UiContext;
use crate::world_render::WorldView;
use macroquad::prelude::*;

pub(crate) fn draw_map_art(ctx: &UiContext<'_>, view: &WorldView) {
    let level = &ctx.data.levels[ctx.session.level_index];
    let key = format!("map_{}", level.id);
    let wall = view.rect(Rect::new(view.visible_left() + 54.0, 152.0, 1012.0, 224.0));
    let tint = match ctx.session.runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => Color::new(0.88, 1.0, 1.0, 0.82),
        LevelPhase::Resolved(MoralChoice::Villain) => Color::new(1.0, 0.76, 0.68, 0.82),
        LevelPhase::AwaitingChoice => Color::new(0.82, 0.88, 0.90, 0.74),
        LevelPhase::Final => Color::new(0.90, 0.95, 1.0, 0.86),
    };
    if !draw_texture_fill(ctx, &key, wall, tint) {
        draw_fallback_panel(wall, ctx.session.level_index);
    }

    draw_route_decals(ctx, view);
}

fn draw_fallback_panel(rect: Rect, level_index: usize) {
    let hue = (level_index as f32 * 0.071) % 1.0;
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08 + hue * 0.12, 0.11, 0.14 + hue * 0.16, 0.55),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.55, 0.75, 0.82, 0.35),
    );
}

fn draw_route_decals(ctx: &UiContext<'_>, view: &WorldView) {
    match ctx.session.runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => draw_helpful_decals(ctx, view),
        LevelPhase::Resolved(MoralChoice::Villain) => draw_gremlin_decals(ctx, view),
        LevelPhase::AwaitingChoice => draw_pending_decals(ctx, view),
        LevelPhase::Final => draw_final_decals(ctx, view),
    }
}

fn draw_pending_decals(ctx: &UiContext<'_>, view: &WorldView) {
    if matches!(ctx.session.level_index, 2 | 5 | 8 | 11 | 17) {
        draw_smoke_band(view, 0.11);
    }
}

fn draw_helpful_decals(ctx: &UiContext<'_>, view: &WorldView) {
    match ctx.session.level_index {
        2 => draw_suit_stations(view, Color::new(0.62, 0.96, 1.0, 0.64)),
        3 => draw_cleaning_lanes(view, Color::new(0.62, 0.95, 1.0, 0.48)),
        4 => draw_med_crosses(view, Color::new(0.74, 1.0, 0.92, 0.58)),
        5 => draw_airflow_columns(view, Color::new(0.55, 0.95, 1.0, 0.32)),
        6 => draw_vines(view, Color::new(0.44, 0.92, 0.48, 0.48)),
        7 => draw_pressure_doors(view, Color::new(0.68, 0.92, 1.0, 0.55)),
        8 => draw_cryo_pods(view, Color::new(0.68, 0.92, 1.0, 0.52)),
        9 => draw_scanner_gates(view, Color::new(0.50, 0.96, 1.0, 0.58)),
        10 => draw_drone_dots(view, Color::new(0.48, 0.92, 1.0, 0.68)),
        11 => draw_cell_doors(view, Color::new(0.70, 0.84, 0.92, 0.54)),
        12 => draw_light_beams(view, Color::new(1.0, 0.94, 0.58, 0.18)),
        13 => draw_targeting_marks(view, Color::new(0.62, 0.95, 1.0, 0.46)),
        14 => draw_laundry_tubes(view, Color::new(0.80, 0.95, 1.0, 0.40)),
        15 => draw_command_panels(view, Color::new(0.65, 0.92, 1.0, 0.50)),
        16 => draw_reactor_pipes(view, Color::new(0.68, 1.0, 0.90, 0.46)),
        17 => draw_lift_rails(view, Color::new(0.74, 0.90, 1.0, 0.50)),
        18 => draw_firewall_nodes(view, Color::new(0.52, 0.96, 1.0, 0.56)),
        _ => {}
    }
}

fn draw_gremlin_decals(ctx: &UiContext<'_>, view: &WorldView) {
    match ctx.session.level_index {
        2 => {
            draw_suit_stations(view, Color::new(1.0, 0.62, 0.48, 0.48));
            draw_foam_bubbles(view, 7, 0.18);
        }
        3 => draw_slip_arrows(view, Color::new(1.0, 0.62, 0.28, 0.44)),
        4 => draw_foam_bubbles(view, 6, 0.18),
        5 => {
            draw_airflow_columns(view, Color::new(0.95, 0.55, 1.0, 0.22));
            draw_smoke_band(view, 0.13);
        }
        6 => draw_overgrown_vines(view),
        7 => draw_pressure_bursts(view),
        8 => draw_blanket_piles(view),
        9 => draw_wrong_badge_marks(view),
        10 => draw_drone_dots(view, Color::new(1.0, 0.56, 0.42, 0.72)),
        11 => draw_revolving_arrows(view),
        12 => draw_shadow_lanes(view),
        13 => draw_foam_bubbles(view, 10, 0.22),
        14 => draw_laundry_tubes(view, Color::new(1.0, 0.68, 0.50, 0.46)),
        15 => draw_command_panels(view, Color::new(1.0, 0.58, 0.42, 0.50)),
        16 => draw_foam_bubbles(view, 9, 0.20),
        17 => draw_lift_confusion(view),
        18 => {
            draw_firewall_nodes(view, Color::new(1.0, 0.42, 0.32, 0.55));
            draw_shadow_lanes(view);
        }
        _ => {}
    }
}

fn draw_final_decals(ctx: &UiContext<'_>, view: &WorldView) {
    if ctx.session.runtime.boss.is_some() {
        draw_firewall_nodes(view, Color::new(0.55, 0.94, 1.0, 0.42));
    }
}

fn draw_suit_stations(view: &WorldView, color: Color) {
    for i in 0..4 {
        let rect = view.rect(Rect::new(148.0 + i as f32 * 178.0, 220.0, 64.0, 102.0));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, color);
        draw_circle(rect.x + rect.w * 0.5, rect.y + 28.0, 12.0, color);
    }
}

fn draw_cleaning_lanes(view: &WorldView, color: Color) {
    for i in 0..5 {
        let y = FLOOR_Y - 26.0 - i as f32 * 16.0;
        let a = view.point(vec2(120.0, y));
        let b = view.point(vec2(view.visible_right() - 120.0, y - 18.0));
        draw_line(a.x, a.y, b.x, b.y, 2.0, color);
    }
}

fn draw_med_crosses(view: &WorldView, color: Color) {
    for x in [206.0, 476.0, 742.0, 944.0] {
        let c = view.point(vec2(x, 244.0));
        draw_rectangle(c.x - 4.0, c.y - 18.0, 8.0, 36.0, color);
        draw_rectangle(c.x - 18.0, c.y - 4.0, 36.0, 8.0, color);
    }
}

fn draw_airflow_columns(view: &WorldView, color: Color) {
    for x in [260.0, 520.0, 780.0] {
        let rect = view.rect(Rect::new(x, 210.0, 70.0, 245.0));
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_triangle(
            vec2(rect.x + rect.w * 0.5, rect.y - 16.0),
            vec2(rect.x + 10.0, rect.y + 20.0),
            vec2(rect.right() - 10.0, rect.y + 20.0),
            Color::new(color.r, color.g, color.b, color.a + 0.12),
        );
    }
}

fn draw_vines(view: &WorldView, color: Color) {
    for i in 0..6 {
        let x = 150.0 + i as f32 * 150.0;
        let a = view.point(vec2(x, 180.0));
        let b = view.point(vec2(x + 40.0, 430.0));
        draw_line(a.x, a.y, b.x, b.y, 3.0, color);
        let leaf = view.point(vec2(x + 18.0, 292.0));
        draw_circle(leaf.x, leaf.y, 9.0, color);
    }
}

fn draw_pressure_doors(view: &WorldView, color: Color) {
    for x in [160.0, 480.0, 800.0] {
        let rect = view.rect(Rect::new(x, 206.0, 88.0, 190.0));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, color);
        draw_line(rect.x, rect.y, rect.right(), rect.bottom(), 2.0, color);
    }
}

fn draw_cryo_pods(view: &WorldView, color: Color) {
    for i in 0..5 {
        let rect = view.rect(Rect::new(126.0 + i as f32 * 170.0, 232.0, 92.0, 44.0));
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.0,
            Color::new(0.9, 1.0, 1.0, 0.45),
        );
    }
}

fn draw_scanner_gates(view: &WorldView, color: Color) {
    for x in [280.0, 610.0, 930.0] {
        let rect = view.rect(Rect::new(x, 220.0, 42.0, 180.0));
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_circle(
            rect.x + rect.w * 0.5,
            rect.y + 34.0,
            8.0,
            Color::new(0.95, 1.0, 1.0, 0.8),
        );
    }
}

fn draw_drone_dots(view: &WorldView, color: Color) {
    for i in 0..8 {
        let p = view.point(vec2(
            170.0 + i as f32 * 105.0,
            214.0 + (i % 3) as f32 * 36.0,
        ));
        draw_circle(p.x, p.y, 12.0, color);
        draw_line(p.x - 18.0, p.y, p.x - 6.0, p.y, 2.0, color);
        draw_line(p.x + 6.0, p.y, p.x + 18.0, p.y, 2.0, color);
    }
}

fn draw_cell_doors(view: &WorldView, color: Color) {
    for i in 0..6 {
        let rect = view.rect(Rect::new(126.0 + i as f32 * 150.0, 204.0, 82.0, 166.0));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, color);
        for bar in 1..4 {
            let x = rect.x + rect.w * bar as f32 / 4.0;
            draw_line(x, rect.y, x, rect.bottom(), 1.5, color);
        }
    }
}

fn draw_light_beams(view: &WorldView, color: Color) {
    for x in [180.0, 520.0, 860.0] {
        let a = view.point(vec2(x, 160.0));
        let b = view.point(vec2(x + 130.0, 430.0));
        let c = view.point(vec2(x - 80.0, 430.0));
        draw_triangle(a, b, c, color);
    }
}

fn draw_targeting_marks(view: &WorldView, color: Color) {
    for p in [vec2(252.0, 252.0), vec2(564.0, 240.0), vec2(874.0, 260.0)] {
        let c = view.point(p);
        draw_circle_lines(c.x, c.y, 24.0, 2.0, color);
        draw_line(c.x - 32.0, c.y, c.x + 32.0, c.y, 1.5, color);
        draw_line(c.x, c.y - 32.0, c.x, c.y + 32.0, 1.5, color);
    }
}

fn draw_laundry_tubes(view: &WorldView, color: Color) {
    for i in 0..3 {
        let y = 210.0 + i as f32 * 58.0;
        let a = view.point(vec2(120.0, y));
        let b = view.point(vec2(1000.0, y + 42.0));
        draw_line(a.x, a.y, b.x, b.y, 8.0, color);
        draw_line(a.x, a.y, b.x, b.y, 2.0, Color::new(1.0, 1.0, 1.0, 0.24));
    }
}

fn draw_command_panels(view: &WorldView, color: Color) {
    for i in 0..5 {
        let rect = view.rect(Rect::new(160.0 + i as f32 * 170.0, 218.0, 96.0, 64.0));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, color);
        draw_circle(rect.x + 18.0, rect.y + 18.0, 5.0, color);
        draw_line(
            rect.x + 34.0,
            rect.y + 18.0,
            rect.right() - 12.0,
            rect.y + 18.0,
            2.0,
            color,
        );
    }
}

fn draw_reactor_pipes(view: &WorldView, color: Color) {
    for i in 0..5 {
        let x = 130.0 + i as f32 * 190.0;
        let a = view.point(vec2(x, 190.0));
        let b = view.point(vec2(x + 76.0, 426.0));
        draw_line(a.x, a.y, b.x, b.y, 7.0, color);
        draw_circle(a.x, a.y, 10.0, color);
    }
}

fn draw_lift_rails(view: &WorldView, color: Color) {
    for x in [300.0, 560.0, 820.0] {
        let a = view.point(vec2(x, 166.0));
        let b = view.point(vec2(x, 452.0));
        draw_line(a.x, a.y, b.x, b.y, 5.0, color);
        for y in [220.0, 300.0, 380.0] {
            let c = view.point(vec2(x - 42.0, y));
            let d = view.point(vec2(x + 42.0, y));
            draw_line(c.x, c.y, d.x, d.y, 2.0, color);
        }
    }
}

fn draw_firewall_nodes(view: &WorldView, color: Color) {
    let nodes = [
        vec2(190.0, 230.0),
        vec2(360.0, 292.0),
        vec2(560.0, 216.0),
        vec2(760.0, 306.0),
        vec2(936.0, 236.0),
    ];
    for pair in nodes.windows(2) {
        let a = view.point(pair[0]);
        let b = view.point(pair[1]);
        draw_line(a.x, a.y, b.x, b.y, 2.0, color);
    }
    for node in nodes {
        let p = view.point(node);
        draw_circle(p.x, p.y, 13.0, color);
    }
}

fn draw_smoke_band(view: &WorldView, alpha: f32) {
    for i in 0..8 {
        let p = view.point(vec2(
            160.0 + i as f32 * 118.0,
            280.0 + (i % 3) as f32 * 26.0,
        ));
        draw_circle(p.x, p.y, 44.0, Color::new(0.72, 0.76, 0.74, alpha));
    }
}

fn draw_foam_bubbles(view: &WorldView, count: usize, alpha: f32) {
    for i in 0..count {
        let p = view.point(vec2(150.0 + i as f32 * 92.0, 400.0 - (i % 4) as f32 * 42.0));
        draw_circle(p.x, p.y, 22.0, Color::new(0.86, 0.96, 1.0, alpha));
        draw_circle_lines(
            p.x,
            p.y,
            22.0,
            2.0,
            Color::new(0.96, 1.0, 1.0, alpha + 0.12),
        );
    }
}

fn draw_slip_arrows(view: &WorldView, color: Color) {
    for i in 0..7 {
        let a = view.point(vec2(140.0 + i as f32 * 124.0, FLOOR_Y - 18.0));
        let b = view.point(vec2(210.0 + i as f32 * 124.0, FLOOR_Y - 18.0));
        draw_line(a.x, a.y, b.x, b.y, 3.0, color);
        draw_triangle(
            b,
            vec2(b.x - 12.0, b.y - 7.0),
            vec2(b.x - 12.0, b.y + 7.0),
            color,
        );
    }
}

fn draw_overgrown_vines(view: &WorldView) {
    draw_vines(view, Color::new(0.40, 1.0, 0.34, 0.62));
    draw_vines(view, Color::new(0.22, 0.72, 0.26, 0.42));
}

fn draw_pressure_bursts(view: &WorldView) {
    for x in [220.0, 520.0, 820.0] {
        let p = view.point(vec2(x, 390.0));
        draw_circle(p.x, p.y, 34.0, Color::new(0.74, 0.90, 1.0, 0.16));
        draw_circle_lines(p.x, p.y, 46.0, 2.0, Color::new(0.86, 1.0, 1.0, 0.28));
    }
}

fn draw_blanket_piles(view: &WorldView) {
    for i in 0..5 {
        let rect = view.rect(Rect::new(170.0 + i as f32 * 170.0, 430.0, 86.0, 30.0));
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.92, 0.72, 0.92, 0.34),
        );
    }
}

fn draw_wrong_badge_marks(view: &WorldView) {
    for i in 0..5 {
        let rect = view.rect(Rect::new(170.0 + i as f32 * 160.0, 240.0, 54.0, 34.0));
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.0,
            Color::new(1.0, 0.62, 0.35, 0.58),
        );
        draw_line(
            rect.x,
            rect.y,
            rect.right(),
            rect.bottom(),
            2.0,
            Color::new(1.0, 0.32, 0.20, 0.62),
        );
    }
}

fn draw_revolving_arrows(view: &WorldView) {
    for x in [250.0, 560.0, 870.0] {
        let p = view.point(vec2(x, 292.0));
        draw_circle_lines(p.x, p.y, 38.0, 3.0, Color::new(1.0, 0.68, 0.36, 0.44));
        draw_triangle(
            vec2(p.x + 34.0, p.y - 6.0),
            vec2(p.x + 50.0, p.y),
            vec2(p.x + 34.0, p.y + 6.0),
            Color::new(1.0, 0.68, 0.36, 0.44),
        );
    }
}

fn draw_shadow_lanes(view: &WorldView) {
    for i in 0..4 {
        let rect = view.rect(Rect::new(100.0 + i as f32 * 250.0, 170.0, 110.0, 300.0));
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.0, 0.0, 0.0, 0.24),
        );
    }
}

fn draw_lift_confusion(view: &WorldView) {
    draw_lift_rails(view, Color::new(1.0, 0.58, 0.38, 0.42));
    for y in [210.0, 314.0, 418.0] {
        let a = view.point(vec2(360.0, y));
        let b = view.point(vec2(790.0, y + 42.0));
        draw_line(a.x, a.y, b.x, b.y, 3.0, Color::new(1.0, 0.38, 0.22, 0.46));
    }
}
