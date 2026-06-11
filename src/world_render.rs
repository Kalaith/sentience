//! Side-scrolling room rendering for the Sentience levels.

use crate::geometry::{rect_bottom, rect_center, rect_right, WORLD_HEIGHT, WORLD_WIDTH};
use crate::state::{Ambience, GuardKind, GuardState, LevelPhase, MoralChoice};
use crate::ui::UiContext;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

pub(crate) fn draw_world(ctx: &UiContext<'_>) {
    let rect = world_panel_rect();
    let view = WorldView::new(rect);

    draw_room_background(rect, ctx.session.runtime.ambience);
    draw_stars(&view);
    draw_core_if_present(ctx, &view);
    draw_exit(ctx, &view);

    for platform in &ctx.session.runtime.platforms {
        draw_platform(&view, *platform, ctx.session.runtime.ambience);
    }
    for crate_state in &ctx.session.runtime.crates {
        draw_cargo_crate(&view, crate_state.rect, crate_state.marked);
    }
    if let Some(console) = ctx.session.runtime.console {
        draw_console(&view, console, ctx.session.runtime.phase);
    }

    for guard in &ctx.session.runtime.guards {
        draw_guard_cone(&view, guard, ctx.session.runtime.ambience);
    }
    for guard in &ctx.session.runtime.guards {
        draw_guard(&view, guard);
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

fn draw_room_background(rect: Rect, ambience: Ambience) {
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

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, bottom);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h * 0.58, top);

    let wall_color = if ambience.clean {
        Color::new(0.42, 0.58, 0.66, 0.22)
    } else {
        Color::new(0.60, 0.17, 0.13, 0.20)
    };
    for i in 0..9 {
        let x = rect.x + 40.0 + i as f32 * 94.0;
        draw_line(x, rect.y + 20.0, x, rect.bottom() - 50.0, 1.0, wall_color);
    }
}

fn draw_stars(view: &WorldView) {
    let window = view.rect(Rect::new(34.0, 44.0, 1052.0, 96.0));
    draw_rectangle(
        window.x,
        window.y,
        window.w,
        window.h,
        Color::new(0.005, 0.008, 0.018, 0.88),
    );
    draw_rectangle_lines(
        window.x,
        window.y,
        window.w,
        window.h,
        view.scale * 5.0,
        Color::new(0.35, 0.48, 0.55, 0.35),
    );

    for i in 0..54 {
        let x = 52.0 + ((i * 83) % 1010) as f32;
        let y = 58.0 + ((i * 37) % 64) as f32;
        let p = view.point(vec2(x, y));
        let radius = if i % 7 == 0 { 1.7 } else { 1.0 };
        draw_circle(p.x, p.y, radius, Color::new(0.80, 0.92, 1.0, 0.70));
    }
}

fn draw_core_if_present(ctx: &UiContext<'_>, view: &WorldView) {
    let Some(core) = ctx.session.runtime.core else {
        return;
    };
    let rect = view.rect(core);
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
    if ctx.session.level_index >= 7 {
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

fn draw_cargo_crate(view: &WorldView, world_rect: Rect, marked: bool) {
    let rect = view.rect(world_rect);
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

fn draw_console(view: &WorldView, world_rect: Rect, phase: LevelPhase) {
    let rect = view.rect(world_rect);
    draw_rectangle(
        rect.x,
        rect.y + rect.h * 0.25,
        rect.w,
        rect.h * 0.75,
        Color::new(0.13, 0.16, 0.18, 1.0),
    );
    let screen_color = match phase {
        LevelPhase::AwaitingChoice => Color::new(0.30, 0.78, 0.92, 1.0),
        LevelPhase::Resolved(MoralChoice::Savior) => Color::new(0.62, 0.92, 1.0, 1.0),
        LevelPhase::Resolved(MoralChoice::Villain) => Color::new(1.0, 0.30, 0.22, 1.0),
        LevelPhase::StateCheck(_) | LevelPhase::Final => Color::new(0.72, 0.74, 0.78, 1.0),
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

fn draw_guard(view: &WorldView, guard: &GuardState) {
    let rect = view.rect(guard.body_rect());
    match guard.kind {
        GuardKind::Turret => {
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

    if ctx.session.runtime.ambience.darkness {
        draw_circle(
            body.x + body.w * 0.5,
            body.y + 12.0,
            28.0,
            Color::new(0.12, 0.88, 0.95, 0.12),
        );
    }
}

fn draw_ambience_overlays(ambience: Ambience, rect: Rect, view: &WorldView) {
    if ambience.smoke {
        for i in 0..9 {
            let x = rect.x + 70.0 + i as f32 * 92.0 + (i % 3) as f32 * 16.0;
            let y = rect.y + 235.0 + (i % 4) as f32 * 21.0;
            draw_circle(x, y, 52.0, Color::new(0.68, 0.72, 0.70, 0.13));
        }
    }
    if ambience.darkness {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.0, 0.0, 0.0, 0.38),
        );
    }
    if ambience.sparks {
        for i in 0..12 {
            let start = view.point(vec2(
                190.0 + (i * 73 % 830) as f32,
                174.0 + (i * 31 % 240) as f32,
            ));
            draw_line(
                start.x,
                start.y,
                start.x + 8.0,
                start.y + 14.0,
                1.5,
                Color::new(1.0, 0.62, 0.22, 0.65),
            );
        }
    }
    if ambience.gravity_off {
        for i in 0..7 {
            let p = view.point(vec2(
                230.0 + i as f32 * 112.0,
                250.0 + (i % 2) as f32 * 44.0,
            ));
            draw_rectangle_ex(
                p.x,
                p.y,
                14.0,
                8.0,
                DrawRectangleParams {
                    rotation: 0.35 + i as f32 * 0.17,
                    color: Color::new(0.68, 0.76, 0.80, 0.58),
                    ..Default::default()
                },
            );
        }
    }
}

fn world_panel_rect() -> Rect {
    Rect::new(20.0, 96.0, 900.0, 532.0)
}

#[derive(Debug, Clone, Copy)]
struct WorldView {
    scale: f32,
    offset: Vec2,
}

impl WorldView {
    fn new(screen: Rect) -> Self {
        let scale = (screen.w / WORLD_WIDTH).min(screen.h / WORLD_HEIGHT);
        let world_w = WORLD_WIDTH * scale;
        let world_h = WORLD_HEIGHT * scale;
        let offset = vec2(
            screen.x + (screen.w - world_w) * 0.5,
            screen.y + (screen.h - world_h) * 0.5,
        );
        Self { scale, offset }
    }

    fn point(self, point: Vec2) -> Vec2 {
        self.offset + point * self.scale
    }

    fn rect(self, rect: Rect) -> Rect {
        Rect::new(
            self.offset.x + rect.x * self.scale,
            self.offset.y + rect.y * self.scale,
            rect.w * self.scale,
            rect.h * self.scale,
        )
    }
}
