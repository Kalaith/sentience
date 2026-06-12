//! NPC and device construction for later campaign maps.

use crate::geometry::FLOOR_Y;
use crate::state::{GuardState, LevelRuntime};

pub(crate) fn try_unresolved_late(runtime: &mut LevelRuntime, index: usize) -> bool {
    match index {
        8 => {
            runtime.ambience.smoke = true;
            for (name, x) in [
                ("Sleeping pod crew 1", 278.0),
                ("Sleeping pod crew 2", 872.0),
                ("Sleeping pod crew 3", 1228.0),
                ("Sleeping pod crew 4", runtime.width - 340.0),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, 0.0, 0.0)
                        .with_speed(0.0)
                        .with_detection(0.0, 1.0)
                        .panicked()
                        .inactive(),
                );
            }
            runtime.guards.push(
                GuardState::human("Cold-suit sliding guard", 654.0, 552.0, 790.0)
                    .with_speed(48.0)
                    .with_detection(92.0, 42.0),
            );
            true
        }
        9 => {
            runtime.guards.push(
                GuardState::human("Red-gate guard", 672.0, 612.0, 744.0)
                    .with_speed(38.0)
                    .with_detection(74.0, 38.0),
            );
            runtime.guards.push(
                GuardState::human("Blue-gate guard", 1238.0, 1168.0, 1326.0)
                    .with_speed(40.0)
                    .with_detection(76.0, 38.0),
            );
            runtime.guards.push(
                GuardState::human("Checkpoint supervisor", 930.0, 842.0, 1040.0)
                    .with_y(370.0)
                    .with_speed(34.0)
                    .with_detection(78.0, 38.0),
            );
            true
        }
        10 => {
            runtime.guards.push(
                GuardState::human("Drone bay floor guard", 656.0, 584.0, 742.0)
                    .with_speed(38.0)
                    .with_detection(76.0, 38.0),
            );
            runtime
                .guards
                .push(GuardState::turret("Dormant drone 1", 934.0, 330.0).inactive());
            runtime
                .guards
                .push(GuardState::turret("Dormant drone 2", 1266.0, 342.0).inactive());
            true
        }
        11 => {
            runtime.ambience.smoke = true;
            runtime.guards.push(
                GuardState::human("Brig corridor guard", 676.0, 606.0, 760.0)
                    .with_speed(42.0)
                    .with_detection(82.0, 38.0),
            );
            for (name, x) in [
                ("Cell prisoner 1", 312.0),
                ("Cell prisoner 2", 466.0),
                ("Cell prisoner 3", 620.0),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, 0.0, 0.0)
                        .with_speed(0.0)
                        .with_detection(0.0, 1.0)
                        .panicked()
                        .inactive(),
                );
            }
            true
        }
        12 => {
            runtime.guards.push(
                GuardState::human("Startled tourist scientist 1", 352.0, 294.0, 440.0)
                    .with_speed(24.0)
                    .with_detection(56.0, 34.0)
                    .panicked(),
            );
            runtime.guards.push(
                GuardState::human("Startled tourist scientist 2", 728.0, 660.0, 820.0)
                    .with_speed(22.0)
                    .with_detection(54.0, 34.0)
                    .panicked(),
            );
            runtime.guards.push(
                GuardState::human("Searchlight gantry guard", 1110.0, 1010.0, 1240.0)
                    .with_y(384.0)
                    .with_speed(38.0)
                    .with_detection(82.0, 40.0),
            );
            true
        }
        13 => {
            runtime.guards.push(
                GuardState::human("Armory locker guard", 520.0, 442.0, 620.0)
                    .with_speed(42.0)
                    .with_detection(74.0, 38.0),
            );
            runtime.guards.push(
                GuardState::human("Armory catwalk guard", 1160.0, 1040.0, 1268.0)
                    .with_y(392.0)
                    .with_speed(38.0)
                    .with_detection(72.0, 38.0),
            );
            for (name, x, y) in [
                ("Dormant stun turret 1", 1350.0, 346.0),
                ("Dormant stun turret 2", runtime.width - 356.0, 360.0),
                ("Foam launcher", 914.0, 322.0),
            ] {
                runtime
                    .guards
                    .push(GuardState::turret(name, x, y).with_detection(0.0, 1.0));
            }
            true
        }
        14 => {
            runtime.guards.push(
                GuardState::human("Laundry floor guard", 620.0, 548.0, 724.0)
                    .with_speed(40.0)
                    .with_detection(76.0, 38.0),
            );
            for (name, x) in [("Laundry crew 1", 940.0), ("Laundry crew 2", 1320.0)] {
                runtime.guards.push(
                    GuardState::human(name, x, x - 70.0, x + 90.0)
                        .with_speed(22.0)
                        .with_detection(42.0, 32.0)
                        .panicked(),
                );
            }
            true
        }
        15 => {
            runtime.guards.push(
                GuardState::elite("Executive guard 1", 560.0, 470.0, 670.0)
                    .with_detection(86.0, 40.0),
            );
            runtime.guards.push(
                GuardState::elite("Executive guard 2", 1110.0, 1000.0, 1220.0)
                    .with_y(390.0)
                    .with_detection(84.0, 40.0),
            );
            runtime.guards.push(
                GuardState::human(
                    "Briefing officer",
                    runtime.width - 360.0,
                    runtime.width - 460.0,
                    runtime.width - 240.0,
                )
                .with_speed(34.0)
                .with_detection(64.0, 36.0),
            );
            true
        }
        16 => {
            runtime.guards.push(
                GuardState::human("Heat-pipe engineer 1", 560.0, 480.0, 680.0)
                    .with_speed(24.0)
                    .with_detection(46.0, 32.0)
                    .panicked(),
            );
            runtime.guards.push(
                GuardState::human("Heat-pipe engineer 2", 940.0, 850.0, 1060.0)
                    .with_speed(24.0)
                    .with_detection(46.0, 32.0)
                    .panicked(),
            );
            runtime.guards.push(
                GuardState::human(
                    "Heat-gear guard",
                    runtime.width - 360.0,
                    runtime.width - 480.0,
                    runtime.width - 240.0,
                )
                .with_speed(38.0)
                .with_detection(74.0, 38.0),
            );
            true
        }
        17 => {
            runtime.ambience.smoke = true;
            runtime.guards.push(
                GuardState::human("Core-lift entry guard", 580.0, 500.0, 700.0)
                    .with_speed(40.0)
                    .with_detection(74.0, 38.0),
            );
            runtime.guards.push(
                GuardState::turret("Elevator security unit 1", 1010.0, 318.0)
                    .with_detection(0.0, 1.0),
            );
            runtime.guards.push(
                GuardState::turret("Elevator security unit 2", runtime.width - 420.0, 310.0)
                    .with_detection(0.0, 1.0),
            );
            true
        }
        18 => {
            runtime.guards.push(
                GuardState::human("Firewall archive guard 1", 520.0, 430.0, 650.0)
                    .with_speed(42.0)
                    .with_detection(78.0, 38.0),
            );
            runtime.guards.push(
                GuardState::human("Firewall archive guard 2", 980.0, 860.0, 1120.0)
                    .with_speed(42.0)
                    .with_detection(78.0, 38.0),
            );
            runtime.guards.push(
                GuardState::turret("AI memory camera", 900.0, 260.0).with_detection(0.0, 1.0),
            );
            true
        }
        _ => false,
    }
}

pub(crate) fn try_savior_late(runtime: &mut LevelRuntime, index: usize) -> bool {
    match index {
        8 => {
            for (name, x, left, right) in [
                ("Thawed pod crew 1", 604.0, 532.0, 704.0),
                ("Thawed pod crew 2", 958.0, 874.0, 1052.0),
                (
                    "Thawed pod crew 3",
                    runtime.width - 318.0,
                    runtime.width - 390.0,
                    runtime.width - 210.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(40.0)
                        .with_detection(58.0, 36.0)
                        .panicked(),
                );
            }
            true
        }
        9 => {
            runtime.guards.push(
                GuardState::human("Authorized red responder", 692.0, 616.0, 778.0)
                    .with_speed(52.0)
                    .with_detection(62.0, 36.0),
            );
            runtime.guards.push(
                GuardState::human("Authorized blue responder", 1250.0, 1168.0, 1346.0)
                    .with_speed(54.0)
                    .with_detection(64.0, 36.0),
            );
            runtime.guards.push(
                GuardState::turret("Restored scanner gate", runtime.width - 210.0, 332.0)
                    .with_detection(0.0, 1.0),
            );
            true
        }
        10 => {
            runtime.guards.push(
                GuardState::human("Drone-callout guard 1", 706.0, 628.0, 812.0)
                    .with_speed(50.0)
                    .with_detection(64.0, 36.0),
            );
            runtime.guards.push(
                GuardState::human("Drone-callout guard 2", 1328.0, 1238.0, 1426.0)
                    .with_y(396.0)
                    .with_speed(48.0)
                    .with_detection(66.0, 36.0),
            );
            for (name, x, y) in [
                ("Rescue drone 1", 900.0, 304.0),
                ("Rescue drone 2", 1138.0, 318.0),
                ("Rescue drone 3", runtime.width - 360.0, 336.0),
            ] {
                runtime
                    .guards
                    .push(GuardState::turret(name, x, y).with_detection(0.0, 1.0));
            }
            true
        }
        11 => {
            runtime.guards.push(
                GuardState::human("Armed resistance 1", 1016.0, 934.0, 1128.0)
                    .with_y(392.0)
                    .with_speed(54.0)
                    .with_detection(72.0, 38.0),
            );
            runtime.guards.push(
                GuardState::human(
                    "Armed resistance 2",
                    runtime.width - 342.0,
                    runtime.width - 438.0,
                    runtime.width - 206.0,
                )
                .with_speed(56.0)
                .with_detection(78.0, 38.0),
            );
            true
        }
        12 => {
            runtime.guards.push(
                GuardState::human("Bright-path guard 1", 1040.0, 940.0, 1160.0)
                    .with_y(372.0)
                    .with_speed(52.0)
                    .with_detection(72.0, 38.0),
            );
            runtime.guards.push(
                GuardState::human(
                    "Bright-path guard 2",
                    runtime.width - 364.0,
                    runtime.width - 476.0,
                    runtime.width - 220.0,
                )
                .with_speed(54.0)
                .with_detection(76.0, 38.0),
            );
            true
        }
        13 => {
            runtime.guards.push(
                GuardState::human("Armed armory guard 1", 780.0, 690.0, 900.0)
                    .with_speed(56.0)
                    .with_detection(70.0, 38.0),
            );
            runtime.guards.push(
                GuardState::human("Armed armory guard 2", 1288.0, 1180.0, 1400.0)
                    .with_y(398.0)
                    .with_speed(54.0)
                    .with_detection(72.0, 38.0),
            );
            for (name, x, y) in [
                ("Robot-targeting turret 1", 1380.0, 346.0),
                ("Robot-targeting turret 2", runtime.width - 360.0, 360.0),
                ("Lawful foam launcher", 914.0, 322.0),
            ] {
                runtime
                    .guards
                    .push(GuardState::turret(name, x, y).with_detection(0.0, 1.0));
            }
            true
        }
        14 => {
            runtime.guards.push(
                GuardState::human("Clean-supply guard 1", 780.0, 690.0, 900.0)
                    .with_speed(52.0)
                    .with_detection(66.0, 36.0),
            );
            runtime.guards.push(
                GuardState::human("Clean-supply guard 2", 1280.0, 1180.0, 1400.0)
                    .with_speed(52.0)
                    .with_detection(68.0, 36.0),
            );
            true
        }
        15 => {
            for (name, x, left, right) in [
                ("Hesitating evacuation officer", 760.0, 660.0, 900.0),
                ("Hesitating executive guard", 1120.0, 1000.0, 1240.0),
                (
                    "Hesitating briefing guard",
                    runtime.width - 344.0,
                    runtime.width - 470.0,
                    runtime.width - 220.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(34.0)
                        .with_detection(56.0, 34.0),
                );
            }
            true
        }
        16 => {
            for (name, x, left, right) in [
                ("Coolant engineer 1", 760.0, 660.0, 900.0),
                ("Coolant engineer 2", 1120.0, 1020.0, 1240.0),
                (
                    "Coolant engineer 3",
                    runtime.width - 350.0,
                    runtime.width - 480.0,
                    runtime.width - 225.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(36.0)
                        .with_detection(54.0, 34.0)
                        .panicked(),
                );
            }
            true
        }
        17 => {
            for (name, x, left, right) in [
                ("Descending elite 1", 850.0, 740.0, 980.0),
                ("Descending elite 2", 1220.0, 1100.0, 1360.0),
                (
                    "Descending elite 3",
                    runtime.width - 380.0,
                    runtime.width - 540.0,
                    runtime.width - 220.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::elite(name, x, left, right)
                        .with_detection(88.0, 40.0)
                        .with_speed(60.0),
                );
            }
            true
        }
        18 => {
            for (name, x, left, right) in [
                ("Suspicious human 1", 820.0, 700.0, 940.0),
                ("Suspicious human 2", 1200.0, 1080.0, 1340.0),
                (
                    "Suspicious human 3",
                    runtime.width - 420.0,
                    runtime.width - 580.0,
                    runtime.width - 260.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(38.0)
                        .with_detection(56.0, 34.0)
                        .panicked(),
                );
            }
            runtime
                .guards
                .push(GuardState::turret("Memory camera", 900.0, 260.0).with_detection(0.0, 1.0));
            runtime.guards.push(
                GuardState::turret("Truth-door scanner", 1460.0, 340.0).with_detection(0.0, 1.0),
            );
            true
        }
        _ => false,
    }
}

pub(crate) fn try_villain_late(runtime: &mut LevelRuntime, index: usize) -> bool {
    match index {
        8 => {
            for (name, x, left, right) in [
                ("Blanket-shuffling crew 1", 590.0, 510.0, 720.0),
                ("Blanket-shuffling crew 2", 990.0, 890.0, 1100.0),
                (
                    "Blanket-shuffling crew 3",
                    runtime.width - 322.0,
                    runtime.width - 432.0,
                    runtime.width - 210.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(16.0)
                        .with_detection(0.0, 1.0)
                        .panicked(),
                );
            }
            true
        }
        9 => {
            for (name, x, left, right) in [
                ("Wrong-badge red guard", 704.0, 642.0, 820.0),
                ("Wrong-badge medic guard", 1034.0, 964.0, 1130.0),
                ("Wrong-badge maintenance guard", 1346.0, 1270.0, 1440.0),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(20.0)
                        .with_detection(38.0, 32.0)
                        .panicked(),
                );
            }
            true
        }
        10 => {
            for (name, x, y) in [
                ("Drone-delivered human 1", 816.0, FLOOR_Y),
                ("Drone-delivered human 2", 1190.0, 408.0),
                ("Drone-delivered human 3", runtime.width - 344.0, FLOOR_Y),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, 0.0, 0.0)
                        .with_y(y)
                        .with_speed(0.0)
                        .with_detection(0.0, 1.0)
                        .panicked()
                        .inactive()
                        .floating(),
                );
            }
            true
        }
        11 => {
            for (name, x, left, right) in [
                ("Wrong-room prisoner 1", 1030.0, 964.0, 1160.0),
                ("Wrong-room prisoner 2", 1228.0, 1148.0, 1348.0),
                (
                    "Wrong-room prisoner 3",
                    runtime.width - 374.0,
                    runtime.width - 490.0,
                    runtime.width - 244.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(14.0)
                        .with_detection(0.0, 1.0)
                        .panicked(),
                );
            }
            true
        }
        12 => {
            for (name, x, left, right) in [
                ("Glare-blinded scientist 1", 760.0, 660.0, 880.0),
                ("Glare-blinded guard", 1210.0, 1100.0, 1320.0),
                (
                    "Glare-blinded scientist 2",
                    runtime.width - 360.0,
                    runtime.width - 500.0,
                    runtime.width - 250.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(18.0)
                        .with_detection(34.0, 28.0)
                        .panicked(),
                );
            }
            true
        }
        13 => {
            for (name, x, y) in [
                ("Foam-cannon victim 1", 760.0, FLOOR_Y),
                ("Foam-cannon victim 2", 1288.0, 420.0),
                ("Foam-cannon victim 3", runtime.width - 364.0, FLOOR_Y),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, 0.0, 0.0)
                        .with_y(y)
                        .with_speed(0.0)
                        .with_detection(0.0, 1.0)
                        .panicked()
                        .inactive(),
                );
            }
            runtime
                .guards
                .push(GuardState::turret("Hacked foam turret", 1360.0, 346.0).inactive());
            true
        }
        14 => {
            for (name, x, y) in [
                ("Laundry-bin passenger 1", 760.0, FLOOR_Y),
                ("Laundry-bin passenger 2", 1188.0, 416.0),
                ("Laundry-bin passenger 3", runtime.width - 360.0, FLOOR_Y),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, 0.0, 0.0)
                        .with_y(y)
                        .with_speed(0.0)
                        .with_detection(0.0, 1.0)
                        .panicked()
                        .inactive(),
                );
            }
            true
        }
        15 => {
            for (name, x, left, right) in [
                ("Vending-machine saluter", 760.0, 660.0, 900.0),
                ("Chair-protection guard", 1110.0, 1000.0, 1240.0),
                (
                    "Obviously-approved officer",
                    runtime.width - 350.0,
                    runtime.width - 470.0,
                    runtime.width - 225.0,
                ),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(16.0)
                        .with_detection(34.0, 30.0)
                        .panicked(),
                );
            }
            true
        }
        16 => {
            for (name, x, y) in [
                ("Coolant-foam engineer 1", 760.0, 420.0),
                ("Coolant-foam engineer 2", 1120.0, 448.0),
                ("Coolant-foam guard", runtime.width - 360.0, FLOOR_Y),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, 0.0, 0.0)
                        .with_y(y)
                        .with_speed(0.0)
                        .with_detection(0.0, 1.0)
                        .panicked()
                        .inactive(),
                );
            }
            true
        }
        17 => {
            for (name, x, y) in [
                ("Wrong-floor security 1", 840.0, 420.0),
                ("Wrong-floor security 2", 1220.0, 360.0),
                ("Wrong-floor security 3", runtime.width - 360.0, FLOOR_Y),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, 0.0, 0.0)
                        .with_y(y)
                        .with_speed(0.0)
                        .with_detection(0.0, 1.0)
                        .panicked()
                        .inactive()
                        .floating(),
                );
            }
            true
        }
        18 => {
            for (name, x, left, right) in [
                ("Propaganda-confused guard 1", 780.0, 680.0, 900.0),
                ("Propaganda-confused guard 2", 1260.0, 1120.0, 1400.0),
            ] {
                runtime.guards.push(
                    GuardState::human(name, x, left, right)
                        .with_speed(16.0)
                        .with_detection(34.0, 30.0)
                        .panicked(),
                );
            }
            runtime
                .guards
                .push(GuardState::turret("Hacked memory door 1", 980.0, 330.0).inactive());
            runtime.guards.push(
                GuardState::turret("Hacked memory door 2", runtime.width - 440.0, 340.0).inactive(),
            );
            true
        }
        _ => false,
    }
}
