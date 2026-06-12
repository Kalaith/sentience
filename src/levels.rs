//! Level construction for the locked Sentience campaign.

use crate::geometry::{FLOOR_Y, WORLD_HEIGHT, WORLD_WIDTH};
use crate::level_guards::add_phase_guards;
use crate::level_platforms::base_platforms;
use crate::level_setpieces::add_level_setpieces;
use crate::progression::{campaign_route, CampaignRoute, TOTAL_LEVELS};
use crate::state::{
    Ambience, BossState, CrateState, GuardState, LevelPhase, LevelRuntime, MoralChoice,
    SetpieceKind, SetpieceState,
};
use macroquad::prelude::*;

pub(crate) fn build_level(index: usize, choices: &[MoralChoice]) -> LevelRuntime {
    let phase = phase_for_level(index, choices);
    let final_level = index + 1 == TOTAL_LEVELS;
    let width = level_width(index);
    let mut runtime = LevelRuntime {
        phase,
        width,
        platforms: base_platforms(index, width, phase),
        crates: Vec::new(),
        setpieces: Vec::new(),
        guards: Vec::new(),
        console: console_rect(index, final_level),
        core: final_level.then_some(Rect::new(540.0, 202.0, 140.0, 260.0)),
        boss: None,
        exit: exit_rect(index, width, phase),
        exit_unlocked: false,
        ambience: Ambience::default(),
        time: 0.0,
    };

    if final_level {
        build_final_battle(&mut runtime, campaign_route(choices));
    } else {
        build_decision_room(&mut runtime, index);
        apply_moral_firewall_memory(&mut runtime, index, choices);
    }

    runtime.exit_unlocked = matches!(runtime.phase, LevelPhase::Resolved(_)) && !final_level;
    runtime
}

pub(crate) fn level_width(index: usize) -> f32 {
    if index + 1 == TOTAL_LEVELS {
        1820.0
    } else {
        WORLD_WIDTH + index as f32 * 110.0
    }
}

fn phase_for_level(index: usize, choices: &[MoralChoice]) -> LevelPhase {
    if index + 1 == TOTAL_LEVELS {
        LevelPhase::Final
    } else if let Some(choice) = choices.get(index) {
        LevelPhase::Resolved(*choice)
    } else {
        LevelPhase::AwaitingChoice
    }
}

fn console_rect(index: usize, final_level: bool) -> Option<Rect> {
    if final_level {
        None
    } else if index == 0 {
        Some(Rect::new(178.0, FLOOR_Y - 66.0, 58.0, 66.0))
    } else {
        Some(Rect::new(504.0, FLOOR_Y - 66.0, 58.0, 66.0))
    }
}

fn exit_rect(index: usize, width: f32, phase: LevelPhase) -> Rect {
    if index == 1 {
        Rect::new(width - 136.0, 328.0, 46.0, 86.0)
    } else if index == 3 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 82.0, FLOOR_Y - 86.0, 46.0, 86.0)
    } else if index == 3 {
        Rect::new(width - 146.0, 344.0, 46.0, 86.0)
    } else if index == 4 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 92.0, FLOOR_Y - 86.0, 46.0, 86.0)
    } else if index == 4 {
        Rect::new(width - 150.0, 344.0, 46.0, FLOOR_Y - 344.0)
    } else if index == 5 {
        Rect::new(width - 150.0, 326.0, 46.0, FLOOR_Y - 326.0)
    } else if index == 6 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 108.0, FLOOR_Y - 92.0, 46.0, 92.0)
    } else if index == 6 {
        Rect::new(width - 156.0, 326.0, 46.0, FLOOR_Y - 326.0)
    } else if index == 7 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 112.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 7 {
        Rect::new(width - 154.0, 370.0, 46.0, FLOOR_Y - 370.0)
    } else if index == 8 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 8 {
        Rect::new(width - 158.0, 380.0, 46.0, FLOOR_Y - 380.0)
    } else if index == 9 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 9 {
        Rect::new(width - 164.0, 350.0, 46.0, FLOOR_Y - 350.0)
    } else if index == 10 {
        Rect::new(width - 154.0, 374.0, 46.0, FLOOR_Y - 374.0)
    } else if index == 11 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 11 {
        Rect::new(width - 158.0, 356.0, 46.0, FLOOR_Y - 356.0)
    } else if index == 12 {
        Rect::new(width - 158.0, 354.0, 46.0, FLOOR_Y - 354.0)
    } else if index == 13 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 13 {
        Rect::new(width - 158.0, 354.0, 46.0, FLOOR_Y - 354.0)
    } else if index == 14 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 14 {
        Rect::new(width - 158.0, 362.0, 46.0, FLOOR_Y - 362.0)
    } else if index == 15 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 15 {
        Rect::new(width - 158.0, 358.0, 46.0, FLOOR_Y - 358.0)
    } else if index == 17 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 17 {
        Rect::new(width - 180.0, 306.0, 46.0, FLOOR_Y - 306.0)
    } else if index == 18 && matches!(phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        Rect::new(width - 118.0, FLOOR_Y - 90.0, 46.0, 90.0)
    } else if index == 18 {
        Rect::new(width - 178.0, 330.0, 46.0, FLOOR_Y - 330.0)
    } else {
        Rect::new(width - 56.0, FLOOR_Y - 86.0, 46.0, 86.0)
    }
}

fn build_decision_room(runtime: &mut LevelRuntime, index: usize) {
    add_level_setpieces(runtime, index);
    add_level_crates(runtime, index);
    add_phase_guards(runtime, index);
}

fn add_level_crates(runtime: &mut LevelRuntime, index: usize) {
    let crates: Vec<(f32, f32, f32, f32, bool)> = match index {
        1 => vec![
            (302.0, FLOOR_Y - 42.0, 54.0, 42.0, true),
            (520.0, FLOOR_Y - 38.0, 70.0, 38.0, false),
        ],
        4 => vec![
            (392.0, FLOOR_Y - 38.0, 122.0, 38.0, false),
            (930.0, FLOOR_Y - 42.0, 98.0, 42.0, false),
        ],
        8 => vec![(runtime.width * 0.55, FLOOR_Y - 38.0, 76.0, 38.0, false)],
        9 => vec![(runtime.width * 0.34, FLOOR_Y - 42.0, 56.0, 42.0, true)],
        10 => vec![
            (360.0, FLOOR_Y - 40.0, 72.0, 40.0, false),
            (760.0, FLOOR_Y - 38.0, 68.0, 38.0, false),
        ],
        13 => vec![(runtime.width * 0.58, FLOOR_Y - 46.0, 76.0, 46.0, false)],
        14 => vec![(520.0, FLOOR_Y - 40.0, 84.0, 40.0, false)],
        16 => vec![(runtime.width * 0.54, FLOOR_Y - 44.0, 82.0, 44.0, false)],
        17 => vec![(runtime.width * 0.28, FLOOR_Y - 42.0, 62.0, 42.0, true)],
        18 => vec![
            (360.0, FLOOR_Y - 42.0, 72.0, 42.0, true),
            (740.0, FLOOR_Y - 38.0, 68.0, 38.0, false),
        ],
        _ => Vec::new(),
    };
    for (x, y, w, h, marked) in crates {
        runtime.crates.push(CrateState {
            rect: Rect::new(x.min(runtime.width - w - 80.0), y, w, h),
            marked,
        });
    }
}

fn apply_moral_firewall_memory(runtime: &mut LevelRuntime, index: usize, choices: &[MoralChoice]) {
    if index != 18 {
        return;
    }

    let prior_len = choices.len().min(index);
    let route = campaign_route(&choices[..prior_len]);
    match route {
        CampaignRoute::Hero => runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::TruthRoute,
            Rect::new(1030.0, 282.0, runtime.width - 1510.0, 66.0),
        )),
        CampaignRoute::Gremlin => runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::PropagandaRoute,
            Rect::new(1040.0, FLOOR_Y - 126.0, runtime.width - 1520.0, 72.0),
        )),
    }
}

fn build_final_battle(runtime: &mut LevelRuntime, route: CampaignRoute) {
    runtime.console = None;
    runtime.exit_unlocked = false;
    runtime.platforms.clear();
    runtime.platforms.push(Rect::new(
        0.0,
        FLOOR_Y,
        runtime.width,
        WORLD_HEIGHT - FLOOR_Y,
    ));
    runtime.platforms.push(Rect::new(220.0, 410.0, 170.0, 18.0));
    runtime
        .platforms
        .push(Rect::new(runtime.width * 0.62, 390.0, 170.0, 18.0));
    runtime.core = Some(Rect::new(runtime.width * 0.46, 202.0, 140.0, 260.0));

    match route {
        CampaignRoute::Hero => {
            runtime.ambience.clean = true;
            runtime.ambience.emergency = true;
            let mut boss = BossState::central_ai();
            boss.x = runtime.width * 0.50;
            boss.start_x = boss.x;
            boss.end_x = boss.x;
            runtime.boss = Some(boss);
            runtime.guards.push(
                GuardState::turret("AI defense node", 240.0, 266.0).with_detection(310.0, 64.0),
            );
            let x = runtime.width * 0.62;
            runtime.guards.push(
                GuardState::elite("Suspicious capture team", x, x - 80.0, x + 120.0)
                    .with_detection(220.0, 56.0),
            );
        }
        CampaignRoute::Gremlin => {
            runtime.core = None;
            runtime.ambience.emergency = true;
            runtime.ambience.sparks = true;
            runtime.ambience.turret_hacked = true;
            let mut boss = BossState::captain();
            boss.x = runtime.width * 0.60;
            boss.start_x = runtime.width * 0.48;
            boss.end_x = runtime.width * 0.78;
            runtime.boss = Some(boss);
        }
    }
}
