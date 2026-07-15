//! Runtime entities shared by simulation, level construction, and rendering.

use crate::geometry::FLOOR_Y;
use crate::state::LevelPhase;
use macroquad::prelude::*;
use macroquad_toolkit::timing::{Cooldown, Timer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardKind {
    Human,
    Elite,
    Turret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossKind {
    CentralAi,
    Captain,
}

impl BossKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::CentralAi => "Central AI",
            Self::Captain => "Captain",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuardState {
    pub name: String,
    pub kind: GuardKind,
    pub x: f32,
    pub y: f32,
    pub start_x: f32,
    pub end_x: f32,
    pub dir: f32,
    pub speed: f32,
    pub range: f32,
    pub fov_degrees: f32,
    pub active: bool,
    pub alive: bool,
    pub panicked: bool,
    pub floating: bool,
    pub scan_phase: f32,
}

impl GuardState {
    pub(crate) fn human(name: &str, x: f32, start_x: f32, end_x: f32) -> Self {
        Self {
            name: name.to_owned(),
            kind: GuardKind::Human,
            x,
            y: FLOOR_Y,
            start_x,
            end_x,
            dir: 1.0,
            speed: 70.0,
            range: 230.0,
            fov_degrees: 66.0,
            active: true,
            alive: true,
            panicked: false,
            floating: false,
            scan_phase: 0.0,
        }
    }

    pub(crate) fn elite(name: &str, x: f32, start_x: f32, end_x: f32) -> Self {
        Self::human(name, x, start_x, end_x)
            .with_speed(118.0)
            .with_detection(390.0, 78.0)
            .with_kind(GuardKind::Elite)
    }

    pub(crate) fn turret(name: &str, x: f32, y: f32) -> Self {
        Self {
            name: name.to_owned(),
            kind: GuardKind::Turret,
            x,
            y,
            start_x: x,
            end_x: x,
            dir: 1.0,
            speed: 0.0,
            range: 500.0,
            fov_degrees: 74.0,
            active: true,
            alive: true,
            panicked: false,
            floating: false,
            scan_phase: 0.0,
        }
    }

    pub(crate) fn with_kind(mut self, kind: GuardKind) -> Self {
        self.kind = kind;
        self
    }

    pub(crate) fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub(crate) fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub(crate) fn with_detection(mut self, range: f32, fov_degrees: f32) -> Self {
        self.range = range;
        self.fov_degrees = fov_degrees;
        self
    }

    pub(crate) fn inactive(mut self) -> Self {
        self.active = false;
        self
    }

    pub(crate) fn panicked(mut self) -> Self {
        self.panicked = true;
        self
    }

    pub(crate) fn floating(mut self) -> Self {
        self.active = false;
        self.floating = true;
        self
    }

    pub fn body_rect(&self) -> Rect {
        match self.kind {
            GuardKind::Turret => Rect::new(self.x - 22.0, self.y, 44.0, 24.0),
            GuardKind::Human | GuardKind::Elite => {
                Rect::new(self.x - 14.0, self.y - 44.0, 28.0, 44.0)
            }
        }
    }

    pub fn eye_position(&self) -> Vec2 {
        match self.kind {
            GuardKind::Turret => vec2(self.x, self.y + 24.0),
            GuardKind::Human | GuardKind::Elite => vec2(self.x + self.dir * 12.0, self.y - 34.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BossState {
    pub kind: BossKind,
    pub x: f32,
    pub y: f32,
    pub start_x: f32,
    pub end_x: f32,
    pub dir: f32,
    pub health: i32,
    pub max_health: i32,
    pub attack_timer: Cooldown,
    pub danger_timer: Timer,
}

impl BossState {
    pub(crate) fn central_ai() -> Self {
        Self {
            kind: BossKind::CentralAi,
            x: 560.0,
            y: 265.0,
            start_x: 560.0,
            end_x: 560.0,
            dir: 1.0,
            health: 8,
            max_health: 8,
            attack_timer: Cooldown::new_armed(1.2),
            danger_timer: Timer::new(0.0),
        }
    }

    pub(crate) fn captain() -> Self {
        Self {
            kind: BossKind::Captain,
            x: 840.0,
            y: FLOOR_Y,
            start_x: 690.0,
            end_x: 1010.0,
            dir: -1.0,
            health: 9,
            max_health: 9,
            attack_timer: Cooldown::new_armed(1.0),
            danger_timer: Timer::new(0.0),
        }
    }

    pub fn body_rect(&self) -> Rect {
        match self.kind {
            BossKind::CentralAi => Rect::new(self.x - 82.0, self.y - 82.0, 164.0, 164.0),
            BossKind::Captain => Rect::new(self.x - 22.0, self.y - 62.0, 44.0, 62.0),
        }
    }

    pub fn danger_rect(&self) -> Rect {
        match self.kind {
            BossKind::CentralAi => Rect::new(self.x - 34.0, FLOOR_Y - 270.0, 68.0, 270.0),
            BossKind::Captain => Rect::new(self.x - 110.0, FLOOR_Y - 72.0, 220.0, 72.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrateState {
    pub rect: Rect,
    pub marked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetpieceKind {
    ScrapHeap,
    BrokenCatwalk,
    CrouchTunnel,
    HangingScrap,
    DroppedScrapBridge,
    FloatingScrapCover,
    ExitLedge,
    CargoBelt,
    CargoLift,
    ScannerGate,
    DroneLane,
    SupplyPalletBridge,
    MagnetizedCrateTrail,
    LockerBank,
    DeconShower,
    SprayLift,
    SuitRack,
    MaintenanceCrawlspace,
    FoamBouncePad,
    DiningTable,
    ServingRail,
    MealCart,
    KitchenPass,
    DiningPit,
    DishReturnRamp,
    SlipperyGel,
    EvacuationFlow,
    ReceptionDesk,
    TreatmentRoom,
    AutoDoc,
    BedLift,
    RecoveryHallway,
    TriageDoor,
    RollingBed,
    BandageCocoon,
    MedicDrone,
    VentShaft,
    MaintenanceRoom,
    FanColumn,
    CrosswindGap,
    SmokePocket,
    WallNet,
    StreamerPurge,
    IrrigationTrench,
    VineCanopy,
    MaintenanceWalkway,
    SeedPod,
    PlantBridge,
    SprinklerZone,
    PlantCurtain,
    VineTunnel,
    TendrilGate,
    AirlockStaging,
    PressureChamber,
    PressureDoor,
    ExteriorMaintenanceStrip,
    TetherAnchor,
    SafetyNet,
    PressureBurst,
    FrozenAisle,
    CryoPodRoom,
    ThawSwitch,
    ColdServicePipe,
    SleeperPod,
    ThermalBlanketCover,
    QueueScannerCover,
    BadgeGate,
    SecurityOffice,
    MaintenanceBypass,
    BadgePrinter,
    GateJam,
    WrongBadgeLoop,
    DroneChargingPad,
    DroneRail,
    DispatchTower,
    DroneServicedHatch,
    RescueDronePath,
    EnthusiasticDroneCarry,
    BrigCellDoor,
    PrisonerWalkway,
    DoorControlRoom,
    EvidenceLock,
    OneWayDoor,
    WrongWaitingRoom,
    RevolvingDoorLoop,
    ObservationDeck,
    TelescopeGantry,
    ShutterZone,
    SearchlightBeam,
    RadiationLock,
    ShadowLane,
    GlareLane,
    WeaponLockerCorridor,
    ArmoryCatwalk,
    FoamPit,
    StunTurretLane,
    FoamLauncher,
    TargetingConsole,
    FoamPile,
    LaundryFloor,
    LaundryTube,
    SuctionBurst,
    TubeExit,
    RollingLaundryCart,
    LaundryBin,
    UniformRetrievalTube,
    TrophyHall,
    CommandDesk,
    EvidenceSafe,
    PrivateEscapeCorridor,
    BriefingRoom,
    CommandLock,
    FalseOrderTrail,
    ReactorEntry,
    HeatPipeMaze,
    SteamJet,
    CoolantValve,
    ReactorWalkway,
    FoamBubble,
    HeatZone,
    LiftLanding,
    CentralElevator,
    ServiceLadder,
    LiftScheduler,
    WrongFloorDoor,
    ElevatorSecurityUnit,
    EmptyLiftWindow,
    EvidenceArchive,
    DataCanister,
    MemoryDoor,
    FirewallCorridor,
    TruthRoute,
    PropagandaRoute,
    AiCamera,
    CoreSeal,
}

#[derive(Debug, Clone)]
pub struct SetpieceState {
    pub kind: SetpieceKind,
    pub rect: Rect,
}

impl SetpieceState {
    pub(crate) fn new(kind: SetpieceKind, rect: Rect) -> Self {
        Self { kind, rect }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Ambience {
    pub clean: bool,
    pub emergency: bool,
    pub smoke: bool,
    pub darkness: bool,
    pub gravity_off: bool,
    pub sparks: bool,
    pub quiet: bool,
    pub turret_hacked: bool,
}

#[derive(Debug, Clone)]
pub struct LevelRuntime {
    pub phase: LevelPhase,
    pub width: f32,
    pub platforms: Vec<Rect>,
    pub crates: Vec<CrateState>,
    pub setpieces: Vec<SetpieceState>,
    pub guards: Vec<GuardState>,
    pub console: Option<Rect>,
    pub core: Option<Rect>,
    pub boss: Option<BossState>,
    pub exit: Rect,
    pub exit_unlocked: bool,
    pub ambience: Ambience,
    pub time: f32,
}
