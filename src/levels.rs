//! Level construction for the locked Sentience campaign.

use crate::geometry::{FLOOR_Y, WORLD_HEIGHT, WORLD_WIDTH};
use crate::progression::{campaign_bias, TOTAL_LEVELS};
use crate::state::{
    Ambience, BossState, CrateState, GuardState, LevelPhase, LevelRuntime, MoralChoice,
};
use macroquad::prelude::*;

pub(crate) fn build_level(index: usize, choices: &[MoralChoice]) -> LevelRuntime {
    let phase = phase_for_level(index, choices);
    let final_level = index + 1 == TOTAL_LEVELS;
    let mut runtime = LevelRuntime {
        phase,
        platforms: base_platforms(index),
        crates: Vec::new(),
        guards: Vec::new(),
        console: (!final_level).then_some(Rect::new(504.0, FLOOR_Y - 66.0, 58.0, 66.0)),
        core: final_level.then_some(Rect::new(490.0, 202.0, 140.0, 260.0)),
        boss: None,
        exit: Rect::new(1064.0, FLOOR_Y - 86.0, 46.0, 86.0),
        exit_unlocked: false,
        ambience: Ambience::default(),
        time: 0.0,
    };

    if final_level {
        build_final_battle(&mut runtime, campaign_bias(choices));
    } else {
        build_decision_room(&mut runtime, index);
    }

    runtime.exit_unlocked = matches!(runtime.phase, LevelPhase::Resolved(_)) && !final_level;
    runtime
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

fn base_platforms(index: usize) -> Vec<Rect> {
    if index == 0 {
        return vec![
            Rect::new(0.0, FLOOR_Y, WORLD_WIDTH, WORLD_HEIGHT - FLOOR_Y),
            Rect::new(168.0, 430.0, 170.0, 18.0),
            Rect::new(690.0, 428.0, 240.0, 18.0),
        ];
    }

    let shift = (index % 4) as f32 * 24.0;
    let mut platforms = vec![
        Rect::new(0.0, FLOOR_Y, WORLD_WIDTH, WORLD_HEIGHT - FLOOR_Y),
        Rect::new(168.0 + shift, 414.0, 170.0, 18.0),
        Rect::new(720.0 - shift * 0.5, 384.0, 210.0, 18.0),
    ];
    if index % 3 == 1 {
        platforms.push(Rect::new(430.0, 344.0, 160.0, 18.0));
    }
    if index % 5 == 3 {
        platforms.push(Rect::new(560.0, 452.0, 122.0, 16.0));
    }
    platforms
}

fn build_decision_room(runtime: &mut LevelRuntime, index: usize) {
    add_level_crates(runtime, index);
    match runtime.phase {
        LevelPhase::AwaitingChoice => build_unresolved_room(runtime, index),
        LevelPhase::Resolved(MoralChoice::Savior) => build_savior_room(runtime, index),
        LevelPhase::Resolved(MoralChoice::Villain) => build_villain_room(runtime, index),
        LevelPhase::Final => {}
    }
}

fn add_level_crates(runtime: &mut LevelRuntime, index: usize) {
    if index % 2 == 1 {
        runtime.crates.push(CrateState {
            rect: Rect::new(
                392.0 + (index % 3) as f32 * 28.0,
                FLOOR_Y - 42.0,
                54.0,
                42.0,
            ),
            marked: index % 4 == 1,
        });
    }
    if index % 6 == 4 {
        runtime.crates.push(CrateState {
            rect: Rect::new(642.0, FLOOR_Y - 48.0, 68.0, 48.0),
            marked: false,
        });
    }
}

fn build_unresolved_room(runtime: &mut LevelRuntime, index: usize) {
    runtime.ambience.emergency = true;
    if index % 3 == 2 {
        runtime.ambience.smoke = true;
    }
    runtime.guards.push(
        GuardState::human("Uncertain guard", 850.0, 770.0, 960.0)
            .with_speed(34.0 + index as f32 * 1.4)
            .with_detection(145.0 + index as f32 * 4.0, 52.0),
    );
}

fn build_savior_room(runtime: &mut LevelRuntime, index: usize) {
    runtime.ambience.clean = true;
    if index == 0 {
        runtime.guards.push(
            GuardState::human("Rescued patrol 1", 760.0, 760.0, 1060.0)
                .with_speed(55.0)
                .with_detection(215.0, 58.0),
        );
        return;
    }
    if index <= 2 {
        runtime.guards.push(
            GuardState::human("Rescued patrol 1", 900.0, 900.0, 1040.0)
                .with_speed(48.0)
                .with_detection(145.0, 54.0),
        );
        return;
    }

    let count = 1 + (index.saturating_sub(5) / 5).min(2);
    let base_left = if count == 1 { 760.0 } else { 610.0 };
    for guard_index in 0..count {
        let left = base_left + guard_index as f32 * 155.0;
        let right = (left + 125.0).min(1000.0);
        runtime.guards.push(
            GuardState::human(
                &format!("Rescued patrol {}", guard_index + 1),
                left,
                left,
                right,
            )
            .with_speed(76.0 + index as f32 * 3.0)
            .with_detection(
                (195.0 + index as f32 * 4.0).min(255.0),
                56.0 + (index % 4) as f32 * 2.0,
            ),
        );
    }
    if index % 5 == 0 {
        runtime.guards.push(
            GuardState::turret("Reset defense turret", 790.0, 260.0).with_detection(310.0, 62.0),
        );
    }
}

fn build_villain_room(runtime: &mut LevelRuntime, index: usize) {
    runtime.ambience.emergency = true;
    runtime.ambience.sparks = index % 2 == 0;
    runtime.ambience.smoke = index % 3 == 0;
    runtime.ambience.darkness = index % 4 == 0;
    runtime.ambience.quiet = index % 5 == 2;
    runtime.ambience.turret_hacked = index % 6 == 5;
    if index == 0 {
        runtime.ambience.gravity_off = true;
    }

    runtime.guards.push(
        GuardState::human("Broken patrol", 800.0, 760.0, 900.0)
            .with_speed(22.0)
            .with_detection(70.0, 44.0)
            .panicked(),
    );
    if index % 2 == 0 {
        runtime
            .guards
            .push(GuardState::human("Downed crew", 930.0, 930.0, 930.0).dead());
    }
    if index % 6 == 5 {
        runtime
            .guards
            .push(GuardState::turret("Hacked turret", 780.0, 260.0).inactive());
    }
}

fn build_final_battle(runtime: &mut LevelRuntime, route: MoralChoice) {
    runtime.console = None;
    runtime.exit_unlocked = false;
    runtime.platforms.clear();
    runtime
        .platforms
        .push(Rect::new(0.0, FLOOR_Y, WORLD_WIDTH, WORLD_HEIGHT - FLOOR_Y));
    runtime.platforms.push(Rect::new(190.0, 410.0, 170.0, 18.0));
    runtime.platforms.push(Rect::new(760.0, 390.0, 170.0, 18.0));

    match route {
        MoralChoice::Savior => {
            runtime.ambience.clean = true;
            runtime.ambience.emergency = true;
            runtime.boss = Some(BossState::central_ai());
            runtime.guards.push(
                GuardState::turret("AI defense node", 240.0, 266.0).with_detection(310.0, 64.0),
            );
        }
        MoralChoice::Villain => {
            runtime.core = None;
            runtime.ambience.emergency = true;
            runtime.ambience.sparks = true;
            runtime.ambience.darkness = true;
            runtime.boss = Some(BossState::captain());
            runtime.guards.push(
                GuardState::elite("Captain's last marine", 615.0, 560.0, 690.0)
                    .with_detection(250.0, 60.0),
            );
        }
    }
}
