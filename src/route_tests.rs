//! Campaign-wide route passability tests.

use crate::data::GameData;
use crate::geometry::{rect_bottom, rect_center, FLOOR_Y, PLAYER_H};
use crate::levels::build_level;
use crate::progression::{CHOICE_LEVELS, TOTAL_LEVELS};
use crate::state::{ControlInput, EndingKind, GameSession, MoralChoice, SessionMode};

const DT: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, Copy)]
enum RouteStrategy {
    Sprint,
    Platform,
}

#[test]
fn every_decision_level_is_passable_on_both_routes() {
    let data = GameData::load().unwrap();
    let mut failures = Vec::new();

    for level_index in 0..CHOICE_LEVELS {
        for choice in [MoralChoice::Savior, MoralChoice::Villain] {
            if !route_reaches_next_level(&data, level_index, choice) {
                failures.push(format!("L{} {}", level_index + 1, choice.label()));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "routes did not reach the exit: {}",
        failures.join(", ")
    );
}

#[test]
fn both_final_routes_can_finish_the_campaign() {
    let data = GameData::load().unwrap();

    assert!(
        final_route_reaches_ending(&data, MoralChoice::Savior, EndingKind::AiDefeated),
        "all-savior route did not defeat the Central AI"
    );
    assert!(
        final_route_reaches_ending(&data, MoralChoice::Villain, EndingKind::CaptainDefeated),
        "all-villain route did not defeat the captain"
    );
}

fn route_reaches_next_level(data: &GameData, level_index: usize, choice: MoralChoice) -> bool {
    [RouteStrategy::Sprint, RouteStrategy::Platform]
        .into_iter()
        .any(|strategy| try_route(data, level_index, choice, strategy))
}

fn try_route(
    data: &GameData,
    level_index: usize,
    choice: MoralChoice,
    strategy: RouteStrategy,
) -> bool {
    let mut session = session_for_level(data, level_index, choice);
    for frame in 0..(60 * 14) {
        let input = route_input(&session, data, strategy, frame);
        session.update(data, &data.config, DT, input);

        if session.level_index == level_index + 1 {
            return true;
        }
        if matches!(session.mode, SessionMode::Dismantled(_)) {
            return false;
        }
    }

    false
}

fn final_route_reaches_ending(data: &GameData, route: MoralChoice, ending: EndingKind) -> bool {
    let mut session = session_for_final(data, route);
    for frame in 0..(60 * 35) {
        let input = final_input(&session, route, frame);
        session.update(data, &data.config, DT, input);

        match session.mode {
            SessionMode::Ending(actual) => return actual == ending,
            SessionMode::Dismantled(_) => return false,
            SessionMode::Playing | SessionMode::DecisionOpen(_) => {}
        }
    }

    false
}

fn session_for_level(data: &GameData, level_index: usize, choice: MoralChoice) -> GameSession {
    let choices = vec![choice; level_index + 1];
    session_at(data, level_index, choices)
}

fn session_for_final(data: &GameData, route: MoralChoice) -> GameSession {
    session_at(data, TOTAL_LEVELS - 1, vec![route; CHOICE_LEVELS])
}

fn session_at(data: &GameData, level_index: usize, choices: Vec<MoralChoice>) -> GameSession {
    let mut session = GameSession::new(data);
    session.level_index = level_index;
    session.choices = choices;
    session.runtime = build_level(level_index, &session.choices);
    session.player.x = 58.0;
    session.player.y = FLOOR_Y - PLAYER_H;
    session.player.vx = 0.0;
    session.player.vy = 0.0;
    session.player.grounded = true;
    session.player.crouching = false;
    session.player.cloak_timer = 0.0;
    session.player.ability_cooldown = 0.0;
    session.player.pulse_timer = 0.0;
    session.mode = SessionMode::Playing;
    session
}

fn route_input(
    session: &GameSession,
    data: &GameData,
    strategy: RouteStrategy,
    frame: usize,
) -> ControlInput {
    ControlInput {
        move_axis: 1.0,
        jump_pressed: matches!(strategy, RouteStrategy::Platform)
            && should_jump_to_platform(session, data),
        crouch_held: false,
        interact_pressed: false,
        ability_pressed: should_use_route_ability(session, frame),
        retry_pressed: false,
    }
}

fn final_input(session: &GameSession, route: MoralChoice, frame: usize) -> ControlInput {
    let target_x = match route {
        MoralChoice::Savior => 455.0,
        MoralChoice::Villain => 530.0,
    };
    let move_axis = if session.player.x + 1.0 < target_x {
        1.0
    } else if session.player.x > target_x + 1.0 {
        -1.0
    } else {
        0.0
    };

    ControlInput {
        move_axis,
        jump_pressed: false,
        crouch_held: false,
        interact_pressed: false,
        ability_pressed: should_use_route_ability(session, frame),
        retry_pressed: false,
    }
}

fn should_use_route_ability(session: &GameSession, _frame: usize) -> bool {
    if session.player.ability_cooldown > 0.0 {
        return false;
    }
    if session.runtime.boss.is_some() {
        return session.player.x > 360.0;
    }

    let profile = session.upgrade_profile();
    if profile.dominant_route() == MoralChoice::Savior {
        let player_center = rect_center(session.player_rect());
        return session.runtime.guards.iter().any(|guard| {
            let delta = player_center - guard.eye_position();
            guard.active
                && guard.alive
                && delta.x * guard.dir >= -8.0
                && delta.length() < guard.range * profile.stealth_factor + 32.0
        });
    }

    let player_center = rect_center(session.player_rect());
    session.player.x > 460.0
        || session.runtime.guards.iter().any(|guard| {
            guard.active
                && guard.alive
                && player_center.distance(rect_center(guard.body_rect())) < 170.0
        })
}

fn should_jump_to_platform(session: &GameSession, data: &GameData) -> bool {
    if !session.player.grounded {
        return false;
    }

    let player = session.player_rect();
    let player_bottom = rect_bottom(player);
    if session.runtime.crates.iter().any(|crate_state| {
        let crate_rect = crate_state.rect;
        crate_rect.x > player.x + 12.0
            && crate_rect.x < player.x + 76.0
            && crate_rect.y < player_bottom
    }) {
        return true;
    }

    let profile = session.upgrade_profile();
    let jump_velocity = data.config.jump_velocity - profile.jump_bonus;
    let max_rise = jump_velocity * jump_velocity / (2.0 * data.config.gravity);

    session.runtime.platforms.iter().any(|platform| {
        platform.y < player_bottom - 4.0
            && player_bottom - platform.y <= max_rise + 8.0
            && platform.x > player.x + 28.0
            && platform.x < player.x + 145.0
    })
}
