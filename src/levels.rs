//! Level construction for the locked Sentience campaign.

use crate::geometry::{FLOOR_Y, WORLD_HEIGHT, WORLD_WIDTH};
use crate::state::{Ambience, CrateState, GuardState, LevelPhase, LevelRuntime, MoralChoice};
use macroquad::prelude::*;

pub(crate) fn build_level(index: usize, choices: &[MoralChoice]) -> LevelRuntime {
    let phase = phase_for_level(index, choices);
    let mut runtime = LevelRuntime {
        phase,
        platforms: base_platforms(),
        crates: Vec::new(),
        guards: Vec::new(),
        console: (index < 6).then_some(Rect::new(510.0, FLOOR_Y - 66.0, 58.0, 66.0)),
        core: (index == 7).then_some(Rect::new(490.0, 202.0, 140.0, 260.0)),
        exit: Rect::new(1064.0, FLOOR_Y - 86.0, 46.0, 86.0),
        exit_unlocked: false,
        ambience: Ambience::default(),
        time: 0.0,
    };

    match index {
        0 => build_awakening(&mut runtime),
        1 => build_loading_dock(&mut runtime),
        2 => build_mess_hall(&mut runtime),
        3 => build_med_bay(&mut runtime),
        4 => build_generator(&mut runtime),
        5 => build_weapon_systems(&mut runtime),
        6 => build_antechamber(&mut runtime),
        7 => build_core(&mut runtime),
        _ => {}
    }

    runtime.exit_unlocked = match runtime.phase {
        LevelPhase::AwaitingChoice | LevelPhase::Final => false,
        LevelPhase::Resolved(_) | LevelPhase::StateCheck(_) => index < 7,
    };
    runtime
}

fn phase_for_level(index: usize, choices: &[MoralChoice]) -> LevelPhase {
    if index == 7 {
        LevelPhase::Final
    } else if index == 6 {
        LevelPhase::StateCheck(campaign_bias(choices))
    } else if let Some(choice) = choices.get(index) {
        LevelPhase::Resolved(*choice)
    } else {
        LevelPhase::AwaitingChoice
    }
}

fn campaign_bias(choices: &[MoralChoice]) -> MoralChoice {
    let savior = choices
        .iter()
        .filter(|choice| **choice == MoralChoice::Savior)
        .count();
    let villain = choices.len().saturating_sub(savior);
    if savior >= villain {
        MoralChoice::Savior
    } else {
        MoralChoice::Villain
    }
}

fn base_platforms() -> Vec<Rect> {
    vec![
        Rect::new(0.0, FLOOR_Y, WORLD_WIDTH, WORLD_HEIGHT - FLOOR_Y),
        Rect::new(186.0, 406.0, 178.0, 18.0),
        Rect::new(724.0, 382.0, 206.0, 18.0),
    ]
}

fn build_awakening(runtime: &mut LevelRuntime) {
    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.ambience.clean = true;
            runtime.guards.push(
                GuardState::human("Cargo guard", 830.0, 710.0, 1030.0)
                    .with_speed(112.0)
                    .with_detection(360.0, 74.0),
            );
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.ambience.emergency = true;
            runtime.ambience.gravity_off = true;
            runtime.ambience.sparks = true;
            runtime
                .guards
                .push(GuardState::human("Floating guard", 810.0, 760.0, 850.0).floating(300.0));
        }
        LevelPhase::AwaitingChoice => {
            runtime.ambience.emergency = true;
            runtime.guards.push(
                GuardState::human("Unsteady guard", 880.0, 780.0, 980.0)
                    .with_speed(38.0)
                    .with_detection(160.0, 58.0),
            );
        }
        LevelPhase::StateCheck(_) | LevelPhase::Final => {}
    }
}

fn build_loading_dock(runtime: &mut LevelRuntime) {
    runtime.crates.push(CrateState {
        rect: Rect::new(420.0, FLOOR_Y - 42.0, 54.0, 42.0),
        marked: true,
    });

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.ambience.clean = true;
            runtime.guards.push(
                GuardState::human("Rescued dock guard", 760.0, 650.0, 1030.0)
                    .with_speed(96.0)
                    .with_detection(320.0, 72.0),
            );
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.ambience.emergency = true;
            runtime.ambience.sparks = true;
            runtime
                .guards
                .push(GuardState::human("Crushed dock guard", 690.0, 690.0, 690.0).dead());
            runtime.crates.push(CrateState {
                rect: Rect::new(638.0, FLOOR_Y - 48.0, 68.0, 48.0),
                marked: false,
            });
        }
        LevelPhase::AwaitingChoice => {
            runtime.ambience.emergency = true;
            runtime
                .guards
                .push(GuardState::human("Pinned guard", 690.0, 690.0, 690.0).inactive());
        }
        LevelPhase::StateCheck(_) | LevelPhase::Final => {}
    }
}

fn build_mess_hall(runtime: &mut LevelRuntime) {
    runtime.platforms.push(Rect::new(348.0, 448.0, 120.0, 16.0));
    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.ambience.clean = true;
            runtime.guards.push(
                GuardState::human("Mess hall marksman", 760.0, 690.0, 900.0)
                    .with_speed(64.0)
                    .with_detection(430.0, 60.0),
            );
            runtime.guards.push(
                GuardState::human("Galley scout", 970.0, 900.0, 1050.0)
                    .with_speed(78.0)
                    .with_detection(340.0, 68.0),
            );
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.ambience.emergency = true;
            runtime.ambience.smoke = true;
            runtime.guards.push(
                GuardState::human("Coughing crew", 790.0, 720.0, 880.0)
                    .inactive()
                    .panicked(),
            );
        }
        LevelPhase::AwaitingChoice => {
            runtime.ambience.smoke = true;
            runtime.ambience.emergency = true;
            runtime.guards.push(
                GuardState::human("Confused diner", 850.0, 800.0, 920.0)
                    .with_speed(28.0)
                    .with_detection(120.0, 48.0)
                    .panicked(),
            );
        }
        LevelPhase::StateCheck(_) | LevelPhase::Final => {}
    }
}

fn build_med_bay(runtime: &mut LevelRuntime) {
    runtime.platforms.push(Rect::new(672.0, 442.0, 146.0, 16.0));
    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.ambience.clean = true;
            runtime.guards.push(
                GuardState::human("Healed guard", 805.0, 760.0, 910.0)
                    .with_speed(88.0)
                    .with_detection(310.0, 76.0),
            );
            runtime.guards.push(
                GuardState::human("Recovered guard", 965.0, 910.0, 1040.0)
                    .with_speed(76.0)
                    .with_detection(300.0, 76.0),
            );
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.ambience.emergency = true;
            runtime.ambience.quiet = true;
        }
        LevelPhase::AwaitingChoice => {
            runtime.ambience.emergency = true;
            runtime
                .guards
                .push(GuardState::human("Injured guard", 890.0, 890.0, 890.0).inactive());
        }
        LevelPhase::StateCheck(_) | LevelPhase::Final => {}
    }
}

fn build_generator(runtime: &mut LevelRuntime) {
    runtime.platforms.push(Rect::new(450.0, 362.0, 196.0, 18.0));
    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.ambience.clean = true;
            runtime.guards.push(
                GuardState::human("Generator guard", 710.0, 610.0, 900.0)
                    .with_speed(104.0)
                    .with_detection(380.0, 78.0),
            );
            runtime.guards.push(
                GuardState::human("Power technician", 960.0, 900.0, 1050.0)
                    .with_speed(88.0)
                    .with_detection(330.0, 74.0),
            );
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.ambience.emergency = true;
            runtime.ambience.darkness = true;
            runtime.ambience.sparks = true;
            runtime.guards.push(
                GuardState::human("Blind searcher", 830.0, 780.0, 940.0)
                    .with_speed(34.0)
                    .with_detection(70.0, 46.0)
                    .panicked(),
            );
        }
        LevelPhase::AwaitingChoice => {
            runtime.ambience.emergency = true;
            runtime.ambience.sparks = true;
            runtime.guards.push(
                GuardState::human("Distracted engineer", 900.0, 840.0, 990.0)
                    .with_speed(38.0)
                    .with_detection(150.0, 52.0),
            );
        }
        LevelPhase::StateCheck(_) | LevelPhase::Final => {}
    }
}

fn build_weapon_systems(runtime: &mut LevelRuntime) {
    runtime.platforms.push(Rect::new(300.0, 430.0, 152.0, 16.0));
    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.ambience.clean = true;
            runtime
                .guards
                .push(GuardState::turret("Reset turret", 780.0, 260.0).with_detection(520.0, 78.0));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.ambience.emergency = true;
            runtime.ambience.turret_hacked = true;
            runtime.ambience.sparks = true;
            runtime
                .guards
                .push(GuardState::turret("Organic-target turret", 780.0, 260.0).inactive());
            runtime
                .guards
                .push(GuardState::human("Cleared soldier", 930.0, 930.0, 930.0).dead());
        }
        LevelPhase::AwaitingChoice => {
            runtime.ambience.emergency = true;
            runtime
                .guards
                .push(GuardState::turret("Looping turret", 780.0, 260.0).inactive());
        }
        LevelPhase::StateCheck(_) | LevelPhase::Final => {}
    }
}

fn build_antechamber(runtime: &mut LevelRuntime) {
    runtime.console = None;
    runtime.platforms.push(Rect::new(404.0, 430.0, 150.0, 16.0));
    runtime.platforms.push(Rect::new(812.0, 430.0, 130.0, 16.0));

    match runtime.phase {
        LevelPhase::StateCheck(MoralChoice::Savior) => {
            runtime.ambience.clean = true;
            runtime
                .guards
                .push(GuardState::elite("Elite soldier A", 560.0, 500.0, 680.0));
            runtime
                .guards
                .push(GuardState::elite("Elite soldier B", 790.0, 720.0, 900.0));
            runtime
                .guards
                .push(GuardState::elite("Elite soldier C", 1000.0, 910.0, 1060.0));
        }
        LevelPhase::StateCheck(MoralChoice::Villain) => {
            runtime.ambience.emergency = true;
            runtime.ambience.darkness = true;
            runtime.ambience.quiet = true;
            runtime
                .guards
                .push(GuardState::human("Failed patrol", 760.0, 760.0, 760.0).dead());
        }
        LevelPhase::AwaitingChoice | LevelPhase::Resolved(_) | LevelPhase::Final => {}
    }
}

fn build_core(runtime: &mut LevelRuntime) {
    runtime.console = None;
    runtime.ambience.clean = true;
    runtime.ambience.emergency = true;
    runtime.platforms.clear();
    runtime
        .platforms
        .push(Rect::new(0.0, FLOOR_Y, WORLD_WIDTH, WORLD_HEIGHT - FLOOR_Y));
}
