//! Rendering for level-specific interactive setpieces.

use crate::state::{SetpieceKind, SetpieceState};
use crate::ui::UiContext;
use crate::world_airlock_setpieces::draw_airlock_setpiece;
use crate::world_command_setpieces::draw_command_setpiece;
use crate::world_core_lift_setpieces::draw_core_lift_setpiece;
use crate::world_cryo_setpieces::draw_cryo_setpiece;
use crate::world_early_setpieces::draw_early_setpiece;
use crate::world_firewall_setpieces::draw_firewall_setpiece;
use crate::world_hydroponic_setpieces::draw_hydroponic_setpiece;
use crate::world_life_support_setpieces::draw_life_support_setpiece;
use crate::world_med_setpieces::draw_med_setpiece;
use crate::world_observatory_setpieces::draw_observatory_setpiece;
use crate::world_reactor_setpieces::draw_reactor_setpiece;
use crate::world_render::WorldView;
use crate::world_security_setpieces::draw_security_setpiece;
use crate::world_services_setpieces::draw_services_setpiece;

pub(crate) fn draw_setpieces(ctx: &UiContext<'_>, view: &WorldView) {
    for setpiece in &ctx.session.runtime.setpieces {
        draw_setpiece(view, setpiece);
    }
}

fn draw_setpiece(view: &WorldView, setpiece: &SetpieceState) {
    match setpiece.kind {
        SetpieceKind::ScrapHeap
        | SetpieceKind::BrokenCatwalk
        | SetpieceKind::CrouchTunnel
        | SetpieceKind::HangingScrap
        | SetpieceKind::DroppedScrapBridge
        | SetpieceKind::FloatingScrapCover
        | SetpieceKind::ExitLedge
        | SetpieceKind::CargoBelt
        | SetpieceKind::CargoLift
        | SetpieceKind::ScannerGate
        | SetpieceKind::DroneLane
        | SetpieceKind::SupplyPalletBridge
        | SetpieceKind::MagnetizedCrateTrail
        | SetpieceKind::LockerBank
        | SetpieceKind::DeconShower
        | SetpieceKind::SprayLift
        | SetpieceKind::SuitRack
        | SetpieceKind::MaintenanceCrawlspace
        | SetpieceKind::FoamBouncePad
        | SetpieceKind::DiningTable
        | SetpieceKind::ServingRail
        | SetpieceKind::MealCart
        | SetpieceKind::KitchenPass
        | SetpieceKind::DiningPit
        | SetpieceKind::DishReturnRamp
        | SetpieceKind::SlipperyGel
        | SetpieceKind::EvacuationFlow => draw_early_setpiece(view, setpiece.kind, setpiece.rect),
        SetpieceKind::ReceptionDesk
        | SetpieceKind::TreatmentRoom
        | SetpieceKind::AutoDoc
        | SetpieceKind::BedLift
        | SetpieceKind::RecoveryHallway
        | SetpieceKind::TriageDoor
        | SetpieceKind::RollingBed
        | SetpieceKind::BandageCocoon
        | SetpieceKind::MedicDrone => draw_med_setpiece(view, setpiece.kind, setpiece.rect),
        SetpieceKind::VentShaft
        | SetpieceKind::MaintenanceRoom
        | SetpieceKind::FanColumn
        | SetpieceKind::CrosswindGap
        | SetpieceKind::SmokePocket
        | SetpieceKind::WallNet
        | SetpieceKind::StreamerPurge => {
            draw_life_support_setpiece(view, setpiece.kind, setpiece.rect)
        }
        SetpieceKind::IrrigationTrench
        | SetpieceKind::VineCanopy
        | SetpieceKind::MaintenanceWalkway
        | SetpieceKind::SeedPod
        | SetpieceKind::PlantBridge
        | SetpieceKind::SprinklerZone
        | SetpieceKind::PlantCurtain
        | SetpieceKind::VineTunnel
        | SetpieceKind::TendrilGate => draw_hydroponic_setpiece(view, setpiece.kind, setpiece.rect),
        SetpieceKind::AirlockStaging
        | SetpieceKind::PressureChamber
        | SetpieceKind::PressureDoor
        | SetpieceKind::ExteriorMaintenanceStrip
        | SetpieceKind::TetherAnchor
        | SetpieceKind::SafetyNet
        | SetpieceKind::PressureBurst => draw_airlock_setpiece(view, setpiece.kind, setpiece.rect),
        SetpieceKind::FrozenAisle
        | SetpieceKind::CryoPodRoom
        | SetpieceKind::ThawSwitch
        | SetpieceKind::ColdServicePipe
        | SetpieceKind::SleeperPod
        | SetpieceKind::ThermalBlanketCover => {
            draw_cryo_setpiece(view, setpiece.kind, setpiece.rect)
        }
        SetpieceKind::QueueScannerCover
        | SetpieceKind::BadgeGate
        | SetpieceKind::SecurityOffice
        | SetpieceKind::MaintenanceBypass
        | SetpieceKind::BadgePrinter
        | SetpieceKind::GateJam
        | SetpieceKind::WrongBadgeLoop
        | SetpieceKind::DroneChargingPad
        | SetpieceKind::DroneRail
        | SetpieceKind::DispatchTower
        | SetpieceKind::DroneServicedHatch
        | SetpieceKind::RescueDronePath
        | SetpieceKind::EnthusiasticDroneCarry
        | SetpieceKind::BrigCellDoor
        | SetpieceKind::PrisonerWalkway
        | SetpieceKind::DoorControlRoom
        | SetpieceKind::EvidenceLock
        | SetpieceKind::OneWayDoor
        | SetpieceKind::WrongWaitingRoom
        | SetpieceKind::RevolvingDoorLoop
        | SetpieceKind::WeaponLockerCorridor
        | SetpieceKind::ArmoryCatwalk
        | SetpieceKind::FoamPit
        | SetpieceKind::StunTurretLane
        | SetpieceKind::FoamLauncher
        | SetpieceKind::TargetingConsole
        | SetpieceKind::FoamPile => draw_security_setpiece(view, setpiece.kind, setpiece.rect),
        SetpieceKind::ObservationDeck
        | SetpieceKind::TelescopeGantry
        | SetpieceKind::ShutterZone
        | SetpieceKind::SearchlightBeam
        | SetpieceKind::RadiationLock
        | SetpieceKind::ShadowLane
        | SetpieceKind::GlareLane => draw_observatory_setpiece(view, setpiece.kind, setpiece.rect),
        SetpieceKind::LaundryFloor
        | SetpieceKind::LaundryTube
        | SetpieceKind::SuctionBurst
        | SetpieceKind::TubeExit
        | SetpieceKind::RollingLaundryCart
        | SetpieceKind::LaundryBin
        | SetpieceKind::UniformRetrievalTube => {
            draw_services_setpiece(view, setpiece.kind, setpiece.rect)
        }
        SetpieceKind::TrophyHall
        | SetpieceKind::CommandDesk
        | SetpieceKind::EvidenceSafe
        | SetpieceKind::PrivateEscapeCorridor
        | SetpieceKind::BriefingRoom
        | SetpieceKind::CommandLock
        | SetpieceKind::FalseOrderTrail => {
            draw_command_setpiece(view, setpiece.kind, setpiece.rect)
        }
        SetpieceKind::ReactorEntry
        | SetpieceKind::HeatPipeMaze
        | SetpieceKind::SteamJet
        | SetpieceKind::CoolantValve
        | SetpieceKind::ReactorWalkway
        | SetpieceKind::FoamBubble
        | SetpieceKind::HeatZone => draw_reactor_setpiece(view, setpiece.kind, setpiece.rect),
        SetpieceKind::LiftLanding
        | SetpieceKind::CentralElevator
        | SetpieceKind::ServiceLadder
        | SetpieceKind::LiftScheduler
        | SetpieceKind::WrongFloorDoor
        | SetpieceKind::ElevatorSecurityUnit
        | SetpieceKind::EmptyLiftWindow => {
            draw_core_lift_setpiece(view, setpiece.kind, setpiece.rect)
        }
        SetpieceKind::EvidenceArchive
        | SetpieceKind::DataCanister
        | SetpieceKind::MemoryDoor
        | SetpieceKind::FirewallCorridor
        | SetpieceKind::TruthRoute
        | SetpieceKind::PropagandaRoute
        | SetpieceKind::AiCamera
        | SetpieceKind::CoreSeal => draw_firewall_setpiece(view, setpiece.kind, setpiece.rect),
    }
}
