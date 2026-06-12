//! Level platform layout construction.

use crate::geometry::{FLOOR_Y, WORLD_HEIGHT};
use crate::state::{LevelPhase, MoralChoice};
use macroquad::prelude::*;

pub(crate) fn base_platforms(index: usize, width: f32, phase: LevelPhase) -> Vec<Rect> {
    let mut platforms = vec![Rect::new(0.0, FLOOR_Y, width, WORLD_HEIGHT - FLOOR_Y)];
    let ledges: Vec<Rect> = match index {
        0 => scrap_wake_platforms(phase),
        1 => cargo_carousel_platforms(width, phase),
        2 => suit_locker_platforms(width, phase),
        3 => mess_hall_platforms(width, phase),
        4 => med_bay_platforms(width, phase),
        5 => vent_stack_platforms(width, phase),
        6 => hydroponic_canopy_platforms(width, phase),
        7 => airlock_tether_platforms(width, phase),
        8 => cryo_thaw_platforms(width, phase),
        9 => badge_checkpoint_platforms(width, phase),
        10 => drone_bay_platforms(width, phase),
        11 => brig_door_platforms(width, phase),
        12 => observatory_shutter_platforms(width, phase),
        13 => armory_safety_platforms(width, phase),
        14 => laundry_tube_platforms(width, phase),
        15 => captains_quarters_platforms(width, phase),
        16 => reactor_foam_platforms(width, phase),
        18 => moral_firewall_platforms(width, phase),
        17 => core_lift_platforms(width, phase),
        19 => platform_row(width, &[(0.18, 410.0, 170.0), (0.62, 390.0, 170.0)]),
        _ => platform_row(
            width,
            &[
                (0.18, 430.0, 170.0),
                (0.45, 416.0, 170.0),
                (0.72, 416.0, 220.0),
            ],
        ),
    };
    platforms.extend(ledges);
    platforms
}

fn scrap_wake_platforms(phase: LevelPhase) -> Vec<Rect> {
    let right_route = match phase {
        LevelPhase::Resolved(MoralChoice::Savior) => Rect::new(520.0, 440.0, 230.0, 18.0),
        LevelPhase::Resolved(MoralChoice::Villain) => Rect::new(610.0, 430.0, 210.0, 18.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => Rect::new(640.0, 438.0, 190.0, 18.0),
    };
    vec![Rect::new(286.0, 452.0, 220.0, 18.0), right_route]
}

fn cargo_carousel_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let final_w = match phase {
        LevelPhase::Resolved(MoralChoice::Savior) => 306.0,
        LevelPhase::Resolved(MoralChoice::Villain) => 330.0,
        LevelPhase::AwaitingChoice | LevelPhase::Final => 286.0,
    };
    vec![
        Rect::new(318.0, 430.0, 240.0, 18.0),
        Rect::new(604.0, 414.0, 230.0, 18.0),
        Rect::new(width - final_w - 76.0, 414.0, final_w, 18.0),
    ]
}

fn suit_locker_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let center_y = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => 390.0,
        _ => 402.0,
    };
    vec![
        Rect::new(178.0, 446.0, 190.0, 18.0),
        Rect::new(528.0, center_y, 138.0, 18.0),
        Rect::new(width - 458.0, 444.0, 190.0, 18.0),
        Rect::new(width - 250.0, 388.0, 158.0, 18.0),
    ]
}

fn mess_hall_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let rail_y = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => 382.0,
        _ => 374.0,
    };
    vec![
        Rect::new(186.0, 456.0, 232.0, 18.0),
        Rect::new(512.0, 452.0, 278.0, 18.0),
        Rect::new(642.0, rail_y, 286.0, 18.0),
        Rect::new(width - 320.0, 430.0, 252.0, 18.0),
    ]
}

fn med_bay_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let recovery_y = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => 424.0,
        _ => 404.0,
    };
    vec![
        Rect::new(176.0, 452.0, 230.0, 18.0),
        Rect::new(512.0, 434.0, 252.0, 18.0),
        Rect::new(820.0, recovery_y, 280.0, 18.0),
        Rect::new(width - 392.0, recovery_y, 300.0, 18.0),
    ]
}

fn vent_stack_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (mid_y, upper_y, final_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (384.0, 366.0, 406.0),
        _ => (368.0, 344.0, 388.0),
    };
    vec![
        Rect::new(176.0, 446.0, 232.0, 18.0),
        Rect::new(486.0, 424.0, 184.0, 18.0),
        Rect::new(646.0, mid_y, 178.0, 18.0),
        Rect::new(838.0, upper_y, 230.0, 18.0),
        Rect::new(width - 396.0, final_y, 304.0, 18.0),
    ]
}

fn hydroponic_canopy_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (lower_y, canopy_y, pod_y, upper_y, exit_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 418.0, 380.0, 362.0, 410.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (452.0, 408.0, 382.0, 352.0, 374.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (458.0, 414.0, 388.0, 360.0, 386.0),
    };

    vec![
        Rect::new(170.0, lower_y, 228.0, 18.0),
        Rect::new(482.0, canopy_y, 196.0, 18.0),
        Rect::new(760.0, pod_y, 148.0, 18.0),
        Rect::new(1016.0, upper_y, 246.0, 18.0),
        Rect::new(width - 408.0, exit_y, 314.0, 18.0),
    ]
}

fn airlock_tether_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (chamber_a, exterior_a, chamber_b, exterior_b, final_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (452.0, 420.0, 388.0, 420.0, 392.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (444.0, 392.0, 438.0, 374.0, 404.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (450.0, 404.0, 438.0, 386.0, 410.0),
    };

    vec![
        Rect::new(214.0, chamber_a, 232.0, 18.0),
        Rect::new(560.0, exterior_a, 184.0, 18.0),
        Rect::new(850.0, chamber_b, 210.0, 18.0),
        Rect::new(1152.0, exterior_b, 222.0, 18.0),
        Rect::new(width - 372.0, final_y, 282.0, 18.0),
    ]
}

fn cryo_thaw_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (left_room, pipe_a, center_room, pipe_b, right_room) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 428.0, 456.0, 410.0, 438.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (448.0, 402.0, 448.0, 386.0, 414.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (454.0, 416.0, 454.0, 398.0, 426.0),
    };

    vec![
        Rect::new(220.0, left_room, 230.0, 18.0),
        Rect::new(548.0, pipe_a, 174.0, 18.0),
        Rect::new(842.0, center_room, 226.0, 18.0),
        Rect::new(1168.0, pipe_b, 196.0, 18.0),
        Rect::new(width - 384.0, right_room, 294.0, 18.0),
    ]
}

fn badge_checkpoint_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (queue_y, red_y, office_y, blue_y, bypass_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 430.0, 392.0, 430.0, 454.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (452.0, 420.0, 370.0, 412.0, 438.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (456.0, 424.0, 382.0, 420.0, 446.0),
    };

    vec![
        Rect::new(190.0, queue_y, 262.0, 18.0),
        Rect::new(550.0, red_y, 188.0, 18.0),
        Rect::new(820.0, office_y, 256.0, 18.0),
        Rect::new(1166.0, blue_y, 214.0, 18.0),
        Rect::new(width - 438.0, bypass_y, 346.0, 18.0),
    ]
}

fn drone_bay_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (pad_y, rail_a, tower_y, rail_b, hatch_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 422.0, 386.0, 414.0, 438.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (452.0, 404.0, 378.0, 396.0, 424.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (456.0, 416.0, 386.0, 408.0, 432.0),
    };

    vec![
        Rect::new(210.0, pad_y, 270.0, 18.0),
        Rect::new(574.0, rail_a, 210.0, 18.0),
        Rect::new(900.0, tower_y, 220.0, 18.0),
        Rect::new(1228.0, rail_b, 224.0, 18.0),
        Rect::new(width - 420.0, hatch_y, 328.0, 18.0),
    ]
}

fn brig_door_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (cell_y, vent_y, control_y, walk_y, exit_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 430.0, 398.0, 420.0, 456.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (448.0, 416.0, 384.0, 392.0, 408.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (454.0, 424.0, 394.0, 406.0, 428.0),
    };

    vec![
        Rect::new(220.0, cell_y, 250.0, 18.0),
        Rect::new(588.0, vent_y, 170.0, 18.0),
        Rect::new(846.0, control_y, 240.0, 18.0),
        Rect::new(1180.0, walk_y, 260.0, 18.0),
        Rect::new(width - 422.0, exit_y, 330.0, 18.0),
    ]
}

fn observatory_shutter_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (deck_a, shutter_a, gantry_y, shutter_b, lock_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 426.0, 390.0, 420.0, 436.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (450.0, 408.0, 372.0, 404.0, 416.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (454.0, 418.0, 384.0, 414.0, 426.0),
    };

    vec![
        Rect::new(218.0, deck_a, 300.0, 18.0),
        Rect::new(626.0, shutter_a, 224.0, 18.0),
        Rect::new(982.0, gantry_y, 260.0, 18.0),
        Rect::new(1360.0, shutter_b, 230.0, 18.0),
        Rect::new(width - 458.0, lock_y, 360.0, 18.0),
    ]
}

fn armory_safety_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (locker_y, foam_y, catwalk_a, turret_y, exit_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 436.0, 402.0, 420.0, 446.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (450.0, 418.0, 382.0, 398.0, 414.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (454.0, 428.0, 392.0, 408.0, 426.0),
    };

    vec![
        Rect::new(220.0, locker_y, 310.0, 18.0),
        Rect::new(660.0, foam_y, 250.0, 18.0),
        Rect::new(1000.0, catwalk_a, 246.0, 18.0),
        Rect::new(1372.0, turret_y, 260.0, 18.0),
        Rect::new(width - 438.0, exit_y, 342.0, 18.0),
    ]
}

fn laundry_tube_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (floor_y, tube_a, mid_y, upper_y, exit_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 428.0, 400.0, 416.0, 456.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (450.0, 410.0, 386.0, 392.0, 418.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (454.0, 420.0, 396.0, 404.0, 432.0),
    };

    vec![
        Rect::new(220.0, floor_y, 292.0, 18.0),
        Rect::new(620.0, tube_a, 230.0, 18.0),
        Rect::new(984.0, mid_y, 232.0, 18.0),
        Rect::new(1328.0, upper_y, 238.0, 18.0),
        Rect::new(width - 430.0, exit_y, 334.0, 18.0),
    ]
}

fn captains_quarters_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (trophy_y, office_y, safe_y, escape_y, briefing_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 426.0, 398.0, 416.0, 454.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (450.0, 408.0, 380.0, 392.0, 420.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (454.0, 418.0, 390.0, 404.0, 436.0),
    };

    vec![
        Rect::new(220.0, trophy_y, 304.0, 18.0),
        Rect::new(648.0, office_y, 244.0, 18.0),
        Rect::new(980.0, safe_y, 230.0, 18.0),
        Rect::new(1308.0, escape_y, 260.0, 18.0),
        Rect::new(width - 430.0, briefing_y, 334.0, 18.0),
    ]
}

fn reactor_foam_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (entry_y, pipe_y, valve_y, foam_y, walkway_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (388.0, 420.0, 448.0, 424.0, 456.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (374.0, 404.0, 438.0, 408.0, 450.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => (382.0, 414.0, 444.0, 418.0, 454.0),
    };

    vec![
        Rect::new(226.0, entry_y, 256.0, 18.0),
        Rect::new(620.0, pipe_y, 230.0, 18.0),
        Rect::new(960.0, valve_y, 230.0, 18.0),
        Rect::new(1290.0, foam_y, 240.0, 18.0),
        Rect::new(width - 430.0, walkway_y, 334.0, 18.0),
    ]
}

fn core_lift_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (entry_y, lower_mid, elevator_y, upper_mid, top_y, side_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 424.0, 392.0, 362.0, 334.0, 452.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (450.0, 410.0, 380.0, 346.0, 322.0, 432.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => {
            (454.0, 418.0, 388.0, 354.0, 330.0, 444.0)
        }
    };

    vec![
        Rect::new(210.0, entry_y, 280.0, 18.0),
        Rect::new(570.0, lower_mid, 240.0, 18.0),
        Rect::new(910.0, elevator_y, 220.0, 18.0),
        Rect::new(1250.0, upper_mid, 250.0, 18.0),
        Rect::new(width - 500.0, top_y, 340.0, 18.0),
        Rect::new(width - 420.0, side_y, 320.0, 18.0),
    ]
}

fn moral_firewall_platforms(width: f32, phase: LevelPhase) -> Vec<Rect> {
    let (archive_y, door_a, truth_y, door_b, propaganda_y, seal_y) = match phase {
        LevelPhase::Resolved(MoralChoice::Villain) => (456.0, 430.0, 386.0, 418.0, 456.0, 444.0),
        LevelPhase::Resolved(MoralChoice::Savior) => (450.0, 412.0, 366.0, 398.0, 438.0, 404.0),
        LevelPhase::AwaitingChoice | LevelPhase::Final => {
            (454.0, 422.0, 378.0, 408.0, 448.0, 420.0)
        }
    };

    vec![
        Rect::new(210.0, archive_y, 310.0, 18.0),
        Rect::new(610.0, door_a, 230.0, 18.0),
        Rect::new(950.0, truth_y, 260.0, 18.0),
        Rect::new(1300.0, door_b, 250.0, 18.0),
        Rect::new(1640.0, propaganda_y, 260.0, 18.0),
        Rect::new(width - 470.0, seal_y, 380.0, 18.0),
    ]
}

fn platform_row(width: f32, specs: &[(f32, f32, f32)]) -> Vec<Rect> {
    specs
        .iter()
        .map(|(ratio, y, w)| Rect::new((width * ratio).min(width - w - 80.0), *y, *w, 18.0))
        .collect()
}
