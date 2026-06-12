//! Map-by-map requirements from MAPS_AND_CHOICES.md.

use crate::geometry::FLOOR_Y;
use crate::levels::build_level;
use crate::state::{GuardKind, MoralChoice, SetpieceKind};

#[test]
fn map_one_matches_scrap_wake_spec() {
    let unresolved = build_level(0, &[]);
    let console = unresolved.console.expect("map 1 should have a console");
    assert!(
        console.x < 260.0,
        "map 1 console should sit beside the starting scrap heap"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        2,
        "map 1 should stay a simple two-platform tutorial"
    );
    assert!(
        unresolved.platforms[1..]
            .iter()
            .all(|platform| FLOOR_Y - platform.y <= 70.0),
        "map 1 platforms must be low enough for the starting jump"
    );
    assert!(has_setpiece(&unresolved, SetpieceKind::ScrapHeap));
    assert!(has_setpiece(&unresolved, SetpieceKind::CrouchTunnel));
    assert!(has_setpiece(&unresolved, SetpieceKind::BrokenCatwalk));
    assert!(has_setpiece(&unresolved, SetpieceKind::HangingScrap));

    let guard = unresolved
        .guards
        .first()
        .expect("map 1 needs a booted guard");
    assert_eq!(guard.name, "Booted cargo guard");
    assert!(
        guard.end_x - guard.start_x <= 180.0,
        "booted guard should have a short tutorial patrol"
    );
    assert!(
        guard.fov_degrees <= 50.0,
        "booted guard should have a narrow sight cone"
    );

    let helpful = build_level(0, &[MoralChoice::Savior]);
    assert!(has_setpiece(&helpful, SetpieceKind::DroppedScrapBridge));
    assert!(!has_setpiece(&helpful, SetpieceKind::HangingScrap));
    assert_eq!(
        helpful.platforms.len() - 1,
        2,
        "helpful gravity repair should keep map 1 to two jump platforms"
    );

    let gremlin = build_level(0, &[MoralChoice::Villain]);
    assert!(has_setpiece(&gremlin, SetpieceKind::FloatingScrapCover));
    assert!(
        gremlin.guards.iter().any(|guard| guard.floating),
        "gremlin gravity cut should leave the guard floating"
    );
}

#[test]
fn map_two_helpful_route_shows_extra_humans_on_supply_routes() {
    let runtime = build_level(1, &[MoralChoice::Savior, MoralChoice::Savior]);
    let upper_humans = runtime
        .guards
        .iter()
        .filter(|guard| guard.alive && guard.active && guard.y < FLOOR_Y - 80.0)
        .count();

    assert!(
        runtime.guards.len() >= 3,
        "map 2 helpful route promises more humans, but only {} guard(s) were built",
        runtime.guards.len()
    );
    assert!(
        upper_humans >= 2,
        "map 2 helpful route should visibly patrol upper supply routes"
    );
}

#[test]
fn map_two_matches_cargo_carousel_spec() {
    let unresolved = build_level(1, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::CargoBelt));
    assert!(has_setpiece(&unresolved, SetpieceKind::CargoLift));
    assert!(has_setpiece(&unresolved, SetpieceKind::ScannerGate));
    assert!(has_setpiece(&unresolved, SetpieceKind::DroneLane));
    assert_eq!(
        unresolved.guards.first().map(|guard| guard.name.as_str()),
        Some("Loader guard"),
        "map 2 should start with a lower loader guard"
    );
    assert_eq!(
        unresolved.crates.len(),
        2,
        "map 2 needs multiple crates for cargo routing"
    );
    assert!(
        unresolved.exit.y < FLOOR_Y - 120.0,
        "map 2 exit should sit on the upper scanner route"
    );

    let helpful = build_level(1, &[MoralChoice::Savior, MoralChoice::Savior]);
    assert!(has_setpiece(&helpful, SetpieceKind::SupplyPalletBridge));
    assert!(
        helpful
            .guards
            .iter()
            .filter(|guard| guard.y < FLOOR_Y - 60.0)
            .count()
            >= 2,
        "helpful cargo alignment should add upper supply-route humans"
    );

    let gremlin = build_level(1, &[MoralChoice::Villain, MoralChoice::Villain]);
    assert!(has_setpiece(&gremlin, SetpieceKind::MagnetizedCrateTrail));
    assert!(
        gremlin.crates.len() >= 3,
        "gremlin magnetized crates should add moving-cover clutter"
    );
}

#[test]
fn map_three_matches_suit_locker_spec() {
    let unresolved = build_level(2, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::LockerBank));
    assert!(has_setpiece(&unresolved, SetpieceKind::DeconShower));
    assert!(has_setpiece(&unresolved, SetpieceKind::SprayLift));
    assert!(has_setpiece(&unresolved, SetpieceKind::SuitRack));
    assert!(has_setpiece(
        &unresolved,
        SetpieceKind::MaintenanceCrawlspace
    ));
    assert_eq!(
        unresolved.platforms.len() - 1,
        4,
        "map 3 should use room features and suit racks, not the old three-platform template"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.name == "Stuck suit technician" && !guard.active),
        "map 3 should include the stuck suit technician before the choice"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.name == "Shower-shaft security" && guard.active),
        "map 3 should include security crossing the shower shaft"
    );

    let helpful = build_level(
        2,
        &[
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
        ],
    );
    assert_eq!(
        helpful
            .guards
            .iter()
            .filter(|guard| guard.name.contains("suit") || guard.name.contains("rack"))
            .count(),
        2,
        "helpful suit repair should add two rack-climbing crew"
    );

    let gremlin = build_level(
        2,
        &[
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
        ],
    );
    assert!(has_setpiece(&gremlin, SetpieceKind::FoamBouncePad));
    assert!(
        gremlin
            .guards
            .iter()
            .any(|guard| guard.name == "Half-suited foam guard" && guard.panicked && guard.active),
        "gremlin foam calibration should leave a half-suited moving blocker"
    );
}

#[test]
fn map_four_matches_mess_hall_rail_jam_spec() {
    let unresolved = build_level(3, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::KitchenPass));
    assert!(has_setpiece(&unresolved, SetpieceKind::DiningPit));
    assert!(has_setpiece(&unresolved, SetpieceKind::ServingRail));
    assert!(has_setpiece(&unresolved, SetpieceKind::DiningTable));
    assert!(has_setpiece(&unresolved, SetpieceKind::DishReturnRamp));
    assert!(has_setpiece(&unresolved, SetpieceKind::MealCart));
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 4 should start with two civilians and one kitchen guard"
    );
    assert!(
        unresolved.exit.y < FLOOR_Y - 120.0,
        "map 4 exit should sit on the dish-return route"
    );

    let helpful = build_level(
        3,
        &[
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
        ],
    );
    assert!(has_setpiece(&helpful, SetpieceKind::EvacuationFlow));
    assert_eq!(
        helpful.guards.len(),
        4,
        "helpful cleaning should create two guards and two evacuating civilians"
    );

    let gremlin = build_level(
        3,
        &[
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
        ],
    );
    assert!(has_setpiece(&gremlin, SetpieceKind::SlipperyGel));
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin meal-cart chaos should open a lower shortcut exit"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Sliding civilian"))
            .count(),
        3,
        "gremlin polish should add three sliding civilians"
    );
}

#[test]
fn map_five_matches_med_bay_triage_loop_spec() {
    let unresolved = build_level(4, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::ReceptionDesk));
    assert!(has_setpiece(&unresolved, SetpieceKind::TreatmentRoom));
    assert!(has_setpiece(&unresolved, SetpieceKind::BedLift));
    assert!(has_setpiece(&unresolved, SetpieceKind::RecoveryHallway));
    assert!(has_setpiece(&unresolved, SetpieceKind::TriageDoor));
    assert!(has_setpiece(&unresolved, SetpieceKind::RollingBed));
    assert!(has_setpiece(&unresolved, SetpieceKind::MedicDrone));
    assert_eq!(
        unresolved.crates.len(),
        2,
        "map 5 should use rolling beds as movable route objects"
    );
    assert!(
        unresolved.exit.y < FLOOR_Y - 120.0,
        "map 5 unresolved/helpful exit should sit behind the upper triage door"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.name == "Injured bed guard" && !guard.active),
        "map 5 should start with an injured guard on a bed"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.name == "Medic drone" && guard.kind == GuardKind::Turret),
        "map 5 should include a short-range medic drone"
    );

    let helpful = build_level(
        4,
        &[
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
        ],
    );
    assert!(has_setpiece(&helpful, SetpieceKind::AutoDoc));
    assert_eq!(
        helpful.guards.len(),
        3,
        "helpful triage should revive two guards and activate one medic drone"
    );
    assert!(
        helpful
            .guards
            .iter()
            .filter(|guard| guard.kind == GuardKind::Human && guard.active)
            .count()
            >= 2,
        "helpful triage should add recovered human patrol pressure"
    );

    let gremlin = build_level(
        4,
        &[
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
        ],
    );
    assert!(has_setpiece(&gremlin, SetpieceKind::BandageCocoon));
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin over-care should open a quiet lower recovery route"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Bandage-cocoon patient"))
            .count(),
        2,
        "gremlin over-care should wrap two humans safely"
    );
}

#[test]
fn map_six_matches_vent_stack_spec() {
    let unresolved = build_level(5, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::VentShaft));
    assert!(has_setpiece(&unresolved, SetpieceKind::MaintenanceRoom));
    assert!(has_setpiece(&unresolved, SetpieceKind::FanColumn));
    assert!(has_setpiece(&unresolved, SetpieceKind::CrosswindGap));
    assert!(
        unresolved.ambience.smoke,
        "map 6 should begin with smoke in life support"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 6 should be a vertical shaft route, not a three-platform row"
    );
    assert!(
        unresolved
            .platforms
            .iter()
            .any(|platform| platform.y <= 368.0),
        "map 6 should include high shaft platforms"
    );
    assert!(
        unresolved.exit.y < FLOOR_Y - 150.0,
        "map 6 exit should sit high in the upper-right vent route"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.name == "Lower fan engineer" && guard.panicked),
        "map 6 should start with an engineer in the lower fan room"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.name == "Upper grate guard" && guard.y < FLOOR_Y - 100.0),
        "map 6 should start with a guard on the upper grate"
    );

    let helpful = build_level(
        5,
        &[
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
        ],
    );
    assert_eq!(
        helpful.guards.len(),
        2,
        "helpful fan stabilization should add two clear-sight guards"
    );
    assert!(
        helpful.guards.iter().any(|guard| guard.y < FLOOR_Y - 100.0),
        "helpful route should include an upper vertical-sight guard"
    );

    let gremlin = build_level(
        5,
        &[
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
        ],
    );
    assert!(has_setpiece(&gremlin, SetpieceKind::SmokePocket));
    assert!(has_setpiece(&gremlin, SetpieceKind::WallNet));
    assert!(has_setpiece(&gremlin, SetpieceKind::StreamerPurge));
    assert!(
        gremlin.ambience.smoke && gremlin.ambience.gravity_off,
        "gremlin streamer purge should leave smoke cover and fan-lift movement ambience"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Wall-netted"))
            .count(),
        2,
        "gremlin route should net both life-support humans"
    );
}

#[test]
fn map_seven_matches_hydroponic_canopy_spec() {
    let unresolved = build_level(6, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::IrrigationTrench));
    assert!(has_setpiece(&unresolved, SetpieceKind::VineCanopy));
    assert!(has_setpiece(&unresolved, SetpieceKind::MaintenanceWalkway));
    assert!(has_setpiece(&unresolved, SetpieceKind::SprinklerZone));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::SeedPod),
        2,
        "map 7 should expose two optional seed-pod alternate routes"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 7 should use a trench/canopy/walkway route, not the old four-platform row"
    );
    assert!(
        unresolved
            .platforms
            .iter()
            .any(|platform| platform.y <= 360.0),
        "map 7 should include an upper maintenance walkway"
    );
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 7 should start with two botanists and one upper guard"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .filter(|guard| guard.name.contains("botanist") && guard.panicked)
            .count()
            >= 2,
        "unresolved map 7 should include two non-combat botanists"
    );

    let helpful = build_level(
        6,
        &[
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
        ],
    );
    assert!(has_setpiece(&helpful, SetpieceKind::PlantBridge));
    assert_eq!(
        helpful.guards.len(),
        3,
        "helpful oxygen repair should add three mobile crew/security NPCs"
    );
    assert!(
        helpful.guards.iter().all(|guard| guard.active),
        "helpful hydroponics humans should be mobile after oxygen recovery"
    );

    let gremlin = build_level(
        6,
        &[
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
        ],
    );
    assert!(has_setpiece(&gremlin, SetpieceKind::PlantCurtain));
    assert!(has_setpiece(&gremlin, SetpieceKind::VineTunnel));
    assert!(has_setpiece(&gremlin, SetpieceKind::TendrilGate));
    assert!(
        gremlin.ambience.smoke,
        "gremlin overgrowth should create plant-cover lanes"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Vine-tangled") && !guard.active)
            .count(),
        2,
        "gremlin hydroponics should tangle two guards safely"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin vine tunnel should open a lower shortcut exit"
    );
}

#[test]
fn map_eight_matches_airlock_tether_spec() {
    let unresolved = build_level(7, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::AirlockStaging));
    assert!(has_setpiece(&unresolved, SetpieceKind::PressureChamber));
    assert!(has_setpiece(&unresolved, SetpieceKind::PressureDoor));
    assert!(has_setpiece(
        &unresolved,
        SetpieceKind::ExteriorMaintenanceStrip
    ));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::TetherAnchor),
        4,
        "map 8 should show multiple tether anchors across the exterior strip"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 8 should use pressure chambers and tether ledges, not a three-platform row"
    );
    assert_eq!(
        unresolved.guards.first().map(|guard| guard.name.as_str()),
        Some("Suited airlock guard"),
        "map 8 should begin with one suited guard"
    );

    let helpful = build_level(
        7,
        &[
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
        ],
    );
    assert_eq!(
        count_setpieces(&helpful, SetpieceKind::PressureDoor),
        3,
        "helpful pressurization should restore multiple timed pressure doors"
    );
    assert_eq!(
        helpful.guards.len(),
        2,
        "helpful airlock repair should add two unsuited guards"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 100.0,
        "helpful airlock exit should remain on the timed pressure-door route"
    );

    let gremlin = build_level(
        7,
        &[
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
        ],
    );
    assert!(has_setpiece(&gremlin, SetpieceKind::PressureBurst));
    assert_eq!(
        count_setpieces(&gremlin, SetpieceKind::SafetyNet),
        2,
        "gremlin decompression should show two emergency safety nets"
    );
    assert!(
        gremlin.ambience.gravity_off,
        "gremlin training decompression should leave low-gravity arcs enabled"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Padded-net") && !guard.active)
            .count(),
        2,
        "gremlin airlock route should net two humans safely"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin pressure bursts should open a lower shortcut exit"
    );
}

#[test]
fn map_nine_matches_cryo_thaw_grid_spec() {
    let unresolved = build_level(8, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::FrozenAisle));
    assert!(has_setpiece(&unresolved, SetpieceKind::ColdServicePipe));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::CryoPodRoom),
        4,
        "map 9 should have four cryo pod rooms around the aisle"
    );
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::ThawSwitch),
        4,
        "map 9 should expose timed thaw controls for multiple chambers"
    );
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::SleeperPod),
        4,
        "map 9 should start with four sleeping crew pods"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 9 should use pod rooms and a cold-service pipe route"
    );
    assert!(unresolved.ambience.smoke, "map 9 should begin in cold mist");
    assert!(
        unresolved
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Sleeping pod crew") && !guard.active)
            .count()
            >= 4,
        "map 9 should represent four sleepers before thawing"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.name == "Cold-suit sliding guard" && guard.active),
        "map 9 should include a sliding cold-suit guard"
    );

    let helpful = build_level(
        8,
        &[
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
            MoralChoice::Savior,
        ],
    );
    assert_eq!(
        count_setpieces(&helpful, SetpieceKind::SleeperPod),
        3,
        "helpful thawing should visibly wake three crew from pods"
    );
    assert_eq!(
        helpful.guards.len(),
        3,
        "helpful cryo thaw should add three mobile crew"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 100.0,
        "helpful cryo route should finish through the upper cold-service path"
    );

    let gremlin = build_level(
        8,
        &[
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Villain,
        ],
    );
    assert!(has_setpiece(&gremlin, SetpieceKind::ThermalBlanketCover));
    assert!(
        gremlin.ambience.darkness && gremlin.ambience.quiet,
        "gremlin cryo routing should preserve dark, quiet cold-room cover"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Blanket-shuffling crew"))
            .count(),
        3,
        "gremlin cryo route should create three slow moving blanket covers"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin blanket shuffle should open a lower shortcut exit"
    );
}

fn has_setpiece(runtime: &crate::state::LevelRuntime, kind: SetpieceKind) -> bool {
    runtime
        .setpieces
        .iter()
        .any(|setpiece| setpiece.kind == kind)
}

fn count_setpieces(runtime: &crate::state::LevelRuntime, kind: SetpieceKind) -> usize {
    runtime
        .setpieces
        .iter()
        .filter(|setpiece| setpiece.kind == kind)
        .count()
}
