//! Level-specific setpiece construction.

use crate::geometry::FLOOR_Y;
use crate::level_setpieces_late::add_late_level_setpieces;
use crate::state::{LevelPhase, LevelRuntime, MoralChoice, SetpieceKind, SetpieceState};
use macroquad::prelude::*;

pub(crate) fn add_level_setpieces(runtime: &mut LevelRuntime, index: usize) {
    if add_late_level_setpieces(runtime, index) {
        return;
    }

    match index {
        0 => add_scrap_wake_setpieces(runtime),
        1 => add_cargo_carousel_setpieces(runtime),
        2 => add_suit_locker_setpieces(runtime),
        3 => add_mess_hall_setpieces(runtime),
        4 => add_med_bay_setpieces(runtime),
        5 => add_vent_stack_setpieces(runtime),
        6 => add_hydroponic_canopy_setpieces(runtime),
        7 => add_airlock_tether_setpieces(runtime),
        8 => add_cryo_thaw_setpieces(runtime),
        _ => {}
    }
}

fn add_scrap_wake_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::ScrapHeap,
            Rect::new(42.0, FLOOR_Y - 72.0, 150.0, 72.0),
        ),
        SetpieceState::new(
            SetpieceKind::CrouchTunnel,
            Rect::new(286.0, FLOOR_Y - 48.0, 220.0, 48.0),
        ),
        SetpieceState::new(
            SetpieceKind::BrokenCatwalk,
            Rect::new(548.0, 394.0, 282.0, 28.0),
        ),
        SetpieceState::new(
            SetpieceKind::ExitLedge,
            Rect::new(runtime.width - 158.0, FLOOR_Y - 28.0, 140.0, 28.0),
        ),
    ]);

    let gravity_state = match runtime.phase {
        LevelPhase::AwaitingChoice => (
            SetpieceKind::HangingScrap,
            Rect::new(504.0, 336.0, 146.0, 80.0),
        ),
        LevelPhase::Resolved(MoralChoice::Savior) => (
            SetpieceKind::DroppedScrapBridge,
            Rect::new(520.0, 440.0, 230.0, 18.0),
        ),
        LevelPhase::Resolved(MoralChoice::Villain) => (
            SetpieceKind::FloatingScrapCover,
            Rect::new(454.0, 322.0, 330.0, 92.0),
        ),
        LevelPhase::Final => return,
    };
    runtime
        .setpieces
        .push(SetpieceState::new(gravity_state.0, gravity_state.1));
}

fn add_cargo_carousel_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::CargoBelt,
            Rect::new(86.0, FLOOR_Y - 24.0, runtime.width - 250.0, 24.0),
        ),
        SetpieceState::new(
            SetpieceKind::CargoLift,
            Rect::new(584.0, FLOOR_Y - 116.0, 116.0, 116.0),
        ),
        SetpieceState::new(
            SetpieceKind::ScannerGate,
            Rect::new(runtime.width - 154.0, 326.0, 54.0, 174.0),
        ),
        SetpieceState::new(
            SetpieceKind::DroneLane,
            Rect::new(250.0, 238.0, runtime.width - 492.0, 44.0),
        ),
    ]);

    let route_piece = match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => (
            SetpieceKind::SupplyPalletBridge,
            Rect::new(314.0, 432.0, 520.0, 26.0),
        ),
        LevelPhase::Resolved(MoralChoice::Villain) => (
            SetpieceKind::MagnetizedCrateTrail,
            Rect::new(270.0, FLOOR_Y - 54.0, runtime.width - 420.0, 42.0),
        ),
        LevelPhase::AwaitingChoice | LevelPhase::Final => return,
    };
    runtime
        .setpieces
        .push(SetpieceState::new(route_piece.0, route_piece.1));
}

fn add_suit_locker_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::LockerBank,
            Rect::new(112.0, FLOOR_Y - 178.0, 286.0, 132.0),
        ),
        SetpieceState::new(
            SetpieceKind::DeconShower,
            Rect::new(488.0, 214.0, 228.0, FLOOR_Y - 214.0),
        ),
        SetpieceState::new(
            SetpieceKind::SprayLift,
            Rect::new(550.0, 248.0, 74.0, FLOOR_Y - 248.0),
        ),
        SetpieceState::new(
            SetpieceKind::SuitRack,
            Rect::new(runtime.width - 472.0, FLOOR_Y - 206.0, 178.0, 158.0),
        ),
        SetpieceState::new(
            SetpieceKind::SuitRack,
            Rect::new(runtime.width - 268.0, FLOOR_Y - 258.0, 168.0, 210.0),
        ),
        SetpieceState::new(
            SetpieceKind::MaintenanceCrawlspace,
            Rect::new(runtime.width - 384.0, FLOOR_Y - 48.0, 284.0, 48.0),
        ),
    ]);

    if matches!(runtime.phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::FoamBouncePad,
            Rect::new(runtime.width - 432.0, FLOOR_Y - 76.0, 156.0, 34.0),
        ));
    }
}

fn add_mess_hall_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::KitchenPass,
            Rect::new(84.0, FLOOR_Y - 158.0, 168.0, 118.0),
        ),
        SetpieceState::new(
            SetpieceKind::DiningPit,
            Rect::new(430.0, FLOOR_Y - 34.0, 286.0, 34.0),
        ),
        SetpieceState::new(
            SetpieceKind::ServingRail,
            Rect::new(332.0, 254.0, runtime.width - 560.0, 44.0),
        ),
        SetpieceState::new(
            SetpieceKind::DiningTable,
            Rect::new(174.0, FLOOR_Y - 56.0, 252.0, 36.0),
        ),
        SetpieceState::new(
            SetpieceKind::DiningTable,
            Rect::new(510.0, FLOOR_Y - 60.0, 292.0, 38.0),
        ),
        SetpieceState::new(
            SetpieceKind::DishReturnRamp,
            Rect::new(runtime.width - 356.0, 406.0, 292.0, 74.0),
        ),
    ]);

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::EvacuationFlow,
                Rect::new(runtime.width - 474.0, FLOOR_Y - 118.0, 338.0, 82.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::SlipperyGel,
                    Rect::new(130.0, FLOOR_Y - 26.0, runtime.width - 260.0, 22.0),
                ),
                SetpieceState::new(SetpieceKind::MealCart, Rect::new(664.0, 374.0, 126.0, 38.0)),
                SetpieceState::new(
                    SetpieceKind::MealCart,
                    Rect::new(runtime.width - 302.0, 424.0, 126.0, 38.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::MealCart,
                Rect::new(632.0, 374.0, 110.0, 36.0),
            ));
        }
    }
}

fn add_med_bay_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::ReceptionDesk,
            Rect::new(96.0, FLOOR_Y - 78.0, 260.0, 58.0),
        ),
        SetpieceState::new(
            SetpieceKind::TreatmentRoom,
            Rect::new(420.0, FLOOR_Y - 190.0, 376.0, 146.0),
        ),
        SetpieceState::new(
            SetpieceKind::BedLift,
            Rect::new(732.0, FLOOR_Y - 154.0, 112.0, 154.0),
        ),
        SetpieceState::new(
            SetpieceKind::RecoveryHallway,
            Rect::new(846.0, 348.0, runtime.width - 1040.0, 56.0),
        ),
        SetpieceState::new(
            SetpieceKind::TriageDoor,
            Rect::new(runtime.width - 142.0, 344.0, 54.0, 156.0),
        ),
        SetpieceState::new(
            SetpieceKind::RollingBed,
            Rect::new(520.0, FLOOR_Y - 42.0, 122.0, 38.0),
        ),
        SetpieceState::new(
            SetpieceKind::MedicDrone,
            Rect::new(656.0, 292.0, 48.0, 36.0),
        ),
    ]);

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::AutoDoc,
                Rect::new(516.0, FLOOR_Y - 210.0, 168.0, 118.0),
            ));
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::BandageCocoon,
                    Rect::new(618.0, FLOOR_Y - 64.0, 76.0, 58.0),
                ),
                SetpieceState::new(
                    SetpieceKind::BandageCocoon,
                    Rect::new(runtime.width - 336.0, 348.0, 76.0, 58.0),
                ),
                SetpieceState::new(
                    SetpieceKind::AutoDoc,
                    Rect::new(516.0, FLOOR_Y - 210.0, 168.0, 118.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {}
    }
}

fn add_vent_stack_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::VentShaft,
            Rect::new(500.0, 168.0, 238.0, FLOOR_Y - 168.0),
        ),
        SetpieceState::new(
            SetpieceKind::MaintenanceRoom,
            Rect::new(122.0, FLOOR_Y - 132.0, 308.0, 104.0),
        ),
        SetpieceState::new(
            SetpieceKind::MaintenanceRoom,
            Rect::new(762.0, 310.0, 292.0, 104.0),
        ),
        SetpieceState::new(
            SetpieceKind::MaintenanceRoom,
            Rect::new(runtime.width - 398.0, 246.0, 286.0, 104.0),
        ),
        SetpieceState::new(
            SetpieceKind::FanColumn,
            Rect::new(538.0, 290.0, 76.0, 210.0),
        ),
        SetpieceState::new(
            SetpieceKind::FanColumn,
            Rect::new(676.0, 212.0, 76.0, 288.0),
        ),
        SetpieceState::new(
            SetpieceKind::CrosswindGap,
            Rect::new(runtime.width - 520.0, 306.0, 346.0, 86.0),
        ),
    ]);

    if matches!(runtime.phase, LevelPhase::Resolved(MoralChoice::Villain)) {
        runtime.setpieces.extend([
            SetpieceState::new(
                SetpieceKind::SmokePocket,
                Rect::new(256.0, 352.0, 160.0, 94.0),
            ),
            SetpieceState::new(
                SetpieceKind::SmokePocket,
                Rect::new(804.0, 284.0, 172.0, 94.0),
            ),
            SetpieceState::new(
                SetpieceKind::WallNet,
                Rect::new(846.0, FLOOR_Y - 142.0, 112.0, 102.0),
            ),
            SetpieceState::new(
                SetpieceKind::WallNet,
                Rect::new(runtime.width - 312.0, 268.0, 112.0, 102.0),
            ),
            SetpieceState::new(
                SetpieceKind::StreamerPurge,
                Rect::new(520.0, 202.0, runtime.width - 840.0, 246.0),
            ),
        ]);
    }
}

fn add_hydroponic_canopy_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::IrrigationTrench,
            Rect::new(134.0, FLOOR_Y - 54.0, runtime.width - 302.0, 46.0),
        ),
        SetpieceState::new(
            SetpieceKind::VineCanopy,
            Rect::new(430.0, 282.0, 560.0, 150.0),
        ),
        SetpieceState::new(
            SetpieceKind::MaintenanceWalkway,
            Rect::new(982.0, 318.0, runtime.width - 1130.0, 52.0),
        ),
        SetpieceState::new(SetpieceKind::SeedPod, Rect::new(744.0, 346.0, 76.0, 52.0)),
        SetpieceState::new(
            SetpieceKind::SeedPod,
            Rect::new(runtime.width - 470.0, 354.0, 76.0, 52.0),
        ),
        SetpieceState::new(
            SetpieceKind::SprinklerZone,
            Rect::new(560.0, 210.0, 420.0, FLOOR_Y - 210.0),
        ),
    ]);

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::PlantBridge,
                    Rect::new(462.0, 410.0, 468.0, 24.0),
                ),
                SetpieceState::new(
                    SetpieceKind::PlantBridge,
                    Rect::new(1000.0, 354.0, runtime.width - 1092.0, 24.0),
                ),
            ]);
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::PlantCurtain,
                    Rect::new(420.0, 334.0, 246.0, 126.0),
                ),
                SetpieceState::new(
                    SetpieceKind::VineTunnel,
                    Rect::new(runtime.width - 520.0, FLOOR_Y - 82.0, 360.0, 64.0),
                ),
                SetpieceState::new(
                    SetpieceKind::TendrilGate,
                    Rect::new(890.0, 314.0, 186.0, 116.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::TendrilGate,
                Rect::new(902.0, 322.0, 156.0, 104.0),
            ));
        }
    }
}

fn add_airlock_tether_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::AirlockStaging,
            Rect::new(78.0, FLOOR_Y - 152.0, 294.0, 110.0),
        ),
        SetpieceState::new(
            SetpieceKind::PressureChamber,
            Rect::new(438.0, FLOOR_Y - 188.0, 244.0, 146.0),
        ),
        SetpieceState::new(
            SetpieceKind::PressureChamber,
            Rect::new(770.0, FLOOR_Y - 184.0, 276.0, 142.0),
        ),
        SetpieceState::new(
            SetpieceKind::PressureChamber,
            Rect::new(1088.0, FLOOR_Y - 206.0, 286.0, 164.0),
        ),
        SetpieceState::new(
            SetpieceKind::ExteriorMaintenanceStrip,
            Rect::new(510.0, 352.0, runtime.width - 780.0, 74.0),
        ),
    ]);

    for x in [594.0, 936.0, 1210.0, runtime.width - 284.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::TetherAnchor,
            Rect::new(x, 340.0, 58.0, 58.0),
        ));
    }

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::PressureDoor,
                    Rect::new(690.0, FLOOR_Y - 194.0, 58.0, 152.0),
                ),
                SetpieceState::new(
                    SetpieceKind::PressureDoor,
                    Rect::new(1048.0, FLOOR_Y - 206.0, 58.0, 164.0),
                ),
                SetpieceState::new(
                    SetpieceKind::PressureDoor,
                    Rect::new(1392.0, FLOOR_Y - 198.0, 58.0, 156.0),
                ),
            ]);
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::PressureBurst,
                    Rect::new(518.0, 322.0, runtime.width - 792.0, 126.0),
                ),
                SetpieceState::new(
                    SetpieceKind::SafetyNet,
                    Rect::new(696.0, 360.0, 132.0, 92.0),
                ),
                SetpieceState::new(
                    SetpieceKind::SafetyNet,
                    Rect::new(runtime.width - 426.0, 344.0, 132.0, 92.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {
            runtime.setpieces.push(SetpieceState::new(
                SetpieceKind::PressureDoor,
                Rect::new(1018.0, FLOOR_Y - 198.0, 58.0, 156.0),
            ));
        }
    }
}

fn add_cryo_thaw_setpieces(runtime: &mut LevelRuntime) {
    runtime.setpieces.extend([
        SetpieceState::new(
            SetpieceKind::FrozenAisle,
            Rect::new(142.0, FLOOR_Y - 28.0, runtime.width - 278.0, 24.0),
        ),
        SetpieceState::new(
            SetpieceKind::ColdServicePipe,
            Rect::new(494.0, 336.0, runtime.width - 780.0, 62.0),
        ),
        SetpieceState::new(
            SetpieceKind::CryoPodRoom,
            Rect::new(178.0, FLOOR_Y - 178.0, 292.0, 130.0),
        ),
        SetpieceState::new(
            SetpieceKind::CryoPodRoom,
            Rect::new(786.0, FLOOR_Y - 184.0, 318.0, 136.0),
        ),
        SetpieceState::new(
            SetpieceKind::CryoPodRoom,
            Rect::new(1118.0, FLOOR_Y - 214.0, 316.0, 166.0),
        ),
        SetpieceState::new(
            SetpieceKind::CryoPodRoom,
            Rect::new(runtime.width - 414.0, FLOOR_Y - 188.0, 286.0, 140.0),
        ),
    ]);

    for x in [390.0, 742.0, 1110.0, runtime.width - 456.0] {
        runtime.setpieces.push(SetpieceState::new(
            SetpieceKind::ThawSwitch,
            Rect::new(x, FLOOR_Y - 98.0, 54.0, 76.0),
        ));
    }

    match runtime.phase {
        LevelPhase::Resolved(MoralChoice::Savior) => {
            for x in [258.0, 858.0, 1210.0] {
                runtime.setpieces.push(SetpieceState::new(
                    SetpieceKind::SleeperPod,
                    Rect::new(x, FLOOR_Y - 144.0, 92.0, 82.0),
                ));
            }
        }
        LevelPhase::Resolved(MoralChoice::Villain) => {
            runtime.setpieces.extend([
                SetpieceState::new(
                    SetpieceKind::ThermalBlanketCover,
                    Rect::new(506.0, FLOOR_Y - 66.0, 226.0, 54.0),
                ),
                SetpieceState::new(
                    SetpieceKind::ThermalBlanketCover,
                    Rect::new(runtime.width - 522.0, FLOOR_Y - 70.0, 244.0, 56.0),
                ),
            ]);
        }
        LevelPhase::AwaitingChoice | LevelPhase::Final => {
            for x in [258.0, 858.0, 1210.0, runtime.width - 340.0] {
                runtime.setpieces.push(SetpieceState::new(
                    SetpieceKind::SleeperPod,
                    Rect::new(x, FLOOR_Y - 144.0, 92.0, 82.0),
                ));
            }
        }
    }
}
