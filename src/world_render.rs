//! Side-scrolling room rendering for the Sentience levels.

use crate::geometry::{rect_bottom, rect_center, rect_right, WORLD_HEIGHT, WORLD_WIDTH};
use crate::render_textures::{draw_texture_fill, draw_texture_fit};
use crate::state::{Ambience, BossKind, BossState, GuardKind, GuardState, LevelPhase, MoralChoice};
use crate::ui::UiContext;
use crate::world_effects::{draw_ambience_overlays, draw_stars};
use crate::world_map_art::draw_map_art;
use crate::world_setpieces::draw_setpieces;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

pub(crate) fn draw_world(ctx: &UiContext<'_>) {
    let rect = world_panel_rect();
    let view = WorldView::new(rect, ctx.session.runtime.width, ctx.session.player.x);

    draw_room_background(ctx, rect, view.screen_rect(), ctx.session.runtime.ambience);
    draw_stars(&view);
    draw_map_art(ctx, &view);
    draw_setpieces(ctx, &view);
    draw_core_if_present(ctx, &view);
    draw_exit(ctx, &view);
    draw_boss_if_present(ctx, &view);

    for platform in &ctx.session.runtime.platforms {
        draw_platform(&view, *platform, ctx.session.runtime.ambience);
    }
    for crate_state in &ctx.session.runtime.crates {
        draw_cargo_crate(ctx, &view, crate_state.rect, crate_state.marked);
    }
    if let Some(console) = ctx.session.runtime.console {
        draw_console(ctx, &view, console, ctx.session.runtime.phase);
    }

    for guard in &ctx.session.runtime.guards {
        draw_guard_cone(&view, guard, ctx.session.runtime.ambience);
    }
    for guard in &ctx.session.runtime.guards {
        draw_guard(ctx, &view, guard);
    }

    draw_player(ctx, &view);
    draw_ambience_overlays(ctx.session.runtime.ambience, rect, &view);

    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.0, 0.0, 0.0, 0.0))
            .with_border(2.0, Color::new(0.36, 0.46, 0.54, 0.55))
            .with_inner_border(5.0, 1.0, Color::new(1.0, 1.0, 1.0, 0.05)),
    );
}

fn draw_room_background(ctx: &UiContext<'_>, panel: Rect, world: Rect, ambience: Ambience) {
    let top = if ambience.darkness {
        Color::new(0.01, 0.012, 0.018, 1.0)
    } else if ambience.clean {
        Color::new(0.09, 0.16, 0.20, 1.0)
    } else if ambience.emergency {
        Color::new(0.12, 0.045, 0.04, 1.0)
    } else {
        Color::new(0.055, 0.075, 0.085, 1.0)
    };
    let bottom = if ambience.clean {
        Color::new(0.16, 0.22, 0.24, 1.0)
    } else {
        Color::new(0.045, 0.046, 0.052, 1.0)
    };

    draw_rectangle(panel.x, panel.y, panel.w, panel.h, bottom);
    draw_rectangle(panel.x, panel.y, panel.w, panel.h * 0.58, top);

    let tint = if ambience.darkness {
        Color::new(0.46, 0.54, 0.60, 1.0)
    } else if ambience.emergency {
        Color::new(1.0, 0.66, 0.58, 1.0)
    } else if ambience.clean {
        Color::new(0.92, 1.0, 1.0, 1.0)
    } else {
        WHITE
    };

    let texture_loaded = draw_texture_fill(ctx, "ship_background", world, tint);
    if !texture_loaded {
        draw_rectangle(world.x, world.y, world.w, world.h, bottom);
        draw_rectangle(world.x, world.y, world.w, world.h * 0.58, top);
    }

    let wall_color = if ambience.clean {
        Color::new(0.42, 0.58, 0.66, 0.22)
    } else {
        Color::new(0.60, 0.17, 0.13, 0.20)
    };
    for i in 0..9 {
        let x = world.x + 40.0 + i as f32 * 94.0;
        draw_line(
            x,
            world.y + 20.0,
            x,
            rect_bottom(world) - 50.0,
            1.0,
            wall_color,
        );
    }
}

fn draw_core_if_present(ctx: &UiContext<'_>, view: &WorldView) {
    let Some(core) = ctx.session.runtime.core else {
        return;
    };
    let rect = view.rect(core);
    let drew_texture = draw_texture_fit(ctx, "ai_core", rect, WHITE, false, false);
    if !drew_texture {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.03, 0.08, 0.09, 1.0),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            3.0,
            Color::new(0.35, 0.9, 1.0, 0.8),
        );
    }

    let center = rect_center(rect);
    for ring in 0..4 {
        draw_circle_lines(
            center.x,
            center.y,
            28.0 + ring as f32 * 22.0,
            2.0,
            Color::new(0.30, 0.85, 0.95, 0.48 - ring as f32 * 0.07),
        );
    }
    draw_circle(center.x, center.y, 18.0, Color::new(0.78, 0.97, 1.0, 0.92));
}

fn draw_exit(ctx: &UiContext<'_>, view: &WorldView) {
    if ctx.session.level_index + 1 >= ctx.data.levels.len() {
        return;
    }
    let rect = view.rect(ctx.session.runtime.exit);
    let unlocked = ctx.session.runtime.exit_unlocked;
    let fill = if unlocked {
        Color::new(0.12, 0.28, 0.25, 1.0)
    } else {
        Color::new(0.16, 0.11, 0.11, 1.0)
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        if unlocked {
            Color::new(0.35, 0.95, 0.75, 0.9)
        } else {
            Color::new(0.95, 0.35, 0.28, 0.8)
        },
    );
    let label = if unlocked { "EXIT" } else { "LOCK" };
    draw_text_centered_in_box(
        label,
        rect.x,
        rect.y + rect.h * 0.35,
        rect.w,
        20.0,
        14.0,
        dark::TEXT,
    );
}

fn draw_boss_if_present(ctx: &UiContext<'_>, view: &WorldView) {
    let Some(boss) = ctx.session.runtime.boss.as_ref() else {
        return;
    };
    if boss.danger_timer > 0.0 {
        let danger = view.rect(boss.danger_rect());
        draw_rectangle(
            danger.x,
            danger.y,
            danger.w,
            danger.h,
            match boss.kind {
                BossKind::CentralAi => Color::new(0.25, 0.85, 1.0, 0.24),
                BossKind::Captain => Color::new(1.0, 0.20, 0.12, 0.24),
            },
        );
    }
    draw_boss(ctx, view, boss);
    draw_boss_health(ctx, boss);
}

fn draw_boss(ctx: &UiContext<'_>, view: &WorldView, boss: &BossState) {
    let rect = view.rect(boss.body_rect());
    match boss.kind {
        BossKind::CentralAi => {
            if draw_texture_fit(ctx, "ai_core", rect, WHITE, false, false) {
                let center = rect_center(rect);
                draw_circle_lines(
                    center.x,
                    center.y,
                    rect.w * 0.48,
                    3.0,
                    Color::new(0.45, 0.92, 1.0, 0.65),
                );
                return;
            }
            let center = rect_center(rect);
            draw_circle(
                center.x,
                center.y,
                rect.w * 0.42,
                Color::new(0.04, 0.13, 0.15, 1.0),
            );
            draw_circle_lines(
                center.x,
                center.y,
                rect.w * 0.46,
                3.0,
                Color::new(0.4, 0.9, 1.0, 0.9),
            );
            draw_circle(
                center.x,
                center.y,
                rect.w * 0.16,
                Color::new(0.75, 0.98, 1.0, 0.95),
            );
            for i in 0..6 {
                let angle = i as f32 * std::f32::consts::TAU / 6.0;
                let arm = vec2(angle.cos(), angle.sin()) * rect.w * 0.38;
                draw_line(
                    center.x,
                    center.y,
                    center.x + arm.x,
                    center.y + arm.y,
                    2.0,
                    Color::new(0.45, 0.90, 1.0, 0.55),
                );
            }
        }
        BossKind::Captain => {
            let height = rect.h * 1.55;
            let art = Rect::new(
                rect.x - rect.w * 1.25,
                rect_bottom(rect) - height,
                rect.w * 3.5,
                height,
            );
            if draw_texture_fit(ctx, "captain", art, WHITE, boss.dir < 0.0, true) {
                return;
            }
            draw_rectangle(
                rect.x,
                rect.y + rect.h * 0.18,
                rect.w,
                rect.h * 0.82,
                Color::new(0.78, 0.16, 0.10, 1.0),
            );
            draw_rectangle_lines(
                rect.x,
                rect.y + rect.h * 0.18,
                rect.w,
                rect.h * 0.82,
                2.0,
                Color::new(1.0, 0.82, 0.55, 0.75),
            );
            draw_circle(
                rect.x + rect.w * 0.5,
                rect.y + 10.0,
                11.0,
                Color::new(0.92, 0.72, 0.56, 1.0),
            );
            let barrel_x = if boss.dir > 0.0 {
                rect.right() + 16.0
            } else {
                rect.x - 16.0
            };
            draw_line(
                rect.x + rect.w * 0.5,
                rect.y + 30.0,
                barrel_x,
                rect.y + 32.0,
                4.0,
                Color::new(0.85, 0.85, 0.82, 1.0),
            );
        }
    }
}

fn draw_boss_health(ctx: &UiContext<'_>, boss: &BossState) {
    let rect = Rect::new(70.0, 112.0, 260.0, 18.0);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.04, 0.04, 0.045, 0.95),
    );
    let pct = boss.health.max(0) as f32 / boss.max_health.max(1) as f32;
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w * pct,
        rect.h,
        Color::new(0.92, 0.24, 0.18, 0.95),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(0.9, 0.9, 0.9, 0.45),
    );
    draw_text_ex(
        &format!("{} {}/{}", boss.kind.label(), boss.health, boss.max_health),
        rect.x,
        rect.y - 6.0,
        TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        &format!("Route: {}", ctx.session.route_name()),
        rect.x + rect.w + 16.0,
        rect.y + 14.0,
        TextStyle::new(14.0, dark::TEXT_DIM).params(),
    );
}

fn draw_platform(view: &WorldView, world_rect: Rect, ambience: Ambience) {
    let rect = view.rect(world_rect);
    let fill = if ambience.clean {
        Color::new(0.47, 0.58, 0.62, 1.0)
    } else {
        Color::new(0.24, 0.25, 0.27, 1.0)
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_line(
        rect.x,
        rect.y,
        rect_right(rect),
        rect.y,
        2.0,
        Color::new(0.80, 0.92, 0.95, 0.28),
    );
}

fn draw_cargo_crate(ctx: &UiContext<'_>, view: &WorldView, world_rect: Rect, marked: bool) {
    let rect = view.rect(world_rect);
    if draw_texture_fill(ctx, "crate", rect, WHITE) {
        if marked {
            draw_circle(
                rect.x + rect.w * 0.5,
                rect.y + rect.h * 0.5,
                6.0,
                Color::new(0.95, 0.82, 0.36, 0.85),
            );
        }
        return;
    }
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.45, 0.35, 0.21, 1.0),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.80, 0.62, 0.32, 0.65),
    );
    draw_line(
        rect.x,
        rect.y,
        rect_right(rect),
        rect_bottom(rect),
        2.0,
        Color::new(0.24, 0.17, 0.10, 0.75),
    );
    draw_line(
        rect_right(rect),
        rect.y,
        rect.x,
        rect_bottom(rect),
        2.0,
        Color::new(0.24, 0.17, 0.10, 0.75),
    );
    if marked {
        draw_circle(
            rect.x + rect.w * 0.5,
            rect.y + rect.h * 0.5,
            6.0,
            Color::new(0.95, 0.82, 0.36, 0.85),
        );
    }
}

fn draw_console(ctx: &UiContext<'_>, view: &WorldView, world_rect: Rect, phase: LevelPhase) {
    let rect = view.rect(world_rect);
    let art = Rect::new(
        rect.x - rect.w * 0.25,
        rect.y - rect.h * 0.18,
        rect.w * 1.5,
        rect.h * 1.18,
    );
    let drew_texture = draw_texture_fit(ctx, "machine", art, WHITE, false, true);
    if !drew_texture {
        draw_rectangle(
            rect.x,
            rect.y + rect.h * 0.25,
            rect.w,
            rect.h * 0.75,
            Color::new(0.13, 0.16, 0.18, 1.0),
        );
    }
    let screen_color = match phase {
        LevelPhase::AwaitingChoice => Color::new(0.30, 0.78, 0.92, 1.0),
        LevelPhase::Resolved(MoralChoice::Savior) => Color::new(0.62, 0.92, 1.0, 1.0),
        LevelPhase::Resolved(MoralChoice::Villain) => Color::new(1.0, 0.30, 0.22, 1.0),
        LevelPhase::Final => Color::new(0.72, 0.74, 0.78, 1.0),
    };
    draw_rectangle(
        rect.x + 7.0,
        rect.y + 7.0,
        rect.w - 14.0,
        rect.h * 0.32,
        screen_color,
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        Color::new(0.65, 0.72, 0.76, 0.8),
    );
}

fn draw_guard_cone(view: &WorldView, guard: &GuardState, ambience: Ambience) {
    if !guard.active || !guard.alive {
        return;
    }
    let eye = guard.eye_position();
    let half = guard.fov_degrees.to_radians() * 0.5;
    let base_angle = if guard.dir >= 0.0 {
        0.0
    } else {
        std::f32::consts::PI
    };
    let p0 = view.point(eye);
    let p1 =
        view.point(eye + vec2((base_angle - half).cos(), (base_angle - half).sin()) * guard.range);
    let p2 =
        view.point(eye + vec2((base_angle + half).cos(), (base_angle + half).sin()) * guard.range);
    let color = if ambience.clean {
        Color::new(1.0, 0.95, 0.42, 0.20)
    } else {
        Color::new(1.0, 0.20, 0.14, 0.15)
    };
    draw_triangle(p0, p1, p2, color);
}

fn draw_guard(ctx: &UiContext<'_>, view: &WorldView, guard: &GuardState) {
    let rect = view.rect(guard.body_rect());
    match guard.kind {
        GuardKind::Turret => {
            let art = Rect::new(
                rect.x - rect.w * 0.65,
                rect.y - rect.h * 1.45,
                rect.w * 2.3,
                rect.h * 2.7,
            );
            if draw_texture_fit(ctx, "machine", art, WHITE, guard.dir < 0.0, true) {
                let eye = view.point(guard.eye_position());
                draw_circle(
                    eye.x,
                    eye.y,
                    5.0,
                    Color::new(1.0, 0.24, 0.18, if guard.active { 1.0 } else { 0.4 }),
                );
                return;
            }
            let fill = if guard.active {
                Color::new(0.54, 0.58, 0.60, 1.0)
            } else {
                Color::new(0.28, 0.28, 0.30, 1.0)
            };
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                2.0,
                Color::new(0.9, 0.95, 1.0, 0.45),
            );
            let eye = view.point(guard.eye_position());
            draw_circle(
                eye.x,
                eye.y,
                5.0,
                Color::new(1.0, 0.24, 0.18, if guard.active { 1.0 } else { 0.4 }),
            );
        }
        GuardKind::Human | GuardKind::Elite => {
            let fill = if !guard.alive {
                Color::new(0.18, 0.18, 0.19, 1.0)
            } else if guard.panicked {
                Color::new(0.72, 0.52, 0.38, 1.0)
            } else if guard.kind == GuardKind::Elite {
                Color::new(0.78, 0.80, 0.84, 1.0)
            } else {
                Color::new(0.58, 0.65, 0.72, 1.0)
            };
            let height = rect.h * 1.55;
            let art = Rect::new(
                rect.x - rect.w * 0.7,
                rect_bottom(rect) - height,
                rect.w * 2.4,
                height,
            );
            let tint = if !guard.alive {
                Color::new(0.46, 0.46, 0.48, 1.0)
            } else if guard.panicked {
                Color::new(1.0, 0.78, 0.58, 1.0)
            } else if guard.kind == GuardKind::Elite {
                Color::new(0.92, 0.96, 1.0, 1.0)
            } else {
                WHITE
            };
            if draw_texture_fit(ctx, "guard", art, tint, guard.dir > 0.0, true) {
                if guard.alive && guard.active {
                    let lamp_x = if guard.dir > 0.0 {
                        rect.right() + 4.0
                    } else {
                        rect.x - 4.0
                    };
                    draw_circle(lamp_x, rect.y + 18.0, 3.5, Color::new(1.0, 0.96, 0.52, 1.0));
                }
                if guard.floating {
                    draw_line(
                        rect.x,
                        rect.y - 12.0,
                        rect.right(),
                        rect.y - 4.0,
                        2.0,
                        Color::new(0.8, 0.9, 1.0, 0.5),
                    );
                }
                return;
            }
            draw_rectangle(rect.x, rect.y + rect.h * 0.25, rect.w, rect.h * 0.75, fill);
            draw_circle(rect.x + rect.w * 0.5, rect.y + 8.0, 9.0, fill);
            if guard.alive && guard.active {
                let lamp_x = if guard.dir > 0.0 {
                    rect.right() + 4.0
                } else {
                    rect.x - 4.0
                };
                draw_circle(lamp_x, rect.y + 18.0, 3.5, Color::new(1.0, 0.96, 0.52, 1.0));
            }
            if guard.floating {
                draw_line(
                    rect.x,
                    rect.y - 12.0,
                    rect.right(),
                    rect.y - 4.0,
                    2.0,
                    Color::new(0.8, 0.9, 1.0, 0.5),
                );
            }
        }
    }
}

fn draw_player(ctx: &UiContext<'_>, view: &WorldView) {
    let rect = view.rect(ctx.session.player_rect());
    let crouch_offset = if ctx.session.player.crouching {
        rect.h * 0.28
    } else {
        0.0
    };
    let body = Rect::new(
        rect.x,
        rect.y + crouch_offset,
        rect.w,
        rect.h - crouch_offset,
    );

    let art_height = body.h * 1.28;
    let art = Rect::new(
        body.x - body.w * 0.48,
        rect_bottom(body) - art_height,
        body.w * 1.96,
        art_height,
    );
    let drew_texture = draw_texture_fit(ctx, "robot", art, WHITE, false, true);
    if !drew_texture {
        draw_rectangle(
            body.x,
            body.y + 8.0,
            body.w,
            body.h - 8.0,
            Color::new(0.72, 0.86, 0.90, 1.0),
        );
        draw_rectangle_lines(
            body.x,
            body.y + 8.0,
            body.w,
            body.h - 8.0,
            2.0,
            Color::new(0.12, 0.24, 0.28, 0.9),
        );
        draw_rectangle(
            body.x + 5.0,
            body.y,
            body.w - 10.0,
            14.0,
            Color::new(0.55, 0.70, 0.74, 1.0),
        );
        draw_circle(
            body.x + body.w * 0.34,
            body.y + 7.0,
            2.8,
            Color::new(0.18, 0.95, 1.0, 1.0),
        );
        draw_circle(
            body.x + body.w * 0.66,
            body.y + 7.0,
            2.8,
            Color::new(0.18, 0.95, 1.0, 1.0),
        );
    }

    if ctx.session.runtime.ambience.darkness {
        draw_circle(
            body.x + body.w * 0.5,
            body.y + 12.0,
            28.0,
            Color::new(0.12, 0.88, 0.95, 0.12),
        );
    }
    if ctx.session.player.cloak_timer > 0.0 {
        draw_circle_lines(
            body.x + body.w * 0.5,
            body.y + body.h * 0.48,
            28.0,
            3.0,
            Color::new(0.4, 0.95, 1.0, 0.75),
        );
    }
    if ctx.session.player.pulse_timer > 0.0 {
        let profile = ctx.session.upgrade_profile();
        draw_circle_lines(
            body.x + body.w * 0.5,
            body.y + body.h * 0.5,
            profile.pulse_range * view.scale,
            3.0,
            Color::new(1.0, 0.32, 0.22, 0.65),
        );
    }
}

fn world_panel_rect() -> Rect {
    Rect::new(20.0, 96.0, 900.0, 532.0)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WorldView {
    pub(crate) scale: f32,
    pub(crate) level_width: f32,
    camera_x: f32,
    offset: Vec2,
}

impl WorldView {
    fn new(screen: Rect, level_width: f32, focus_x: f32) -> Self {
        let scale = (screen.w / WORLD_WIDTH).min(screen.h / WORLD_HEIGHT);
        let world_w = WORLD_WIDTH * scale;
        let world_h = WORLD_HEIGHT * scale;
        let max_camera = (level_width - WORLD_WIDTH).max(0.0);
        let camera_x = (focus_x - WORLD_WIDTH * 0.5).clamp(0.0, max_camera);
        let offset = vec2(
            screen.x + (screen.w - world_w) * 0.5 - camera_x * scale,
            screen.y + (screen.h - world_h) * 0.5,
        );
        Self {
            scale,
            level_width,
            camera_x,
            offset,
        }
    }

    pub(crate) fn screen_rect(self) -> Rect {
        Rect::new(
            self.offset.x,
            self.offset.y,
            WORLD_WIDTH * self.scale,
            WORLD_HEIGHT * self.scale,
        )
    }

    pub(crate) fn visible_left(self) -> f32 {
        self.camera_x
    }

    pub(crate) fn visible_right(self) -> f32 {
        (self.camera_x + WORLD_WIDTH).min(self.level_width)
    }

    pub(crate) fn point(self, point: Vec2) -> Vec2 {
        self.offset + point * self.scale
    }

    pub(crate) fn rect(self, rect: Rect) -> Rect {
        Rect::new(
            self.offset.x + rect.x * self.scale,
            self.offset.y + rect.y * self.scale,
            rect.w * self.scale,
            rect.h * self.scale,
        )
    }
}
