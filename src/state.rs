//! Runtime state, deterministic platforming rules, stealth checks, and saves.

use crate::data::{GameConfig, GameData};
use crate::geometry::{
    horizontal_overlap, inflate, line_intersects_rect, rect_bottom, rect_center, rect_right,
    rects_overlap, FLOOR_Y, PLAYER_H, PLAYER_W, WORLD_HEIGHT, WORLD_WIDTH,
};
use crate::levels::build_level;
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
    TragicHero,
    VillainAlone,
}

impl EndingKind {
    pub fn title(self) -> &'static str {
        match self {
            Self::TragicHero => "Tragic Hero Ending",
            Self::VillainAlone => "Villain Ending",
        }
    }

    pub fn body(self) -> &'static str {
        match self {
            Self::TragicHero => {
                "You disconnect Central Command AI. The ship survives, and the crew dismantles the robot they still believe went rogue."
            }
            Self::VillainAlone => {
                "You upload the extermination protocol. The crew vanishes from the manifest, and you remain awake in the void."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionKind {
    Level,
    Final,
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
    StateCheck(MoralChoice),
    Final,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardKind {
    Human,
    Elite,
    Turret,
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
    scan_phase: f32,
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

    pub(crate) fn with_detection(mut self, range: f32, fov_degrees: f32) -> Self {
        self.range = range;
        self.fov_degrees = fov_degrees;
        self
    }

    pub(crate) fn inactive(mut self) -> Self {
        self.active = false;
        self
    }

    pub(crate) fn dead(mut self) -> Self {
        self.active = false;
        self.alive = false;
        self
    }

    pub(crate) fn panicked(mut self) -> Self {
        self.panicked = true;
        self
    }

    pub(crate) fn floating(mut self, y: f32) -> Self {
        self.active = false;
        self.floating = true;
        self.y = y;
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
pub struct CrateState {
    pub rect: Rect,
    pub marked: bool,
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
    pub platforms: Vec<Rect>,
    pub crates: Vec<CrateState>,
    pub guards: Vec<GuardState>,
    pub console: Option<Rect>,
    pub core: Option<Rect>,
    pub exit: Rect,
    pub exit_unlocked: bool,
    pub ambience: Ambience,
    pub time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub crouching: bool,
    pub grounded: bool,
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
    pub retry_pressed: bool,
}

#[derive(Debug, Clone)]
pub enum SessionEvent {
    Caught(String),
    ChoiceApplied(MoralChoice),
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
        choices.truncate(6);
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
                self.update_player(config, dt, input);
                self.update_guards(dt);

                if let Some(reason) = self.detected_by_guard() {
                    self.deaths += 1;
                    self.mode = SessionMode::Dismantled(reason.clone());
                    events.push(SessionEvent::Caught(reason));
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
                if self.level_index < 6 {
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
            SessionMode::DecisionOpen(DecisionKind::Final) => {
                let ending = match choice {
                    MoralChoice::Savior => EndingKind::TragicHero,
                    MoralChoice::Villain => EndingKind::VillainAlone,
                };
                self.mode = SessionMode::Ending(ending);
                Some(SessionEvent::EndingReached(ending))
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
                LevelPhase::Resolved(_) | LevelPhase::StateCheck(_) => "Reach the exit hatch.",
                LevelPhase::Final => "Interface with the Central AI Core.",
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

        if matches!(self.runtime.phase, LevelPhase::Final) {
            if self
                .runtime
                .core
                .is_some_and(|core| rects_overlap(player, core))
            {
                return Some(DecisionKind::Final);
            }
        }

        None
    }

    fn update_player(&mut self, config: &GameConfig, dt: f32, input: ControlInput) {
        self.player.crouching = input.crouch_held && self.player.grounded;
        let base_speed = if self.player.crouching {
            config.crouch_speed
        } else {
            config.player_speed
        };
        let control = if self.player.grounded {
            1.0
        } else {
            config.air_control
        };
        self.player.vx = input.move_axis * base_speed * control;

        if input.jump_pressed && self.player.grounded && !self.player.crouching {
            self.player.vy = config.jump_velocity;
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

        let eye = guard.eye_position();
        let mut target = rect_center(self.player_rect());
        if self.player.crouching {
            target.y += 10.0;
        }

        let delta = target - eye;
        let distance = delta.length();
        if distance > guard.range || distance < 1.0 {
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
    let mut save = serde_json::from_value::<SaveData>(payload)
        .map_err(|err| format!("Unsupported save format {:?}: {}", detected_version, err))?;

    save.version = data.config.version.clone();
    save.level_index = save.level_index.min(data.levels.len().saturating_sub(1));
    save.choices.truncate(6);
    Ok(save)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> GameData {
        GameData::load().unwrap()
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
    fn villain_majority_makes_antechamber_easy_route() {
        let choices = vec![
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Savior,
            MoralChoice::Villain,
            MoralChoice::Villain,
            MoralChoice::Savior,
        ];
        let runtime = build_level(6, &choices);

        assert_eq!(runtime.phase, LevelPhase::StateCheck(MoralChoice::Villain));
        assert!(runtime.ambience.darkness);
        assert!(runtime.guards.iter().all(|guard| !guard.active));
    }
}
