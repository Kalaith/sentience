//! Setpiece construction for later campaign maps.

use crate::geometry::FLOOR_Y;
use crate::state::{LevelPhase, LevelRuntime, MoralChoice, SetpieceKind, SetpieceState};
use macroquad::prelude::*;

pub(crate) fn add_late_level_setpieces(runtime: &mut LevelRuntime, index: usize) -> bool {
    match index {
        9 => add_badge_checkpoint_setpieces(runtime),
        10 => add_drone_bay_setpieces(runtime),
        11 => add_brig_door_setpieces(runtime),
        12 => add_observatory_shutter_setpieces(runtime),
        13 => add_armory_safety_setpieces(runtime),
        14 => add_laundry_tube_setpieces(runtime),
        15 => add_captains_quarters_setpieces(runtime),
        16 => add_reactor_foam_setpieces(runtime),
        17 => add_core_lift_setpieces(runtime),
        18 => add_moral_firewall_setpieces(runtime),
        _ => return false,
    }
    true
}

fn add_badge_checkpoint_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::QueueScannerCover,
            Rect::new(136.0, FLOOR_Y - 88.0, 338.0, 64.0),
        ),
        SetpieceState::new(
            SetpieceKind::BadgePrinter,
            Rect::new(490.0, FLOOR_Y - 118.0, 94.0, 92.0),
        ),
        SetpieceState::new(
            SetpieceKind::SecurityOffice,
            Rect::new(786.0, 282.0, 332.0, 108.0),
        ),
        SetpieceState::new(
            SetpieceKind::MaintenanceBypass,
            Rect::new(runtime.width - 540.0, FLOOR_Y - 76.0, 420.0, 60.0),
        ),
    ]);

    for (x, h) in [(650.0, 150.0), (990.0, 176.0), (1326.0, 154.0)] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::BadgeGate,
            Rect::new(x, FLOOR_Y - h - 24.0, 64.0, h),
        ));
    }

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::ScannerGate,
                Rect::new(runtime.width - 236.0, 340.0, 54.0, 156.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::GateJam,
                    Rect::new(610.0, FLOOR_Y - 144.0, 870.0, 108.0),
                ),
                SetpieceState::new(
                    SetpieceKind::WrongBadgeLoop,
                    Rect::new(748.0, 342.0, runtime.width - 980.0, 94.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}

fn add_drone_bay_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::DroneChargingPad,
            Rect::new(178.0, FLOOR_Y - 72.0, 348.0, 56.0),
        ),
        SetpieceState::new(
            SetpieceKind::DroneChargingPad,
            Rect::new(628.0, FLOOR_Y - 70.0, 292.0, 54.0),
        ),
        SetpieceState::new(
            SetpieceKind::DroneRail,
            Rect::new(520.0, 270.0, runtime.width - 748.0, 92.0),
        ),
        SetpieceState::new(
            SetpieceKind::DispatchTower,
            Rect::new(884.0, FLOOR_Y - 224.0, 252.0, 184.0),
        ),
        SetpieceState::new(
            SetpieceKind::DroneServicedHatch,
            Rect::new(runtime.width - 154.0, 374.0, 72.0, 122.0),
        ),
    ]);

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::RescueDronePath,
                Rect::new(520.0, 286.0, runtime.width - 720.0, 116.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::EnthusiasticDroneCarry,
                Rect::new(520.0, 318.0, runtime.width - 720.0, 124.0),
            ));
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}

fn add_brig_door_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::PrisonerWalkway,
            Rect::new(760.0, 332.0, runtime.width - 1030.0, 84.0),
        ),
        SetpieceState::new(
            SetpieceKind::DoorControlRoom,
            Rect::new(806.0, 304.0, 326.0, 118.0),
        ),
        SetpieceState::new(
            SetpieceKind::EvidenceLock,
            Rect::new(runtime.width - 168.0, 354.0, 70.0, 142.0),
        ),
        SetpieceState::new(
            SetpieceKind::MaintenanceCrawlspace,
            Rect::new(540.0, FLOOR_Y - 54.0, 380.0, 48.0),
        ),
    ]);

    for x in [276.0, 430.0, 584.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::BrigCellDoor,
            Rect::new(x, FLOOR_Y - 158.0, 72.0, 128.0),
        ));
    }

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            for x in [1016.0, 1300.0] {
                runtime.setpieces.push(SetpieceState::new(
                    SetpieceKind::OneWayDoor,
                    Rect::new(x, 328.0, 70.0, 112.0),
                ));
            }
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::WrongWaitingRoom,
                    Rect::new(1040.0, FLOOR_Y - 128.0, 320.0, 88.0),
                ),
                SetpieceState::new(
                    SetpieceKind::RevolvingDoorLoop,
                    Rect::new(runtime.width - 500.0, FLOOR_Y - 136.0, 142.0, 108.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}

fn add_observatory_shutter_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::ObservationDeck,
            Rect::new(126.0, FLOOR_Y - 92.0, runtime.width - 310.0, 70.0),
        ),
        SetpieceState::new(
            SetpieceKind::TelescopeGantry,
            Rect::new(866.0, 288.0, 470.0, 118.0),
        ),
        SetpieceState::new(
            SetpieceKind::SearchlightBeam,
            Rect::new(904.0, 258.0, 610.0, 204.0),
        ),
        SetpieceState::new(
            SetpieceKind::RadiationLock,
            Rect::new(runtime.width - 168.0, 354.0, 72.0, 142.0),
        ),
    ]);

    for x in [570.0, 1050.0, 1500.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::ShutterZone,
            Rect::new(x, 298.0, 240.0, 174.0),
        ));
    }

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::SearchlightBeam,
                Rect::new(1170.0, 274.0, runtime.width - 1390.0, 190.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::ShadowLane,
                    Rect::new(550.0, 392.0, runtime.width - 940.0, 74.0),
                ),
                SetpieceState::new(
                    SetpieceKind::GlareLane,
                    Rect::new(1000.0, 300.0, runtime.width - 1260.0, 146.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}

fn add_armory_safety_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::WeaponLockerCorridor,
            Rect::new(142.0, FLOOR_Y - 156.0, 520.0, 118.0),
        ),
        SetpieceState::new(
            SetpieceKind::FoamPit,
            Rect::new(646.0, FLOOR_Y - 76.0, 320.0, 58.0),
        ),
        SetpieceState::new(
            SetpieceKind::ArmoryCatwalk,
            Rect::new(936.0, 314.0, runtime.width - 1180.0, 106.0),
        ),
        SetpieceState::new(
            SetpieceKind::StunTurretLane,
            Rect::new(1280.0, 320.0, runtime.width - 1520.0, 146.0),
        ),
        SetpieceState::new(
            SetpieceKind::FoamLauncher,
            Rect::new(868.0, 316.0, 104.0, 76.0),
        ),
        SetpieceState::new(
            SetpieceKind::TargetingConsole,
            Rect::new(498.0, FLOOR_Y - 112.0, 92.0, 84.0),
        ),
    ]);

    if matches!(runtime.phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        runtime.setpieces.extend([
            SetpieceState::new(
                SetpieceKind::FoamPile,
                Rect::new(680.0, FLOOR_Y - 88.0, 310.0, 72.0),
            ),
            SetpieceState::new(
                SetpieceKind::FoamPile,
                Rect::new(runtime.width - 520.0, FLOOR_Y - 94.0, 300.0, 74.0),
            ),
        ]);
    }
}

fn add_laundry_tube_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::LaundryFloor,
            Rect::new(120.0, FLOOR_Y - 82.0, runtime.width - 260.0, 64.0),
        ),
        SetpieceState::new(
            SetpieceKind::LaundryTube,
            Rect::new(548.0, 306.0, runtime.width - 780.0, 96.0),
        ),
        SetpieceState::new(
            SetpieceKind::SuctionBurst,
            Rect::new(656.0, 336.0, runtime.width - 960.0, 110.0),
        ),
        SetpieceState::new(
            SetpieceKind::RollingLaundryCart,
            Rect::new(510.0, FLOOR_Y - 50.0, 116.0, 44.0),
        ),
    ]);

    for x in [900.0, 1260.0, runtime.width - 350.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::TubeExit,
            Rect::new(x, 346.0, 92.0, 82.0),
        ));
    }

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::LaundryBin,
                Rect::new(runtime.width - 476.0, FLOOR_Y - 82.0, 132.0, 68.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::UniformRetrievalTube,
                    Rect::new(610.0, 318.0, runtime.width - 940.0, 126.0),
                ),
                SetpieceState::new(
                    SetpieceKind::LaundryBin,
                    Rect::new(720.0, FLOOR_Y - 82.0, 132.0, 68.0),
                ),
                SetpieceState::new(
                    SetpieceKind::LaundryBin,
                    Rect::new(runtime.width - 466.0, FLOOR_Y - 84.0, 132.0, 70.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}

fn add_captains_quarters_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::TrophyHall,
            Rect::new(120.0, FLOOR_Y - 164.0, 450.0, 126.0),
        ),
        SetpieceState::new(
            SetpieceKind::CommandDesk,
            Rect::new(620.0, FLOOR_Y - 136.0, 330.0, 102.0),
        ),
        SetpieceState::new(
            SetpieceKind::EvidenceSafe,
            Rect::new(1010.0, FLOOR_Y - 176.0, 110.0, 130.0),
        ),
        SetpieceState::new(
            SetpieceKind::PrivateEscapeCorridor,
            Rect::new(1240.0, 326.0, runtime.width - 1570.0, 86.0),
        ),
        SetpieceState::new(
            SetpieceKind::BriefingRoom,
            Rect::new(runtime.width - 520.0, FLOOR_Y - 174.0, 420.0, 130.0),
        ),
    ]);

    for x in [920.0, runtime.width - 274.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::CommandLock,
            Rect::new(x, FLOOR_Y - 156.0, 66.0, 126.0),
        ));
    }

    if matches!(runtime.phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::FalseOrderTrail,
            Rect::new(690.0, FLOOR_Y - 150.0, runtime.width - 980.0, 116.0),
        ));
    }
}

fn add_reactor_foam_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::ReactorEntry,
            Rect::new(160.0, 316.0, 380.0, 106.0),
        ),
        SetpieceState::new(
            SetpieceKind::HeatPipeMaze,
            Rect::new(560.0, 328.0, 620.0, 142.0),
        ),
        SetpieceState::new(
            SetpieceKind::HeatZone,
            Rect::new(682.0, FLOOR_Y - 76.0, runtime.width - 1050.0, 58.0),
        ),
        SetpieceState::new(
            SetpieceKind::ReactorWalkway,
            Rect::new(runtime.width - 560.0, FLOOR_Y - 94.0, 440.0, 72.0),
        ),
    ]);

    for x in [760.0, 1080.0, 1410.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::SteamJet,
            Rect::new(x, 344.0, 116.0, 128.0),
        ));
    }
    for x in [934.0, runtime.width - 480.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::CoolantValve,
            Rect::new(x, FLOOR_Y - 118.0, 82.0, 82.0),
        ));
    }

    if matches!(runtime.phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        runtime.setpieces.extend([
            SetpieceState::new(
                SetpieceKind::FoamBubble,
                Rect::new(780.0, 382.0, 260.0, 84.0),
            ),
            SetpieceState::new(
                SetpieceKind::FoamBubble,
                Rect::new(runtime.width - 520.0, FLOOR_Y - 126.0, 310.0, 92.0),
            ),
        ]);
    }
}

fn add_core_lift_setpieces(runtime: &mut LevelRuntime) {
    for (x, y, w) in [
        (170.0, FLOOR_Y - 70.0, 360.0),
        (540.0, 388.0, 320.0),
        (1220.0, 326.0, 340.0),
        (runtime.width - 560.0, 306.0, 430.0),
    ] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::LiftLanding,
            Rect::new(x, y, w, 58.0),
        ));
    }

    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::CentralElevator,
            Rect::new(880.0, 286.0, 280.0, FLOOR_Y - 312.0),
        ),
        SetpieceState::new(
            SetpieceKind::ServiceLadder,
            Rect::new(670.0, 306.0, 88.0, FLOOR_Y - 330.0),
        ),
        SetpieceState::new(
            SetpieceKind::ServiceLadder,
            Rect::new(runtime.width - 360.0, 300.0, 88.0, FLOOR_Y - 324.0),
        ),
        SetpieceState::new(
            SetpieceKind::LiftScheduler,
            Rect::new(500.0, FLOOR_Y - 118.0, 96.0, 86.0),
        ),
    ]);

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::ElevatorSecurityUnit,
                Rect::new(1030.0, 318.0, 80.0, 60.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::WrongFloorDoor,
                    Rect::new(1248.0, 334.0, 76.0, 116.0),
                ),
                SetpieceState::new(
                    SetpieceKind::EmptyLiftWindow,
                    Rect::new(runtime.width - 520.0, 322.0, 168.0, 104.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}

fn add_moral_firewall_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::EvidenceArchive,
            Rect::new(118.0, FLOOR_Y - 170.0, 480.0, 128.0),
        ),
        SetpieceState::new(
            SetpieceKind::FirewallCorridor,
            Rect::new(590.0, 314.0, runtime.width - 950.0, 156.0),
        ),
        SetpieceState::new(
            SetpieceKind::AiCamera,
            Rect::new(860.0, 236.0, 110.0, 150.0),
        ),
        SetpieceState::new(
            SetpieceKind::CoreSeal,
            Rect::new(runtime.width - 170.0, 330.0, 82.0, 166.0),
        ),
    ]);

    for x in [650.0, 1060.0, 1480.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::MemoryDoor,
            Rect::new(x, FLOOR_Y - 172.0, 76.0, 138.0),
        ));
    }
    for x in [358.0, 738.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::DataCanister,
            Rect::new(x, FLOOR_Y - 54.0, 72.0, 48.0),
        ));
    }

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::TruthRoute,
                Rect::new(900.0, 334.0, runtime.width - 1280.0, 84.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::PropagandaRoute,
                Rect::new(920.0, FLOOR_Y - 98.0, runtime.width - 1320.0, 78.0),
            ));
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}
