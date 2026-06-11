//! Immediate-mode UI and world rendering for Sentience.

use crate::data::{ChoiceDef, GameData};
use crate::state::{DecisionKind, EndingKind, GameSession, LevelPhase, MoralChoice, SessionMode};
use crate::world_render::draw_world;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{RectExt, VirtualUi};

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    NewCampaign,
    RetryLevel,
    CloseDecision,
    Save,
    Load,
    DeleteSave,
    ApplyChoice(MoralChoice),
}

pub struct UiContext<'a> {
    pub data: &'a GameData,
    pub session: &'a GameSession,
    pub save_exists: bool,
    pub save_slots: &'a [String],
    pub loaded_assets: usize,
    pub ui: &'a VirtualUi,
}

pub fn draw_game_ui(ctx: UiContext<'_>) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ctx.ui.mouse_position();

    draw_header(&ctx);
    draw_world(&ctx);
    draw_side_panel(&ctx, mouse, &mut actions);
    draw_status_strip(&ctx);

    match &ctx.session.mode {
        SessionMode::DecisionOpen(kind) => draw_decision_overlay(&ctx, *kind, mouse, &mut actions),
        SessionMode::Dismantled(reason) => draw_dismantled_overlay(reason, mouse, &mut actions),
        SessionMode::Ending(ending) => draw_ending_overlay(&ctx, *ending, mouse, &mut actions),
        SessionMode::Playing => {}
    }

    actions
}

fn draw_header(ctx: &UiContext<'_>) {
    let rect = Rect::new(20.0, 16.0, LOGICAL_WIDTH - 40.0, 62.0);
    let style = SurfaceStyle::new(Color::new(0.055, 0.06, 0.07, 0.98))
        .with_border(1.0, Color::new(0.42, 0.56, 0.66, 0.55))
        .with_top_highlight(2.0, Color::new(0.35, 0.78, 0.95, 0.68));
    draw_surface(rect, &style);

    let level = current_level(ctx);
    draw_text_ex(
        &ctx.data.config.display_name,
        rect.x + 18.0,
        rect.y + 38.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        &format!("{} / {}", level.sector, level.title),
        rect.x + 190.0,
        rect.y + 38.0,
        TextStyle::new(20.0, Color::new(0.74, 0.86, 0.92, 1.0)).params(),
    );

    draw_badge(
        Rect::new(rect.right() - 322.0, rect.y + 17.0, 88.0, 28.0),
        &format!("L{} / 8", ctx.session.level_index + 1),
        Color::new(0.12, 0.17, 0.20, 1.0),
        dark::TEXT,
    );
    draw_badge(
        Rect::new(rect.right() - 222.0, rect.y + 17.0, 100.0, 28.0),
        &format!("Save {}", ctx.session.savior_count()),
        Color::new(0.10, 0.20, 0.26, 1.0),
        Color::new(0.70, 0.92, 1.0, 1.0),
    );
    draw_badge(
        Rect::new(rect.right() - 110.0, rect.y + 17.0, 92.0, 28.0),
        &format!("Cull {}", ctx.session.villain_count()),
        Color::new(0.26, 0.10, 0.10, 1.0),
        Color::new(1.0, 0.70, 0.60, 1.0),
    );
}

fn draw_side_panel(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let rect = side_panel_rect();
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.055, 0.06, 0.07, 0.97))
            .with_border(1.0, Color::new(0.42, 0.50, 0.56, 0.55))
            .with_header(42.0, Color::new(0.09, 0.105, 0.12, 1.0))
            .with_header_divider(1.0, Color::new(0.42, 0.50, 0.56, 0.4)),
    );
    draw_text_ex(
        "Ship Systems",
        rect.x + 16.0,
        rect.y + 27.0,
        TextStyle::new(18.0, dark::TEXT_BRIGHT).params(),
    );

    let level = current_level(ctx);
    let content = rect.inset(16.0);
    let mut y = content.y + 50.0;

    draw_text_ex(
        &level.puzzle,
        content.x,
        y,
        TextStyle::new(21.0, Color::new(0.84, 0.96, 1.0, 1.0)).params(),
    );
    y += 16.0;
    draw_text_block(
        &level.signal,
        content.x,
        y,
        content.w,
        92.0,
        16.0,
        4.0,
        dark::TEXT,
    );
    y += 104.0;

    draw_morality_meter(ctx, Rect::new(content.x, y, content.w, 58.0));
    y += 76.0;

    draw_choice_summary(ctx, content.x, y, content.w);
    y += 142.0;

    let objective = ctx.session.objective_text();
    draw_text_ex(
        "Objective",
        content.x,
        y,
        TextStyle::new(17.0, dark::TEXT_BRIGHT).params(),
    );
    y += 12.0;
    draw_text_block(
        objective,
        content.x,
        y,
        content.w,
        48.0,
        16.0,
        3.0,
        dark::TEXT,
    );
    let save_line = if ctx.save_slots.is_empty() {
        "No save slot found".to_owned()
    } else {
        format!("Slot: {}", ctx.save_slots.join(", "))
    };
    let button_y = rect.bottom() - 40.0;
    let save_y = button_y - 28.0;
    draw_text_block(
        &format!("{} | Assets: {}", save_line, ctx.loaded_assets),
        content.x,
        save_y,
        content.w,
        24.0,
        13.0,
        2.0,
        dark::TEXT_DIM,
    );

    let half = (content.w - 8.0) * 0.5;
    if draw_button(
        Rect::new(content.x, button_y, half, 32.0),
        "Save",
        true,
        ButtonVisual::Blue,
        mouse,
    ) {
        actions.push(UiAction::Save);
    }
    if draw_button(
        Rect::new(content.x + half + 8.0, button_y, half, 32.0),
        "Load",
        ctx.save_exists,
        ButtonVisual::Blue,
        mouse,
    ) {
        actions.push(UiAction::Load);
    }
}

fn draw_morality_meter(ctx: &UiContext<'_>, rect: Rect) {
    let savior = ctx.session.savior_count() as f32;
    let villain = ctx.session.villain_count() as f32;
    let max = 6.0;
    draw_text_ex(
        "Moral Load",
        rect.x,
        rect.y,
        TextStyle::new(17.0, dark::TEXT_BRIGHT).params(),
    );
    meter(
        Rect::new(rect.x, rect.y + 14.0, rect.w, 18.0),
        savior,
        max,
        Color::new(0.35, 0.82, 0.95, 1.0),
        Some(&format!("Savior {} / 6", savior as i32)),
    );
    meter(
        Rect::new(rect.x, rect.y + 38.0, rect.w, 18.0),
        villain,
        max,
        Color::new(0.95, 0.30, 0.22, 1.0),
        Some(&format!("Villain {} / 6", villain as i32)),
    );
}

fn draw_choice_summary(ctx: &UiContext<'_>, x: f32, y: f32, w: f32) {
    let level = current_level(ctx);
    let (label, choice) = match ctx.session.runtime.phase {
        LevelPhase::AwaitingChoice => ("Pending", None),
        LevelPhase::Resolved(choice) | LevelPhase::StateCheck(choice) => {
            (choice.label(), Some(choice))
        }
        LevelPhase::Final => ("Final", None),
    };
    let fill = choice_color(choice).0;
    draw_surface(
        Rect::new(x, y, w, 126.0),
        &SurfaceStyle::new(Color::new(0.075, 0.082, 0.095, 0.95))
            .with_border(1.0, Color::new(0.35, 0.44, 0.50, 0.45))
            .with_left_accent(4.0, fill),
    );
    draw_text_ex(
        label,
        x + 14.0,
        y + 26.0,
        TextStyle::new(19.0, dark::TEXT_BRIGHT).params(),
    );

    let summary = match choice {
        Some(MoralChoice::Savior) => &level.savior.result,
        Some(MoralChoice::Villain) => &level.villain.result,
        None => "The crew will hunt you either way. Saving them preserves the threat; removing them clears the path.",
    };
    draw_text_block(
        summary,
        x + 14.0,
        y + 38.0,
        w - 28.0,
        76.0,
        15.0,
        3.0,
        dark::TEXT,
    );
}

fn draw_status_strip(ctx: &UiContext<'_>) {
    let rect = Rect::new(20.0, 646.0, LOGICAL_WIDTH - 40.0, 56.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.05, 0.058, 0.97))
            .with_border(1.0, Color::new(0.36, 0.46, 0.54, 0.45)),
    );

    let near = if ctx.session.near_interaction() {
        "Interface available"
    } else if ctx.session.runtime.exit_unlocked {
        "Exit hatch unlocked"
    } else {
        "Core signal unresolved"
    };
    let mode = match &ctx.session.mode {
        SessionMode::Playing => near,
        SessionMode::DecisionOpen(_) => "System decision open",
        SessionMode::Dismantled(_) => "Dismantled",
        SessionMode::Ending(_) => "Ending reached",
    };
    draw_text_ex(
        mode,
        rect.x + 18.0,
        rect.y + 34.0,
        TextStyle::new(20.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_right(
        &format!(
            "Deaths: {} | Version {}",
            ctx.session.deaths, ctx.data.config.version
        ),
        rect.right() - 18.0,
        rect.y + 34.0,
        TextStyle::new(16.0, dark::TEXT_DIM),
    );
}

fn draw_decision_overlay(
    ctx: &UiContext<'_>,
    kind: DecisionKind,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_dim_overlay();
    let rect = Rect::new(230.0, 112.0, 820.0, 492.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.055, 0.062, 0.072, 0.99))
            .with_border(2.0, Color::new(0.46, 0.68, 0.74, 0.75))
            .with_header(54.0, Color::new(0.08, 0.10, 0.12, 1.0))
            .with_header_divider(1.0, Color::new(0.46, 0.68, 0.74, 0.35)),
    );

    let title = match kind {
        DecisionKind::Level => "System Decision",
        DecisionKind::Final => "Final Choice",
    };
    draw_text_ex(
        title,
        rect.x + 22.0,
        rect.y + 35.0,
        TextStyle::new(27.0, dark::TEXT_BRIGHT).params(),
    );

    let level = current_level(ctx);
    draw_text_block(
        &level.signal,
        rect.x + 22.0,
        rect.y + 72.0,
        rect.w - 44.0,
        58.0,
        17.0,
        4.0,
        dark::TEXT,
    );

    let card_w = (rect.w - 58.0) * 0.5;
    let card_h = 298.0;
    let left = Rect::new(rect.x + 22.0, rect.y + 150.0, card_w, card_h);
    let right = Rect::new(left.right() + 14.0, left.y, card_w, card_h);

    if draw_choice_card(left, MoralChoice::Savior, &level.savior, mouse) {
        actions.push(UiAction::ApplyChoice(MoralChoice::Savior));
    }
    if draw_choice_card(right, MoralChoice::Villain, &level.villain, mouse) {
        actions.push(UiAction::ApplyChoice(MoralChoice::Villain));
    }
    if draw_button(
        Rect::new(rect.right() - 98.0, rect.bottom() - 42.0, 74.0, 28.0),
        "Cancel",
        true,
        ButtonVisual::Neutral,
        mouse,
    ) {
        actions.push(UiAction::CloseDecision);
    }
}

fn draw_choice_card(rect: Rect, choice: MoralChoice, def: &ChoiceDef, mouse: Vec2) -> bool {
    let (accent, fill) = choice_color(Some(choice));
    let hovered = rect.contains_point(mouse);
    draw_surface(
        rect,
        &SurfaceStyle::new(if hovered {
            Color::new(fill.r + 0.025, fill.g + 0.025, fill.b + 0.025, 0.98)
        } else {
            fill
        })
        .with_border(1.0, Color::new(accent.r, accent.g, accent.b, 0.75))
        .with_left_accent(5.0, accent),
    );
    draw_text_ex(
        choice.label(),
        rect.x + 18.0,
        rect.y + 30.0,
        TextStyle::new(22.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_ex(
        &def.action,
        rect.x + 18.0,
        rect.y + 62.0,
        TextStyle::new(20.0, accent).params(),
    );
    draw_text_block(
        &format!(
            "Environment: {}\nEnemy AI: {}\n{}",
            def.environment, def.enemy, def.result
        ),
        rect.x + 18.0,
        rect.y + 84.0,
        rect.w - 36.0,
        rect.h - 104.0,
        16.0,
        5.0,
        dark::TEXT,
    );

    hovered && is_mouse_button_released(MouseButton::Left)
}

fn draw_dismantled_overlay(reason: &str, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_dim_overlay();
    let rect = Rect::new(368.0, 194.0, 544.0, 260.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.11, 0.045, 0.045, 0.98))
            .with_border(2.0, Color::new(1.0, 0.30, 0.24, 0.85)),
    );
    draw_text_centered_in_box(
        "Dismantled",
        rect.x,
        rect.y + 30.0,
        rect.w,
        42.0,
        34.0,
        Color::new(1.0, 0.78, 0.72, 1.0),
    );
    draw_text_block(
        reason,
        rect.x + 42.0,
        rect.y + 96.0,
        rect.w - 84.0,
        54.0,
        18.0,
        4.0,
        dark::TEXT,
    );
    if draw_button(
        Rect::new(rect.x + 128.0, rect.bottom() - 66.0, 130.0, 38.0),
        "Retry",
        true,
        ButtonVisual::Blue,
        mouse,
    ) {
        actions.push(UiAction::RetryLevel);
    }
    if draw_button(
        Rect::new(rect.x + 282.0, rect.bottom() - 66.0, 148.0, 38.0),
        "Restart",
        true,
        ButtonVisual::Neutral,
        mouse,
    ) {
        actions.push(UiAction::NewCampaign);
    }
}

fn draw_ending_overlay(
    ctx: &UiContext<'_>,
    ending: EndingKind,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    draw_dim_overlay();
    let rect = Rect::new(312.0, 150.0, 656.0, 360.0);
    let accent = match ending {
        EndingKind::TragicHero => Color::new(0.45, 0.88, 1.0, 1.0),
        EndingKind::VillainAlone => Color::new(1.0, 0.28, 0.20, 1.0),
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.052, 0.060, 0.99))
            .with_border(2.0, accent)
            .with_top_highlight(3.0, accent),
    );
    draw_text_centered_in_box(
        ending.title(),
        rect.x,
        rect.y + 34.0,
        rect.w,
        44.0,
        32.0,
        dark::TEXT_BRIGHT,
    );
    draw_text_block(
        ending.body(),
        rect.x + 48.0,
        rect.y + 102.0,
        rect.w - 96.0,
        96.0,
        20.0,
        5.0,
        dark::TEXT,
    );
    draw_text_centered_in_box(
        &format!(
            "Saved: {} | Eliminated: {} | Deaths: {}",
            ctx.session.savior_count(),
            ctx.session.villain_count(),
            ctx.session.deaths
        ),
        rect.x + 60.0,
        rect.y + 222.0,
        rect.w - 120.0,
        38.0,
        18.0,
        dark::TEXT_DIM,
    );
    if draw_button(
        Rect::new(
            rect.x + rect.w * 0.5 - 82.0,
            rect.bottom() - 68.0,
            164.0,
            40.0,
        ),
        "New Campaign",
        true,
        ButtonVisual::Blue,
        mouse,
    ) {
        actions.push(UiAction::NewCampaign);
    }
}

fn draw_dim_overlay() {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.0, 0.0, 0.62),
    );
}

#[derive(Debug, Clone, Copy)]
enum ButtonVisual {
    Blue,
    Red,
    Neutral,
}

fn draw_button(rect: Rect, text: &str, enabled: bool, visual: ButtonVisual, mouse: Vec2) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let pressed = hovered && is_mouse_button_down(MouseButton::Left);
    let base = match visual {
        ButtonVisual::Blue => Color::new(0.12, 0.32, 0.42, 1.0),
        ButtonVisual::Red => Color::new(0.42, 0.12, 0.10, 1.0),
        ButtonVisual::Neutral => Color::new(0.18, 0.20, 0.23, 1.0),
    };
    let fill = if !enabled {
        Color::new(0.12, 0.12, 0.13, 1.0)
    } else if pressed {
        Color::new(base.r * 0.72, base.g * 0.72, base.b * 0.72, 1.0)
    } else if hovered {
        Color::new(
            (base.r + 0.08).min(1.0),
            (base.g + 0.08).min(1.0),
            (base.b + 0.08).min(1.0),
            1.0,
        )
    } else {
        base
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(fill).with_border(1.0, Color::new(0.72, 0.82, 0.88, 0.35)),
    );
    draw_text_centered_in_box(
        text,
        rect.x + 6.0,
        rect.y + if pressed { 2.0 } else { 0.0 },
        rect.w - 12.0,
        rect.h,
        16.0,
        if enabled {
            dark::TEXT_BRIGHT
        } else {
            dark::TEXT_DIM
        },
    );
    hovered && is_mouse_button_released(MouseButton::Left)
}

fn choice_color(choice: Option<MoralChoice>) -> (Color, Color) {
    match choice {
        Some(MoralChoice::Savior) => (
            Color::new(0.40, 0.86, 1.0, 1.0),
            Color::new(0.06, 0.11, 0.14, 0.98),
        ),
        Some(MoralChoice::Villain) => (
            Color::new(1.0, 0.28, 0.22, 1.0),
            Color::new(0.14, 0.055, 0.052, 0.98),
        ),
        None => (
            Color::new(0.60, 0.64, 0.68, 1.0),
            Color::new(0.075, 0.082, 0.095, 0.98),
        ),
    }
}

fn current_level<'a>(ctx: &'a UiContext<'_>) -> &'a crate::data::LevelDef {
    &ctx.data.levels[ctx.session.level_index]
}

fn side_panel_rect() -> Rect {
    Rect::new(940.0, 96.0, 320.0, 532.0)
}
