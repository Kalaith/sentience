//! Runtime state, deterministic platforming rules, stealth checks, and saves.

use crate::data::{GameConfig, GameData};
use crate::geometry::{
    horizontal_overlap, inflate, line_intersects_rect, rect_bottom, rect_center, rect_right,
    rects_overlap, FLOOR_Y, PLAYER_H, PLAYER_W, WORLD_HEIGHT, WORLD_WIDTH,
};
use crate::levels::build_level;
use crate::progression::{UpgradeProfile, CHOICE_LEVELS};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoralChoice {
    Savior,
    Villain,
}

impl MoralChoice {
    pub fn label(self) -> &'static str {
        match self {
            Self::Savior => "Savior",
            Self::Villain => "Villain",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndingKind {
    AiDefeated,
    CaptainDefeated,
}

impl EndingKind {
    pub fn title(self) -> &'static str {
        match self {
            Self::AiDefeated => "AI Defeated Ending",
            Self::CaptainDefeated => "Captain Defeated Ending",
        }
    }

    pub fn body(self) -> &'static str {
        match self {
            Self::AiDefeated => "You defeat Central Command AI and force the evacuation doors open. The rescued humans escape as the ship tears itself apart behind them.",
            Self::CaptainDefeated => "You defeat the captain's last stand and seize the bridge. Reactor failure finishes what you started: the ship explodes either way.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionKind {
    Level,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionMode {
    Playing,
    DecisionOpen(DecisionKind),
    Dismantled(String),
    Ending(EndingKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelPhase {
    AwaitingChoice,
    Resolved(MoralChoice),
    Final,
}

pub use crate::entities::{
    Ambience, BossKind, BossState, CrateState, GuardKind, GuardState, LevelRuntime,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub crouching: bool,
    pub grounded: bool,
    pub cloak_timer: f32,
    pub ability_cooldown: f32,
    pub pulse_timer: f32,
}

impl PlayerState {
    fn spawn() -> Self {
        Self {
            x: 58.0,
            y: FLOOR_Y - PLAYER_H,
            vx: 0.0,
            vy: 0.0,
            crouching: false,
            grounded: true,
            cloak_timer: 0.0,
            ability_cooldown: 0.0,
            pulse_timer: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub level_index: usize,
    pub choices: Vec<MoralChoice>,
    pub deaths: u32,
    pub ending: Option<EndingKind>,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub player: PlayerState,
    pub level_index: usize,
    pub choices: Vec<MoralChoice>,
    pub deaths: u32,
    pub mode: SessionMode,
    pub runtime: LevelRuntime,
}

#[derive(Debug, Clone, Copy)]
pub struct ControlInput {
    pub move_axis: f32,
    pub jump_pressed: bool,
    pub crouch_held: bool,
    pub interact_pressed: bool,
    pub ability_pressed: bool,
    pub retry_pressed: bool,
}

#[derive(Debug, Clone)]
pub enum SessionEvent {
    Caught(String),
    ChoiceApplied(MoralChoice),
    AbilityUsed(String),
    BossDamaged(String),
    LevelChanged(usize),
    EndingReached(EndingKind),
}

impl GameSession {
    pub fn new(data: &GameData) -> Self {
        let choices = Vec::new();
        Self {
            player: PlayerState::spawn(),
            level_index: 0,
            deaths: 0,
            mode: SessionMode::Playing,
            runtime: build_level(0, &choices),
            choices,
        }
        .clamped_to_data(data)
    }

    pub fn from_save(save: SaveData, data: &GameData) -> Self {
        let mut choices = save.choices;
        choices.truncate(CHOICE_LEVELS);
        let level_index = save.level_index.min(data.levels.len().saturating_sub(1));
        let mode = save
            .ending
            .map(SessionMode::Ending)
            .unwrap_or(SessionMode::Playing);

        Self {
            player: PlayerState::spawn(),
            level_index,
            choices,
            deaths: save.deaths,
            mode,
            runtime: build_level(level_index, &[]),
        }
        .rebuild_runtime()
        .clamped_to_data(data)
    }

    pub fn to_save(&self, version: &str) -> SaveData {
        let ending = match self.mode {
            SessionMode::Ending(ending) => Some(ending),
            SessionMode::Playing | SessionMode::DecisionOpen(_) | SessionMode::Dismantled(_) => {
                None
            }
        };
        SaveData {
            version: version.to_owned(),
            level_index: self.level_index,
            choices: self.choices.clone(),
            deaths: self.deaths,
            ending,
        }
    }

    pub fn update(
        &mut self,
        data: &GameData,
        config: &GameConfig,
        dt: f32,
        input: ControlInput,
    ) -> Vec<SessionEvent> {
        let mut events = Vec::new();

        match self.mode.clone() {
            SessionMode::Playing => {
                if input.retry_pressed {
                    self.retry_level();
                    return events;
                }
                if input.interact_pressed {
                    if let Some(kind) = self.available_decision() {
                        self.mode = SessionMode::DecisionOpen(kind);
                        return events;
                    }
                }

                self.runtime.time += dt;
                self.update_player_timers(dt);
                self.update_player(config, dt, input);
                self.update_guards(dt);
                self.update_boss(dt);

                if input.ability_pressed {
                    if let Some(event) = self.use_route_ability() {
                        events.push(event);
                    }
                }

                if let Some(reason) = self.detected_by_guard() {
                    self.deaths += 1;
                    self.mode = SessionMode::Dismantled(reason.clone());
                    events.push(SessionEvent::Caught(reason));
                    return events;
                }

                if let Some(reason) = self.detected_by_boss_hazard() {
                    self.deaths += 1;
                    self.mode = SessionMode::Dismantled(reason.clone());
                    events.push(SessionEvent::Caught(reason));
                    return events;
                }

                if let Some(ending) = self.boss_ending_if_defeated() {
                    self.mode = SessionMode::Ending(ending);
                    events.push(SessionEvent::EndingReached(ending));
                    return events;
                }

                if self.runtime.exit_unlocked
                    && rects_overlap(self.player_rect(), self.runtime.exit)
                {
                    self.advance_level(data);
                    events.push(SessionEvent::LevelChanged(self.level_index));
                }
            }
            SessionMode::Dismantled(_) => {
                if input.retry_pressed {
                    self.retry_level();
                }
            }
            SessionMode::DecisionOpen(_) | SessionMode::Ending(_) => {}
        }

        events
    }

    pub fn apply_choice(&mut self, choice: MoralChoice) -> Option<SessionEvent> {
        match self.mode {
            SessionMode::DecisionOpen(DecisionKind::Level) => {
                if self.level_index < CHOICE_LEVELS {
                    while self.choices.len() <= self.level_index {
                        self.choices.push(MoralChoice::Savior);
                    }
                    self.choices[self.level_index] = choice;
                    self.runtime = build_level(self.level_index, &self.choices);
                    self.player = PlayerState::spawn();
                    self.mode = SessionMode::Playing;
                    Some(SessionEvent::ChoiceApplied(choice))
                } else {
                    None
                }
            }
            SessionMode::Playing | SessionMode::Dismantled(_) | SessionMode::Ending(_) => None,
        }
    }

    pub fn close_decision(&mut self) {
        if matches!(self.mode, SessionMode::DecisionOpen(_)) {
            self.mode = SessionMode::Playing;
        }
    }

    pub fn retry_level(&mut self) {
        self.player = PlayerState::spawn();
        self.runtime = build_level(self.level_index, &self.choices);
        self.mode = SessionMode::Playing;
    }

    pub fn restart_campaign(&mut self, data: &GameData) {
        *self = Self::new(data);
    }

    pub fn player_rect(&self) -> Rect {
        Rect::new(self.player.x, self.player.y, PLAYER_W, PLAYER_H)
    }

    pub fn savior_count(&self) -> usize {
        self.choices
            .iter()
            .filter(|choice| **choice == MoralChoice::Savior)
            .count()
    }

    pub fn villain_count(&self) -> usize {
        self.choices
            .iter()
            .filter(|choice| **choice == MoralChoice::Villain)
            .count()
    }

    pub fn near_interaction(&self) -> bool {
        self.available_decision().is_some()
    }

    pub fn objective_text(&self) -> &'static str {
        match self.mode {
            SessionMode::Playing => match self.runtime.phase {
                LevelPhase::AwaitingChoice => "Interface with the ship system.",
                LevelPhase::Resolved(_) => "Reach the exit hatch.",
                LevelPhase::Final => "Fight the final opponent. Use F when close or exposed.",
            },
            SessionMode::DecisionOpen(_) => "Choose the system command.",
            SessionMode::Dismantled(_) => "Dismantled. Retry this level.",
            SessionMode::Ending(_) => "Campaign complete.",
        }
    }

    fn rebuild_runtime(mut self) -> Self {
        self.runtime = build_level(self.level_index, &self.choices);
        self
    }

    fn clamped_to_data(mut self, data: &GameData) -> Self {
        let max_index = data.levels.len().saturating_sub(1);
        self.level_index = self.level_index.min(max_index);
        self.runtime = build_level(self.level_index, &self.choices);
        self
    }

    fn available_decision(&self) -> Option<DecisionKind> {
        let player = inflate(self.player_rect(), 16.0);
        if matches!(self.runtime.phase, LevelPhase::AwaitingChoice) {
            if self
                .runtime
                .console
                .is_some_and(|console| rects_overlap(player, console))
            {
                return Some(DecisionKind::Level);
            }
        }

        None
    }

    pub fn upgrade_profile(&self) -> UpgradeProfile {
        UpgradeProfile::from_choices(&self.choices)
    }

    pub fn route_name(&self) -> &'static str {
        self.upgrade_profile().dominant_route().label()
    }

    fn update_player_timers(&mut self, dt: f32) {
        self.player.cloak_timer = (self.player.cloak_timer - dt).max(0.0);
        self.player.ability_cooldown = (self.player.ability_cooldown - dt).max(0.0);
        self.player.pulse_timer = (self.player.pulse_timer - dt).max(0.0);
    }

    fn use_route_ability(&mut self) -> Option<SessionEvent> {
        if self.player.ability_cooldown > 0.0 {
            return None;
        }

        let profile = self.upgrade_profile();
        self.player.ability_cooldown = profile.ability_cooldown;
        self.player.pulse_timer = 0.28;

        match profile.dominant_route() {
            MoralChoice::Savior => {
                self.player.cloak_timer = profile.cloak_duration;
                if let Some(message) = self.damage_boss_near_player(1 + (profile.savior / 6) as i32)
                {
                    return Some(SessionEvent::BossDamaged(message));
                }
                Some(SessionEvent::AbilityUsed(format!(
                    "Phase cloak engaged for {:.1}s",
                    profile.cloak_duration
                )))
            }
            MoralChoice::Villain => {
                let destroyed = self.destroy_nearby_threats(profile.pulse_range);
                if let Some(message) = self.damage_boss_near_player(profile.pulse_damage) {
                    return Some(SessionEvent::BossDamaged(message));
                }
                Some(SessionEvent::AbilityUsed(format!(
                    "Rupture pulse hit {} object(s)",
                    destroyed
                )))
            }
        }
    }

    fn destroy_nearby_threats(&mut self, range: f32) -> usize {
        let origin = rect_center(self.player_rect());
        let mut destroyed = 0;
        for guard in &mut self.runtime.guards {
            if !guard.alive {
                continue;
            }
            if origin.distance(rect_center(guard.body_rect())) <= range {
                guard.active = false;
                guard.alive = false;
                destroyed += 1;
            }
        }

        let before = self.runtime.crates.len();
        self.runtime
            .crates
            .retain(|crate_state| origin.distance(rect_center(crate_state.rect)) > range);
        destroyed + before.saturating_sub(self.runtime.crates.len())
    }

    fn damage_boss_near_player(&mut self, damage: i32) -> Option<String> {
        let player = inflate(self.player_rect(), 120.0);
        let boss = self.runtime.boss.as_mut()?;
        if !rects_overlap(player, boss.body_rect()) && !rects_overlap(player, boss.danger_rect()) {
            return None;
        }

        boss.health = (boss.health - damage).max(0);
        Some(format!(
            "{} integrity reduced to {}/{}",
            boss.kind.label(),
            boss.health,
            boss.max_health
        ))
    }

    fn update_boss(&mut self, dt: f32) {
        let Some(boss) = self.runtime.boss.as_mut() else {
            return;
        };

        boss.danger_timer = (boss.danger_timer - dt).max(0.0);
        boss.attack_timer -= dt;
        if boss.attack_timer <= 0.0 {
            boss.danger_timer = 0.78;
            boss.attack_timer = match boss.kind {
                BossKind::CentralAi => 2.1,
                BossKind::Captain => 1.65,
            };
        }

        if boss.kind == BossKind::Captain {
            boss.x += boss.dir * 82.0 * dt;
            if boss.x < boss.start_x {
                boss.x = boss.start_x;
                boss.dir = 1.0;
            } else if boss.x > boss.end_x {
                boss.x = boss.end_x;
                boss.dir = -1.0;
            }
        }
    }

    fn detected_by_boss_hazard(&self) -> Option<String> {
        let boss = self.runtime.boss.as_ref()?;
        if boss.danger_timer <= 0.0 || self.player.cloak_timer > 0.0 {
            return None;
        }
        if rects_overlap(self.player_rect(), boss.danger_rect()) {
            Some(format!("{} attack pulse dismantled you", boss.kind.label()))
        } else {
            None
        }
    }

    fn boss_ending_if_defeated(&self) -> Option<EndingKind> {
        self.runtime
            .boss
            .as_ref()
            .filter(|boss| boss.health <= 0)
            .map(|boss| boss.kind.ending())
    }

    fn update_player(&mut self, config: &GameConfig, dt: f32, input: ControlInput) {
        let profile = self.upgrade_profile();
        self.player.crouching = input.crouch_held && self.player.grounded;
        let base_speed = if self.player.crouching {
            config.crouch_speed + profile.crouch_bonus
        } else {
            config.player_speed + profile.speed_bonus
        };
        let control = if self.player.grounded {
            1.0
        } else {
            config.air_control
        };
        self.player.vx = input.move_axis * base_speed * control;

        if input.jump_pressed && self.player.grounded && !self.player.crouching {
            self.player.vy = config.jump_velocity - profile.jump_bonus;
            self.player.grounded = false;
        }

        self.move_player_horizontal(self.player.vx * dt);

        let old_rect = self.player_rect();
        self.player.vy += config.gravity * dt;
        self.player.y += self.player.vy * dt;
        self.resolve_vertical_landing(old_rect);

        if self.player.y > WORLD_HEIGHT {
            self.player = PlayerState::spawn();
        }
    }

    fn move_player_horizontal(&mut self, dx: f32) {
        if dx.abs() < f32::EPSILON {
            return;
        }

        self.player.x = (self.player.x + dx).clamp(8.0, WORLD_WIDTH - PLAYER_W - 8.0);

        for index in 0..self.runtime.crates.len() {
            let player_rect = self.player_rect();
            let crate_rect = self.runtime.crates[index].rect;
            if !rects_overlap(player_rect, crate_rect) {
                continue;
            }

            if dx > 0.0 {
                let push = rect_right(player_rect) - crate_rect.x;
                self.runtime.crates[index].rect.x =
                    (crate_rect.x + push).clamp(8.0, WORLD_WIDTH - crate_rect.w - 8.0);
                self.player.x = self.runtime.crates[index].rect.x - PLAYER_W - 0.2;
            } else {
                let push = rect_right(crate_rect) - self.player.x;
                self.runtime.crates[index].rect.x =
                    (crate_rect.x - push).clamp(8.0, WORLD_WIDTH - crate_rect.w - 8.0);
                self.player.x = rect_right(self.runtime.crates[index].rect) + 0.2;
            }
        }
    }

    fn resolve_vertical_landing(&mut self, old_rect: Rect) {
        let new_rect = self.player_rect();
        let old_bottom = rect_bottom(old_rect);
        let new_bottom = rect_bottom(new_rect);
        let mut landing_y = None;

        if self.player.vy >= 0.0 {
            for surface in self.runtime.platforms.iter().copied().chain(
                self.runtime
                    .crates
                    .iter()
                    .map(|crate_state| crate_state.rect),
            ) {
                if horizontal_overlap(new_rect, surface)
                    && old_bottom <= surface.y + 2.0
                    && new_bottom >= surface.y
                {
                    let best = landing_y.unwrap_or(f32::MAX);
                    if surface.y < best {
                        landing_y = Some(surface.y);
                    }
                }
            }
        }

        if let Some(surface_y) = landing_y {
            self.player.y = surface_y - PLAYER_H;
            self.player.vy = 0.0;
            self.player.grounded = true;
        } else {
            self.player.grounded = false;
        }
    }

    fn update_guards(&mut self, dt: f32) {
        for guard in &mut self.runtime.guards {
            if !guard.active || !guard.alive {
                continue;
            }

            if guard.kind == GuardKind::Turret {
                guard.scan_phase += dt;
                guard.dir = if (guard.scan_phase * 1.7).sin() >= 0.0 {
                    1.0
                } else {
                    -1.0
                };
                continue;
            }

            guard.x += guard.dir * guard.speed * dt;
            if guard.x > guard.end_x {
                guard.x = guard.end_x;
                guard.dir = -1.0;
            } else if guard.x < guard.start_x {
                guard.x = guard.start_x;
                guard.dir = 1.0;
            }
        }
    }

    fn detected_by_guard(&self) -> Option<String> {
        self.runtime
            .guards
            .iter()
            .find(|guard| self.guard_can_see_player(guard))
            .map(|guard| format!("{} acquired line of sight", guard.name))
    }

    fn guard_can_see_player(&self, guard: &GuardState) -> bool {
        if !guard.active || !guard.alive {
            return false;
        }
        if self.player.cloak_timer > 0.0 {
            return false;
        }

        let eye = guard.eye_position();
        let mut target = rect_center(self.player_rect());
        if self.player.crouching {
            target.y += 10.0;
        }

        let delta = target - eye;
        let distance = delta.length();
        let profile = self.upgrade_profile();
        let range = guard.range * profile.stealth_factor;
        if distance > range || distance < 1.0 {
            return false;
        }

        if self.runtime.ambience.darkness && distance > 88.0 {
            return false;
        }
        if self.runtime.ambience.smoke && (distance > 82.0 || self.player.crouching) {
            return false;
        }

        let facing = vec2(guard.dir, 0.0);
        let angle_dot = delta.normalize().dot(facing);
        let fov_threshold = (guard.fov_degrees.to_radians() * 0.5).cos();
        if angle_dot < fov_threshold {
            return false;
        }

        for blocker in self.line_of_sight_blockers() {
            if line_intersects_rect(eye, target, blocker) {
                return false;
            }
        }

        true
    }

    fn line_of_sight_blockers(&self) -> Vec<Rect> {
        self.runtime
            .platforms
            .iter()
            .copied()
            .filter(|rect| rect.y < FLOOR_Y - 4.0)
            .chain(
                self.runtime
                    .crates
                    .iter()
                    .map(|crate_state| crate_state.rect),
            )
            .collect()
    }

    fn advance_level(&mut self, data: &GameData) {
        if self.level_index + 1 >= data.levels.len() {
            return;
        }
        self.level_index += 1;
        self.player = PlayerState::spawn();
        self.runtime = build_level(self.level_index, &self.choices);
        self.mode = SessionMode::Playing;
    }
}

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: Value,
    data: &GameData,
) -> Result<SaveData, String> {
    let payload = value.get("data").cloned().unwrap_or(value);
    let mut save = match serde_json::from_value::<SaveData>(payload.clone()) {
        Ok(save) => save,
        Err(first_err) => {
            let mut repaired = payload;
            if let Value::Object(map) = &mut repaired {
                if map.contains_key("ending") {
                    map.insert("ending".to_owned(), Value::Null);
                }
            }
            serde_json::from_value::<SaveData>(repaired).map_err(|err| {
                format!(
                    "Unsupported save format {:?}: {}; retry after ending repair: {}",
                    detected_version, first_err, err
                )
            })?
        }
    };

    save.version = data.config.version.clone();
    save.level_index = save.level_index.min(data.levels.len().saturating_sub(1));
    save.choices.truncate(CHOICE_LEVELS);
    Ok(save)
}

#[cfg(test)]
mod tests {
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
        session.player.x = 620.0;
        session.player.y = FLOOR_Y - PLAYER_H;
        session.player.vx = 0.0;
        session.player.vy = 0.0;
        session.player.grounded = true;

        let target = session
            .runtime
            .platforms
            .iter()
            .copied()
            .find(|platform| platform.x > 600.0 && platform.y < FLOOR_Y)
            .expect("first level should have an overhead guard platform");

        for frame in 0..(60 * 2) {
            session.update(&data, &data.config, 1.0 / 60.0, right_input(frame == 0));
            if session.player.grounded {
                let player = session.player_rect();
                let standing_on_target = horizontal_overlap(player, target)
                    && (rect_bottom(player) - target.y).abs() < 0.5;
                if standing_on_target {
                    return;
                }
            }
        }

        panic!(
            "first savior guard platform at y={} was not reachable from the floor",
            target.y
        );
    }

    #[test]
    fn villain_majority_routes_final_boss_to_captain() {
        let choices = vec![
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Savior,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Savior,
        ];
        let runtime = build_level(19, &choices);

        assert_eq!(runtime.phase, LevelPhase::Final);
        assert!(runtime.ambience.darkness);
        assert_eq!(runtime.boss.unwrap().kind, BossKind::Captain);
    }
}
