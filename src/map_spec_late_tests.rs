//! Later map requirements from MAPS_AND_CHOICES.md.

use crate::geometry::FLOOR_Y;
use crate::levels::build_level;
use crate::state::{GuardKind, MoralChoice, SetpieceKind};

#[test]
fn map_ten_matches_badge_printer_checkpoint_spec() {
    let unresolved = build_level(9, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::QueueScannerCover));
    assert!(has_setpiece(&unresolved, SetpieceKind::BadgePrinter));
    assert!(has_setpiece(&unresolved, SetpieceKind::SecurityOffice));
    assert!(has_setpiece(&unresolved, SetpieceKind::MaintenanceBypass));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::BadgeGate),
        3,
        "map 10 should have red, blue, and yellow badge gates"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 10 should use scanner gates, office overlook, and bypass geometry"
    );
    assert_eq!(
        unresolved.crates.len(),
        1,
        "map 10 should include a movable checkpoint crate or badge cart"
    );
    assert!(
        unresolved
            .crates
            .iter()
            .any(|crate_state| crate_state.marked),
        "map 10's crate should be a marked route object"
    );
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 10 should start with two gate guards and one supervisor"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.y < FLOOR_Y - 80.0),
        "map 10 should include an upper office supervisor"
    );

    let helpful = build_level(9, &vec![MoralChoice::Savior; 10]);
    assert!(has_setpiece(&helpful, SetpieceKind::ScannerGate));
    assert_eq!(
        helpful.guards.len(),
        3,
        "helpful badge printing should add two responders and one scanner device"
    );
    assert!(
        helpful
            .guards
            .iter()
            .any(|guard| guard.kind == GuardKind::Turret),
        "helpful badge route should restore a scanner security device"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 100.0,
        "helpful checkpoint exit should use the upper/authorized route"
    );

    let gremlin = build_level(9, &vec![MoralChoice::Villain; 10]);
    assert!(has_setpiece(&gremlin, SetpieceKind::GateJam));
    assert!(has_setpiece(&gremlin, SetpieceKind::WrongBadgeLoop));
    assert!(
        gremlin.ambience.sparks && gremlin.ambience.quiet && gremlin.ambience.turret_hacked,
        "gremlin wrong badges should jam scanners and scramble security"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Wrong-badge"))
            .count(),
        3,
        "gremlin checkpoint should trap three humans in badge loops"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin scanner confusion should open the maintenance bypass exit"
    );
}

#[test]
fn map_eleven_matches_drone_bay_sorting_floor_spec() {
    let unresolved = build_level(10, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::DroneChargingPad));
    assert!(has_setpiece(&unresolved, SetpieceKind::DroneRail));
    assert!(has_setpiece(&unresolved, SetpieceKind::DispatchTower));
    assert!(has_setpiece(&unresolved, SetpieceKind::DroneServicedHatch));
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 11 should use lower pads, overhead rails, tower, and hatch route"
    );
    assert_eq!(
        unresolved.crates.len(),
        2,
        "map 11 should include movable crates on the sorting floor"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Dormant drone") && !guard.active)
            .count()
            >= 2,
        "map 11 should begin with two dormant drones"
    );

    let helpful = build_level(10, &vec![MoralChoice::Savior; 11]);
    assert!(has_setpiece(&helpful, SetpieceKind::RescueDronePath));
    assert_eq!(
        helpful.guards.len(),
        5,
        "helpful drone rescue mode should add three drones and two callout guards"
    );
    assert_eq!(
        helpful
            .guards
            .iter()
            .filter(|guard| guard.kind == GuardKind::Turret)
            .count(),
        3,
        "helpful drone bay should activate three drone devices"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 100.0,
        "map 11 exit should sit behind the drone-serviced hatch"
    );

    let gremlin = build_level(10, &vec![MoralChoice::Villain; 11]);
    assert!(has_setpiece(&gremlin, SetpieceKind::EnthusiasticDroneCarry));
    assert!(
        gremlin.ambience.gravity_off,
        "gremlin drone assistance should enable floating carry routes"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Drone-delivered") && !guard.active)
            .count(),
        3,
        "gremlin drone bay should carry three humans to harmless drop-offs"
    );
}

#[test]
fn map_twelve_matches_brig_door_puzzle_spec() {
    let unresolved = build_level(11, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::PrisonerWalkway));
    assert!(has_setpiece(&unresolved, SetpieceKind::DoorControlRoom));
    assert!(has_setpiece(&unresolved, SetpieceKind::EvidenceLock));
    assert!(has_setpiece(
        &unresolved,
        SetpieceKind::MaintenanceCrawlspace
    ));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::BrigCellDoor),
        3,
        "map 12 should have three sequenced cell doors"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 12 should use stacked cells, crawl-control room, and upper walkway"
    );
    assert!(
        unresolved.ambience.smoke,
        "map 12 should begin with smoky detention-deck ambience"
    );
    assert_eq!(
        unresolved
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Cell prisoner") && !guard.active)
            .count(),
        3,
        "map 12 should start with three prisoners in cells"
    );

    let helpful = build_level(11, &vec![MoralChoice::Savior; 12]);
    assert_eq!(
        count_setpieces(&helpful, SetpieceKind::OneWayDoor),
        2,
        "helpful prisoner routing should open ordered one-way doors"
    );
    assert_eq!(
        helpful.guards.len(),
        2,
        "helpful brig route should add two armed resistance NPCs"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 100.0,
        "helpful brig route should finish through the evidence lock"
    );

    let gremlin = build_level(11, &vec![MoralChoice::Villain; 12]);
    assert!(has_setpiece(&gremlin, SetpieceKind::WrongWaitingRoom));
    assert!(has_setpiece(&gremlin, SetpieceKind::RevolvingDoorLoop));
    assert!(
        gremlin.ambience.quiet,
        "gremlin brig routing should quiet the corridor through congestion"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Wrong-room prisoner"))
            .count(),
        3,
        "gremlin brig route should route three prisoners into wrong waiting rooms"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin brig route should open the lower maintenance-crawl exit"
    );
}

#[test]
fn map_thirteen_matches_observatory_shutter_ring_spec() {
    let unresolved = build_level(12, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::ObservationDeck));
    assert!(has_setpiece(&unresolved, SetpieceKind::TelescopeGantry));
    assert!(has_setpiece(&unresolved, SetpieceKind::SearchlightBeam));
    assert!(has_setpiece(&unresolved, SetpieceKind::RadiationLock));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::ShutterZone),
        3,
        "map 13 should have three rotating shutter zones"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 13 should use lower deck, shutter zones, gantry, and radiation lock"
    );
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 13 should start with two scientists/tourists and one searchlight guard"
    );
    assert!(
        unresolved
            .guards
            .iter()
            .any(|guard| guard.y < FLOOR_Y - 80.0),
        "map 13 should include a gantry guard"
    );

    let helpful = build_level(12, &vec![MoralChoice::Savior; 13]);
    assert_eq!(
        helpful.guards.len(),
        2,
        "helpful shutter closure should add two bright-path guards"
    );
    assert!(
        count_setpieces(&helpful, SetpieceKind::SearchlightBeam) >= 2,
        "helpful route should intensify searchlight coverage"
    );

    let gremlin = build_level(12, &vec![MoralChoice::Villain; 13]);
    assert!(has_setpiece(&gremlin, SetpieceKind::ShadowLane));
    assert!(has_setpiece(&gremlin, SetpieceKind::GlareLane));
    assert!(
        gremlin.ambience.darkness,
        "gremlin shutter rotation should create shadow stealth lanes"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Glare-blinded"))
            .count(),
        3,
        "gremlin observatory route should blind three humans into unreliable patrols"
    );
}

#[test]
fn map_fourteen_matches_armory_safety_failure_spec() {
    let unresolved = build_level(13, &[]);
    assert!(has_setpiece(
        &unresolved,
        SetpieceKind::WeaponLockerCorridor
    ));
    assert!(has_setpiece(&unresolved, SetpieceKind::ArmoryCatwalk));
    assert!(has_setpiece(&unresolved, SetpieceKind::FoamPit));
    assert!(has_setpiece(&unresolved, SetpieceKind::StunTurretLane));
    assert!(has_setpiece(&unresolved, SetpieceKind::FoamLauncher));
    assert!(has_setpiece(&unresolved, SetpieceKind::TargetingConsole));
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 14 should use locker corridor, foam pit, upper catwalk, and turret lane"
    );
    assert_eq!(
        unresolved.crates.len(),
        1,
        "map 14 should include one decoy/cover object in the armory"
    );
    assert_eq!(
        unresolved
            .guards
            .iter()
            .filter(|guard| guard.kind == GuardKind::Turret)
            .count(),
        3,
        "map 14 should start with two stun turrets and one foam launcher"
    );

    let helpful = build_level(13, &vec![MoralChoice::Savior; 14]);
    assert_eq!(
        helpful.guards.len(),
        5,
        "helpful armory targeting should add two armed guards and three active devices"
    );
    assert_eq!(
        helpful
            .guards
            .iter()
            .filter(|guard| guard.kind == GuardKind::Turret)
            .count(),
        3,
        "helpful armory route should keep turret logic active"
    );

    let gremlin = build_level(13, &vec![MoralChoice::Villain; 14]);
    assert!(has_setpiece(&gremlin, SetpieceKind::FoamPile));
    assert!(
        gremlin.ambience.smoke && gremlin.ambience.sparks && gremlin.ambience.turret_hacked,
        "gremlin armory route should leave foam cover, sparks, and hacked targeting"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Foam-cannon victim") && !guard.active)
            .count(),
        3,
        "gremlin armory route should foam three armed humans safely"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin foam piles should open a lower shortcut exit"
    );
}

#[test]
fn map_fifteen_matches_laundry_tube_network_spec() {
    let unresolved = build_level(14, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::LaundryFloor));
    assert!(has_setpiece(&unresolved, SetpieceKind::LaundryTube));
    assert!(has_setpiece(&unresolved, SetpieceKind::SuctionBurst));
    assert!(has_setpiece(&unresolved, SetpieceKind::RollingLaundryCart));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::TubeExit),
        3,
        "map 15 should expose three tube exits"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 15 should use lower floor, middle tube route, upper service route, and exits"
    );
    assert_eq!(
        unresolved.crates.len(),
        1,
        "map 15 should include one rolling laundry cart route object"
    );
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 15 should begin with one guard and two crew members"
    );

    let helpful = build_level(14, &vec![MoralChoice::Savior; 15]);
    assert_eq!(
        helpful.guards.len(),
        2,
        "helpful laundry routing should add two guards below"
    );
    assert!(has_setpiece(&helpful, SetpieceKind::LaundryBin));
    assert!(
        helpful.exit.y < FLOOR_Y - 100.0,
        "helpful laundry route should finish through a higher tube exit"
    );

    let gremlin = build_level(14, &vec![MoralChoice::Villain; 15]);
    assert!(has_setpiece(&gremlin, SetpieceKind::UniformRetrievalTube));
    assert!(
        gremlin.ambience.gravity_off,
        "gremlin laundry route should use tube launch movement ambience"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Laundry-bin passenger") && !guard.active)
            .count(),
        3,
        "gremlin laundry route should pull three humans into padded bins"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin uniform retrieval should open a lower shortcut exit"
    );
}

#[test]
fn map_sixteen_matches_captains_quarters_permissions_spec() {
    let unresolved = build_level(15, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::TrophyHall));
    assert!(has_setpiece(&unresolved, SetpieceKind::CommandDesk));
    assert!(has_setpiece(&unresolved, SetpieceKind::EvidenceSafe));
    assert!(has_setpiece(
        &unresolved,
        SetpieceKind::PrivateEscapeCorridor
    ));
    assert!(has_setpiece(&unresolved, SetpieceKind::BriefingRoom));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::CommandLock),
        2,
        "map 16 should use command locks on connected offices"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 16 should use connected offices, safe route, upper escape corridor, and briefing room"
    );
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 16 should start with two elite guards and one briefing officer"
    );

    let helpful = build_level(15, &vec![MoralChoice::Savior; 16]);
    assert_eq!(
        helpful.guards.len(),
        3,
        "helpful command evidence should leave three humans hesitating but patrolling"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 100.0,
        "helpful command route should finish through the upper escape/evidence path"
    );

    let gremlin = build_level(15, &vec![MoralChoice::Villain; 16]);
    assert!(has_setpiece(&gremlin, SetpieceKind::FalseOrderTrail));
    assert!(
        gremlin.ambience.quiet && gremlin.ambience.turret_hacked,
        "gremlin command stamps should confuse orders and hacked permissions"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| {
                guard.name.contains("saluter")
                    || guard.name.contains("Chair")
                    || guard.name.contains("approved")
            })
            .count(),
        3,
        "gremlin command route should create three false-order behaviors"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin false orders should open a lower shortcut exit"
    );
}

#[test]
fn map_seventeen_matches_reactor_foam_descent_spec() {
    let unresolved = build_level(16, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::ReactorEntry));
    assert!(has_setpiece(&unresolved, SetpieceKind::HeatPipeMaze));
    assert!(has_setpiece(&unresolved, SetpieceKind::HeatZone));
    assert!(has_setpiece(&unresolved, SetpieceKind::ReactorWalkway));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::SteamJet),
        3,
        "map 17 should include timed steam jets"
    );
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::CoolantValve),
        2,
        "map 17 should include side coolant valves"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        5,
        "map 17 should be a descending heat/foam route"
    );
    assert_eq!(
        unresolved.crates.len(),
        1,
        "map 17 should include one coolant route object"
    );
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 17 should start with two engineers and one heat-gear guard"
    );

    let helpful = build_level(16, &vec![MoralChoice::Savior; 17]);
    assert_eq!(
        helpful.guards.len(),
        3,
        "helpful coolant flow should add three repair engineers"
    );

    let gremlin = build_level(16, &vec![MoralChoice::Villain; 17]);
    assert!(has_setpiece(&gremlin, SetpieceKind::FoamBubble));
    assert!(
        gremlin.ambience.smoke && gremlin.ambience.sparks,
        "gremlin over-foam should leave smoke/steam and sparks"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Coolant-foam") && !guard.active)
            .count(),
        3,
        "gremlin reactor route should trap three humans in foam bubbles"
    );
}

#[test]
fn map_eighteen_matches_core_lift_scheduler_spec() {
    let unresolved = build_level(17, &[]);
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::LiftLanding),
        4,
        "map 18 should have four stacked lift landings"
    );
    assert!(has_setpiece(&unresolved, SetpieceKind::CentralElevator));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::ServiceLadder),
        2,
        "map 18 should have two side service routes"
    );
    assert!(has_setpiece(&unresolved, SetpieceKind::LiftScheduler));
    assert_eq!(
        unresolved.platforms.len() - 1,
        6,
        "map 18 should use landings, central elevator, and service routes"
    );
    assert!(
        unresolved.ambience.smoke,
        "map 18 should begin with smoky shaft ambience"
    );
    assert_eq!(
        unresolved.guards.len(),
        3,
        "map 18 should begin with one guard and two elevator security units"
    );

    let helpful = build_level(17, &vec![MoralChoice::Savior; 18]);
    assert!(has_setpiece(&helpful, SetpieceKind::ElevatorSecurityUnit));
    assert_eq!(
        helpful.guards.len(),
        3,
        "helpful lift routing should add three descending elite guards"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 150.0,
        "helpful lift route should exit at the top-right landing"
    );

    let gremlin = build_level(17, &vec![MoralChoice::Villain; 18]);
    assert!(has_setpiece(&gremlin, SetpieceKind::WrongFloorDoor));
    assert!(has_setpiece(&gremlin, SetpieceKind::EmptyLiftWindow));
    assert!(
        gremlin.ambience.gravity_off && gremlin.ambience.quiet,
        "gremlin lift routing should enable empty low-gravity lift windows"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Wrong-floor") && !guard.active)
            .count(),
        3,
        "gremlin lift route should send three security units to wrong floors"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin wrong-floor routing should exit at the lower-right landing"
    );
}

#[test]
fn map_nineteen_matches_moral_firewall_spec() {
    let unresolved = build_level(18, &[]);
    assert!(has_setpiece(&unresolved, SetpieceKind::EvidenceArchive));
    assert!(has_setpiece(&unresolved, SetpieceKind::FirewallCorridor));
    assert!(has_setpiece(&unresolved, SetpieceKind::AiCamera));
    assert!(has_setpiece(&unresolved, SetpieceKind::CoreSeal));
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::MemoryDoor),
        3,
        "map 19 should have three memory doors"
    );
    assert_eq!(
        count_setpieces(&unresolved, SetpieceKind::DataCanister),
        2,
        "map 19 should expose movable evidence canisters"
    );
    assert_eq!(
        unresolved.platforms.len() - 1,
        6,
        "map 19 should use archive, memory doors, truth route, propaganda route, and core seal"
    );
    assert_eq!(
        unresolved.crates.len(),
        2,
        "map 19 should include two movable data canisters"
    );

    let helpful = build_level(18, &vec![MoralChoice::Savior; 19]);
    assert!(has_setpiece(&helpful, SetpieceKind::TruthRoute));
    assert_eq!(
        helpful.guards.len(),
        5,
        "helpful firewall route should add three humans and two memory devices"
    );
    assert!(
        helpful.exit.y < FLOOR_Y - 120.0,
        "helpful firewall route should use the upper truth route"
    );

    let mut prior_helpful_then_gremlin = vec![MoralChoice::Savior; 19];
    prior_helpful_then_gremlin[18] = MoralChoice::Villain;
    let mixed_memory = build_level(18, &prior_helpful_then_gremlin);
    assert!(
        has_setpiece(&mixed_memory, SetpieceKind::TruthRoute),
        "map 19 should remember prior helpful morality even if the current choice is gremlin"
    );
    assert!(
        has_setpiece(&mixed_memory, SetpieceKind::PropagandaRoute),
        "current gremlin firewall choice should still add propaganda route logic"
    );

    let gremlin = build_level(18, &vec![MoralChoice::Villain; 19]);
    assert!(has_setpiece(&gremlin, SetpieceKind::PropagandaRoute));
    assert!(
        gremlin.ambience.smoke
            && gremlin.ambience.darkness
            && gremlin.ambience.quiet
            && gremlin.ambience.turret_hacked,
        "gremlin firewall route should expose hacked, dark, quiet propaganda defenses"
    );
    assert_eq!(
        gremlin
            .guards
            .iter()
            .filter(|guard| guard.name.starts_with("Propaganda-confused"))
            .count(),
        2,
        "gremlin firewall route should confuse humans through false orders"
    );
    assert!(
        gremlin.exit.y > FLOOR_Y - 100.0,
        "gremlin firewall route should use the lower propaganda exit"
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
