use super::*;

fn test_data() -> GameData {
    GameData::load().unwrap()
}

fn move_right_input() -> ControlInput {
    right_input(false)
}

fn right_input(jump_pressed: bool) -> ControlInput {
    ControlInput {
        move_axis: 1.0,
        jump_pressed,
        crouch_held: false,
        interact_pressed: false,
        ability_pressed: false,
        retry_pressed: false,
    }
}

fn interact_input() -> ControlInput {
    ControlInput {
        move_axis: 0.0,
        jump_pressed: false,
        crouch_held: false,
        interact_pressed: true,
        ability_pressed: false,
        retry_pressed: false,
    }
}

#[test]
fn savior_choice_unlocks_level_and_counts_morality() {
    let data = test_data();
    let mut session = GameSession::new(&data);

    session.mode = SessionMode::DecisionOpen(DecisionKind::Level);
    session.apply_choice(MoralChoice::Savior);

    assert_eq!(session.savior_count(), 1);
    assert!(session.runtime.exit_unlocked);
    assert_eq!(
        session.runtime.phase,
        LevelPhase::Resolved(MoralChoice::Savior)
    );
}

#[test]
fn first_savior_route_can_reach_exit() {
    let data = test_data();
    let mut session = GameSession::new(&data);

    session.mode = SessionMode::DecisionOpen(DecisionKind::Level);
    session.apply_choice(MoralChoice::Savior);

    for _ in 0..(60 * 7) {
        session.update(&data, &data.config, 1.0 / 60.0, move_right_input());
        if session.level_index == 1 {
            assert!(matches!(session.mode, SessionMode::Playing));
            return;
        }
        assert!(
            !matches!(session.mode, SessionMode::Dismantled(_)),
            "first savior route became unwinnable: {:?}",
            session.mode
        );
    }

    panic!("first savior route did not reach the exit within 7 seconds");
}

#[test]
fn first_savior_guard_platform_is_jump_reachable() {
    let data = test_data();
    let mut session = GameSession::new(&data);

    session.mode = SessionMode::DecisionOpen(DecisionKind::Level);
    session.apply_choice(MoralChoice::Savior);
    session.runtime.guards.clear();
    session.player.x = 480.0;
    session.player.y = FLOOR_Y - PLAYER_H;
    session.player.vx = 0.0;
    session.player.vy = 0.0;
    session.player.grounded = true;

    let target = session
        .runtime
        .platforms
        .iter()
        .copied()
        .find(|platform| platform.x > 500.0 && platform.y < FLOOR_Y)
        .expect("first level should have a low scrap bridge platform");

    for frame in 0..(60 * 2) {
        session.update(&data, &data.config, 1.0 / 60.0, right_input(frame == 0));
        if session.player.grounded {
            let player = session.player_rect();
            let standing_on_target =
                horizontal_overlap(player, target) && (rect_bottom(player) - target.y).abs() < 0.5;
            if standing_on_target {
                return;
            }
        }
    }

    panic!(
        "first savior scrap bridge at y={} was not reachable from the floor",
        target.y
    );
}

#[test]
fn first_helpful_choice_can_switch_to_gremlin_at_console() {
    let data = test_data();
    let mut session = GameSession::new(&data);

    session.mode = SessionMode::DecisionOpen(DecisionKind::Level);
    assert!(session.apply_choice(MoralChoice::Savior).is_some());
    session.player.x = 184.0;
    session.player.y = FLOOR_Y - PLAYER_H;

    session.update(&data, &data.config, 1.0 / 60.0, interact_input());

    assert!(matches!(
        session.mode,
        SessionMode::DecisionOpen(DecisionKind::Level)
    ));
    assert!(!session.can_apply_choice(MoralChoice::Savior));
    assert!(session.can_apply_choice(MoralChoice::Villain));
    assert!(session.apply_choice(MoralChoice::Villain).is_some());
    assert_eq!(session.savior_count(), 0);
    assert_eq!(session.villain_count(), 1);
    assert_eq!(
        session.runtime.phase,
        LevelPhase::Resolved(MoralChoice::Villain)
    );
}

#[test]
fn first_gremlin_choice_cannot_be_undone() {
    let data = test_data();
    let mut session = GameSession::new(&data);

    session.mode = SessionMode::DecisionOpen(DecisionKind::Level);
    assert!(session.apply_choice(MoralChoice::Villain).is_some());
    session.player.x = 184.0;
    session.player.y = FLOOR_Y - PLAYER_H;

    session.update(&data, &data.config, 1.0 / 60.0, interact_input());
    assert!(matches!(session.mode, SessionMode::Playing));
    assert!(!session.can_apply_choice(MoralChoice::Savior));

    session.mode = SessionMode::DecisionOpen(DecisionKind::Level);
    assert!(session.apply_choice(MoralChoice::Savior).is_none());
    assert_eq!(session.savior_count(), 0);
    assert_eq!(session.villain_count(), 1);
    assert_eq!(
        session.runtime.phase,
        LevelPhase::Resolved(MoralChoice::Villain)
    );
}

#[test]
fn villain_majority_routes_final_boss_to_captain() {
    let choices = vec![
        MoralChoice::Villain,
        MoralChoice::Villain,
        MoralChoice::Villain,
        MoralChoice::Villain,
        MoralChoice::Villain,
        MoralChoice::Villain,
    ];
    let runtime = build_level(19, &choices);

    assert_eq!(runtime.phase, LevelPhase::Final);
    assert!(runtime.ambience.turret_hacked);
    assert!(runtime.ambience.sparks);
    assert_eq!(runtime.boss.unwrap().kind, BossKind::Captain);
}

#[test]
fn helpful_tie_routes_final_boss_to_ai() {
    let mut choices = Vec::new();
    for index in 0..19 {
        choices.push(if index % 2 == 0 {
            MoralChoice::Savior
        } else {
            MoralChoice::Villain
        });
    }
    let runtime = build_level(19, &choices);

    assert_eq!(runtime.phase, LevelPhase::Final);
    assert_eq!(runtime.boss.unwrap().kind, BossKind::CentralAi);
}

#[test]
fn reachable_endings_include_ship_explosion() {
    assert!(EndingKind::AiDefeated.body().contains("ship explodes"));
    assert!(EndingKind::CaptainDefeated.body().contains("ship explodes"));
}
