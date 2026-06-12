//! Level-specific NPC, device, and route-pressure construction.

use crate::geometry::FLOOR_Y;
use crate::level_guards_late::{try_savior_late, try_unresolved_late, try_villain_late};
use crate::state::{CrateState, GuardState, LevelPhase, LevelRuntime, MoralChoice};
use macroquad::prelude::*;

pub(crate) fn add_phase_guards(runtime: &mut LevelRuntime, index: usize) {
    match runtime.phase {
        LevelPhase::AwaitingChoice => build_unresolved_room(runtime, index),
        LevelPhase::Resolved(MoralChoice::Savior) => build_savior_room(runtime, index),
        LevelPhase::Resolved(MoralChoice::Villain) => build_villain_room(runtime, index),
        LevelPhase::Final => {}
    }
}

fn build_unresolved_room(runtime: &mut LevelRuntime, index: usize) {
    runtime.ambience.emergency = true;
    if index == 0 {
        runtime.guards.push(
            GuardState::human("Booted cargo guard", 760.0, 705.0, 870.0)
                .with_speed(34.0)
                .with_detection(132.0, 46.0),
        );
        return;
    }
    if index == 1 {
        runtime.guards.push(
            GuardState::human("Loader guard", 650.0, 585.0, 730.0)
                .with_speed(42.0)
                .with_detection(138.0, 50.0),
        );
        return;
    }
    if index == 2 {
        runtime.ambience.smoke = true;
        runtime.guards.push(
            GuardState::human("Stuck suit technician", runtime.width - 350.0, 0.0, 0.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        runtime.guards.push(
            GuardState::human("Shower-shaft security", 660.0, 588.0, 738.0)
                .with_speed(38.0)
                .with_detection(122.0, 48.0),
        );
        return;
    }
    if index == 3 {
        runtime.guards.push(
            GuardState::human("Panicked diner", 340.0, 278.0, 412.0)
                .with_speed(24.0)
                .with_detection(64.0, 40.0)
                .panicked(),
        );
        runtime.guards.push(
            GuardState::human("Lost cafeteria crew", 720.0, 650.0, 790.0)
                .with_speed(22.0)
                .with_detection(58.0, 38.0)
                .panicked(),
        );
        runtime.guards.push(
            GuardState::human("Kitchen pass guard", 505.0, 448.0, 564.0)
                .with_speed(42.0)
                .with_detection(118.0, 46.0),
        );
        return;
    }
    if index == 4 {
        runtime.guards.push(
            GuardState::human("Injured bed guard", 566.0, 0.0, 0.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        runtime
            .guards
            .push(GuardState::turret("Medic drone", 678.0, 294.0).with_detection(96.0, 42.0));
        return;
    }
    if index == 5 {
        runtime.ambience.smoke = true;
        runtime.guards.push(
            GuardState::human("Lower fan engineer", 292.0, 238.0, 358.0)
                .with_speed(26.0)
                .with_detection(62.0, 38.0)
                .panicked(),
        );
        runtime.guards.push(
            GuardState::human("Upper grate guard", 934.0, 862.0, 1038.0)
                .with_y(344.0)
                .with_speed(32.0)
                .with_detection(76.0, 40.0),
        );
        return;
    }
    if index == 6 {
        runtime.guards.push(
            GuardState::human("Alert botanist 1", 312.0, 262.0, 374.0)
                .with_speed(24.0)
                .with_detection(58.0, 36.0)
                .panicked(),
        );
        runtime.guards.push(
            GuardState::human("Alert botanist 2", 642.0, 590.0, 704.0)
                .with_y(414.0)
                .with_speed(22.0)
                .with_detection(56.0, 36.0)
                .panicked(),
        );
        runtime.guards.push(
            GuardState::human("Upper garden guard", 1138.0, 1044.0, 1236.0)
                .with_y(360.0)
                .with_speed(36.0)
                .with_detection(88.0, 40.0),
        );
        return;
    }
    if index == 7 {
        runtime.guards.push(
            GuardState::human("Suited airlock guard", 610.0, 552.0, 704.0)
                .with_speed(30.0)
                .with_detection(78.0, 38.0),
        );
        return;
    }
    if try_unresolved_late(runtime, index) {
        return;
    }
    if matches!(index, 2 | 5 | 8 | 11 | 17) {
        runtime.ambience.smoke = true;
    }
    let x = (runtime.width * 0.72).min(runtime.width - 160.0);
    runtime.guards.push(
        GuardState::human("Uncertain guard", x, x - 80.0, x + 110.0)
            .with_speed(34.0 + index as f32 * 1.4)
            .with_detection(145.0 + index as f32 * 4.0, 52.0),
    );
}

fn build_savior_room(runtime: &mut LevelRuntime, index: usize) {
    runtime.ambience.clean = true;
    if index == 0 {
        runtime.guards.push(
            GuardState::human("Boot-free guard", 760.0, 760.0, 1110.0)
                .with_speed(58.0)
                .with_detection(168.0, 50.0),
        );
        return;
    }
    if index == 1 {
        let width = runtime.width;
        runtime.guards.push(
            GuardState::human(
                "Lower dock loader",
                width * 0.62,
                width * 0.55,
                width * 0.72,
            )
            .with_speed(48.0)
            .with_detection(105.0, 48.0),
        );
        runtime.guards.push(
            GuardState::human(
                "Upper supply scout",
                width * 0.40,
                width * 0.34,
                width * 0.48,
            )
            .with_y(414.0)
            .with_speed(38.0)
            .with_detection(32.0, 34.0),
        );
        runtime.guards.push(
            GuardState::human("Forklift spotter", width * 0.78, width * 0.72, width * 0.84)
                .with_y(414.0)
                .with_speed(42.0)
                .with_detection(36.0, 34.0),
        );
        return;
    }
    if index == 2 {
        let rack_a = runtime.width - 360.0;
        let rack_b = runtime.width - 170.0;
        runtime.guards.push(
            GuardState::human("Unsuited rack crew", rack_a, rack_a - 54.0, rack_a + 70.0)
                .with_y(444.0)
                .with_speed(34.0)
                .with_detection(62.0, 38.0),
        );
        runtime.guards.push(
            GuardState::human("Upper suit climber", rack_b, rack_b - 58.0, rack_b + 50.0)
                .with_y(388.0)
                .with_speed(32.0)
                .with_detection(58.0, 36.0),
        );
        runtime.guards.push(
            GuardState::human("Decon security guard", 690.0, 614.0, 780.0)
                .with_speed(52.0)
                .with_detection(118.0, 46.0),
        );
        return;
    }
    if index == 3 {
        runtime.guards.push(
            GuardState::human("Organized kitchen guard", 505.0, 448.0, 584.0)
                .with_speed(54.0)
                .with_detection(122.0, 48.0),
        );
        runtime.guards.push(
            GuardState::human("Dining floor guard", 832.0, 770.0, 928.0)
                .with_speed(58.0)
                .with_detection(120.0, 48.0),
        );
        runtime.guards.push(
            GuardState::human(
                "Evacuation civilian",
                runtime.width - 272.0,
                runtime.width - 334.0,
                runtime.width - 188.0,
            )
            .with_speed(30.0)
            .with_detection(44.0, 34.0)
            .panicked(),
        );
        runtime.guards.push(
            GuardState::human(
                "Dish return civilian",
                runtime.width - 398.0,
                runtime.width - 458.0,
                runtime.width - 312.0,
            )
            .with_y(430.0)
            .with_speed(28.0)
            .with_detection(38.0, 34.0)
            .panicked(),
        );
        return;
    }
    if index == 4 {
        runtime.guards.push(
            GuardState::human("Revived security patient", 650.0, 586.0, 746.0)
                .with_speed(44.0)
                .with_detection(8.0, 12.0),
        );
        runtime.guards.push(
            GuardState::human(
                "Recovered blockade guard",
                runtime.width - 286.0,
                runtime.width - 356.0,
                runtime.width - 190.0,
            )
            .with_y(404.0)
            .with_speed(38.0)
            .with_detection(8.0, 12.0),
        );
        runtime
            .guards
            .push(GuardState::turret("Lawful medic drone", 700.0, 294.0).with_detection(0.0, 1.0));
        return;
    }
    if index == 5 {
        runtime.guards.push(
            GuardState::human("Lower clear-sight guard", 662.0, 586.0, 746.0)
                .with_speed(44.0)
                .with_detection(64.0, 36.0),
        );
        runtime.guards.push(
            GuardState::human("Upper vertical-sight guard", 958.0, 872.0, 1048.0)
                .with_y(344.0)
                .with_speed(42.0)
                .with_detection(70.0, 38.0),
        );
        return;
    }
    if index == 6 {
        runtime.guards.push(
            GuardState::human("Unhelmeted botanist runner", 486.0, 438.0, 628.0)
                .with_y(408.0)
                .with_speed(50.0)
                .with_detection(64.0, 38.0),
        );
        runtime.guards.push(
            GuardState::human("Oxygen crew scout", 878.0, 786.0, 944.0)
                .with_y(382.0)
                .with_speed(48.0)
                .with_detection(66.0, 38.0),
        );
        runtime.guards.push(
            GuardState::human("Upper garden security", 1168.0, 1048.0, 1264.0)
                .with_y(352.0)
                .with_speed(56.0)
                .with_detection(82.0, 40.0),
        );
        return;
    }
    if index == 7 {
        runtime.guards.push(
            GuardState::human("Pressurized chamber guard", 884.0, 812.0, 1018.0)
                .with_speed(54.0)
                .with_detection(76.0, 38.0),
        );
        runtime.guards.push(
            GuardState::human("Exterior strip responder", 1220.0, 1138.0, 1366.0)
                .with_y(374.0)
                .with_speed(50.0)
                .with_detection(72.0, 38.0),
        );
        return;
    }
    if try_savior_late(runtime, index) {
        return;
    }
    if index < 2 {
        let x = runtime.width - 220.0;
        runtime.guards.push(
            GuardState::human("Helpful-route guard", x, x - 80.0, runtime.width - 74.0)
                .with_speed(48.0)
                .with_detection(145.0, 54.0),
        );
        return;
    }

    let count = helpful_guard_count(index);
    push_helpful_patrols(runtime, index, count);
    if matches!(index, 9 | 10 | 13 | 18) {
        let turret_x = runtime.width * 0.66;
        runtime.guards.push(
            GuardState::turret("Restored security device", turret_x, 260.0)
                .with_detection(300.0, 62.0),
        );
    }
    if matches!(index, 10 | 18) {
        let x = runtime.width * 0.82;
        runtime.guards.push(
            GuardState::elite("Coordinated security", x, x - 80.0, x + 120.0)
                .with_detection(235.0, 58.0),
        );
    }
}

fn build_villain_room(runtime: &mut LevelRuntime, index: usize) {
    runtime.ambience.emergency = true;
    runtime.ambience.sparks = matches!(index, 3 | 9 | 13 | 16 | 18);
    runtime.ambience.smoke = matches!(index, 2 | 5 | 6 | 8 | 13 | 16 | 18);
    runtime.ambience.darkness = matches!(index, 8 | 12 | 18);
    runtime.ambience.gravity_off = matches!(index, 0 | 5 | 7 | 10 | 14 | 17);
    runtime.ambience.quiet = matches!(index, 8 | 9 | 11 | 15 | 17 | 18);
    runtime.ambience.turret_hacked = matches!(index, 9 | 13 | 15 | 18);

    let name = gremlin_guard_name(index);
    let x = (runtime.width * 0.65).min(runtime.width - 160.0);
    let mut guard = GuardState::human(name, x, x - 70.0, x + 110.0)
        .with_speed(18.0)
        .with_detection(48.0, 38.0)
        .panicked();
    if matches!(index, 0 | 5 | 6 | 7 | 10 | 16 | 17) {
        guard = guard.floating();
    }
    if matches!(index, 2 | 8) {
        guard = guard.inactive();
    }
    if index == 2 {
        guard = GuardState::human(
            name,
            runtime.width - 334.0,
            runtime.width - 380.0,
            runtime.width - 260.0,
        )
        .with_speed(16.0)
        .with_detection(34.0, 32.0)
        .panicked();
    }
    if index == 3 {
        runtime.guards.push(
            GuardState::human("Sliding civilian 1", 384.0, 300.0, 520.0)
                .with_speed(30.0)
                .with_detection(0.0, 1.0)
                .panicked(),
        );
        runtime.guards.push(
            GuardState::human("Sliding civilian 2", 720.0, 620.0, 860.0)
                .with_speed(34.0)
                .with_detection(0.0, 1.0)
                .panicked(),
        );
        runtime.guards.push(
            GuardState::human(
                "Sliding civilian 3",
                runtime.width - 270.0,
                runtime.width - 398.0,
                runtime.width - 188.0,
            )
            .with_speed(32.0)
            .with_detection(0.0, 1.0)
            .panicked(),
        );
        return;
    }
    if index == 4 {
        runtime.guards.push(
            GuardState::human("Bandage-cocoon patient 1", 620.0, 0.0, 0.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        runtime.guards.push(
            GuardState::human("Bandage-cocoon patient 2", runtime.width - 324.0, 0.0, 0.0)
                .with_y(404.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        return;
    }
    if index == 5 {
        runtime.guards.push(
            GuardState::human("Wall-netted engineer", 886.0, 0.0, 0.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        runtime.guards.push(
            GuardState::human("Wall-netted upper guard", runtime.width - 270.0, 0.0, 0.0)
                .with_y(344.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        return;
    }
    if index == 6 {
        runtime.guards.push(
            GuardState::human("Vine-tangled guard 1", 634.0, 0.0, 0.0)
                .with_y(418.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        runtime.guards.push(
            GuardState::human("Vine-tangled guard 2", 1118.0, 0.0, 0.0)
                .with_y(362.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive(),
        );
        return;
    }
    if index == 7 {
        runtime.guards.push(
            GuardState::human("Padded-net guard 1", 724.0, 0.0, 0.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive()
                .floating(),
        );
        runtime.guards.push(
            GuardState::human("Padded-net guard 2", runtime.width - 360.0, 0.0, 0.0)
                .with_y(382.0)
                .with_speed(0.0)
                .with_detection(0.0, 1.0)
                .panicked()
                .inactive()
                .floating(),
        );
        return;
    }
    if try_villain_late(runtime, index) {
        return;
    }
    runtime.guards.push(guard);

    if matches!(index, 1 | 4 | 8 | 13 | 16) {
        let w = 72.0;
        runtime.crates.push(CrateState {
            rect: Rect::new(
                (runtime.width * 0.70).min(runtime.width - w - 80.0),
                FLOOR_Y - 42.0,
                w,
                42.0,
            ),
            marked: false,
        });
    }
    if matches!(index, 13 | 18) {
        let turret_x = runtime.width * 0.62;
        runtime
            .guards
            .push(GuardState::turret("Confused security device", turret_x, 260.0).inactive());
    }
}

fn helpful_guard_count(index: usize) -> usize {
    match index {
        3..=9 => 1,
        10..=14 => 2,
        _ => 3,
    }
}

fn push_helpful_patrols(runtime: &mut LevelRuntime, index: usize, count: usize) {
    for guard_index in 0..count {
        let spacing = if count == 1 {
            0.0
        } else {
            0.26 / (count - 1) as f32
        };
        let start = if count == 1 { 0.82 } else { 0.54 };
        let center = runtime.width * (start + guard_index as f32 * spacing);
        let left = (center - 70.0).max(620.0);
        let right = (center + 85.0).min(runtime.width - 82.0);
        let range = if count == 1 {
            150.0 + index as f32 * 2.0
        } else {
            (195.0 + index as f32 * 4.0).min(255.0)
        };
        runtime.guards.push(
            GuardState::human(
                &format!("Restored patrol {}", guard_index + 1),
                center,
                left,
                right,
            )
            .with_speed(76.0 + index as f32 * 3.0)
            .with_detection(range, 56.0 + (index % 4) as f32 * 2.0),
        );
    }
}

fn gremlin_guard_name(index: usize) -> &'static str {
    match index {
        0 => "Floating boot guard",
        1 => "Crate-magnet guard",
        2 => "Half-suited foam guard",
        3 => "Sliding cafeteria guard",
        4 => "Bandage-cocoon patient",
        5 => "Streamer-netted technician",
        6 => "Vine-tangled gardener",
        7 => "Padded-net security",
        8 => "Blanket-shuffling crew",
        9 => "Wrong-badge supervisor",
        10 => "Drone-delivered guard",
        11 => "Revolving-door prisoner",
        12 => "Silhouette searchlight guard",
        13 => "Foam-cannon victim",
        14 => "Laundry-bin passenger",
        15 => "Vending-machine saluter",
        16 => "Coolant-foam engineer",
        17 => "Wrong-floor security",
        _ => "Propaganda-confused guard",
    }
}
