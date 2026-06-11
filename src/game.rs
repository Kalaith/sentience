//! High-level game loop, session events, and toolkit integration.

use crate::data::GameData;
use crate::state::{
    migrate_save_value, ControlInput, EndingKind, GameSession, MoralChoice, SaveData, SessionEvent,
};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    delete_slot, get_save_slots, load_from_slot_with_migration, save_to_slot_with_version,
    slot_exists,
};
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, dark, end_virtual_ui_frame};

pub struct Game {
    data: GameData,
    session: GameSession,
    assets: AssetManager,
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    save_exists: bool,
    save_slots: Vec<String>,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().unwrap_or_else(|err| {
            panic!("Sentience embedded data failed to load: {}", err);
        });

        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, Color::new(0.55, 0.8, 1.0, 1.0));
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        let _ = assets.load_asset_pack("assets.zip").await;
        let loaded_assets = assets.load_texture_configs(&data.texture_manifest).await;

        let mut notifications = NotificationManager::new();
        notifications.info(format!(
            "Signal acquired. {} texture manifest entries loaded.",
            loaded_assets
        ));

        let session = GameSession::new(&data);
        let mut game = Self {
            data,
            session,
            assets,
            notifications,
            events: EventBus::new(),
            save_exists: false,
            save_slots: Vec::new(),
        };
        game.refresh_save_state();
        game
    }

    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);

        if is_key_pressed(KeyCode::Escape) {
            self.events.push(UiAction::CloseDecision);
        }

        let input = capture_controls();
        let session_events = self
            .session
            .update(&self.data, &self.data.config, dt, input);
        for event in session_events {
            self.handle_session_event(event);
        }

        if is_key_pressed(KeyCode::P) {
            self.events.push(UiAction::Save);
        }
        if is_key_pressed(KeyCode::O) {
            self.events.push(UiAction::Load);
        }

        let actions: Vec<UiAction> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);

        let virtual_ui = begin_virtual_ui_frame(ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let ctx = UiContext {
            data: &self.data,
            session: &self.session,
            save_exists: self.save_exists,
            save_slots: &self.save_slots,
            loaded_assets: self.assets.len(),
            ui: &virtual_ui,
        };

        let actions = ui::draw_game_ui(ctx);
        end_virtual_ui_frame();

        for action in actions {
            self.events.push(action);
        }

        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::BottomRight,
                ..Default::default()
            });
    }

    fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::NewCampaign => {
                self.session.restart_campaign(&self.data);
                self.notifications.info("Campaign restarted");
            }
            UiAction::RetryLevel => {
                self.session.retry_level();
                self.notifications.info("Level reset");
            }
            UiAction::CloseDecision => self.session.close_decision(),
            UiAction::Save => self.save_game(),
            UiAction::Load => self.load_game(),
            UiAction::DeleteSave => self.delete_save(),
            UiAction::ApplyChoice(choice) => {
                if let Some(event) = self.session.apply_choice(choice) {
                    self.handle_session_event(event);
                }
            }
        }
    }

    fn handle_session_event(&mut self, event: SessionEvent) {
        match event {
            SessionEvent::Caught(reason) => {
                self.notifications.danger(format!("Dismantled: {}", reason));
            }
            SessionEvent::ChoiceApplied(choice) => match choice {
                MoralChoice::Savior => self
                    .notifications
                    .warning("Savior route engaged. Crew threat increased."),
                MoralChoice::Villain => self
                    .notifications
                    .success("Villain route engaged. Crew threat reduced."),
            },
            SessionEvent::LevelChanged(index) => {
                if let Some(level) = self.data.levels.get(index) {
                    self.notifications.info(format!("Entering {}", level.title));
                }
            }
            SessionEvent::EndingReached(ending) => self.notify_ending(ending),
        }
    }

    fn notify_ending(&mut self, ending: EndingKind) {
        match ending {
            EndingKind::TragicHero => self.notifications.warning(ending.title()),
            EndingKind::VillainAlone => self.notifications.danger(ending.title()),
        }
    }

    fn save_game(&mut self) {
        let save = self.session.to_save(&self.data.config.version);
        match save_to_slot_with_version(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &save,
            &self.data.config.version,
        ) {
            Ok(()) => {
                self.notifications.success("Game saved");
                self.refresh_save_state();
            }
            Err(err) => self.notifications.danger(format!("Save failed: {}", err)),
        }
    }

    fn load_game(&mut self) {
        let loaded: Result<SaveData, String> = load_from_slot_with_migration(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &self.data.config.version,
            |version, value| migrate_save_value(version, value, &self.data),
        );

        match loaded {
            Ok(save) => {
                self.session = GameSession::from_save(save, &self.data);
                self.notifications.success("Game loaded");
                self.refresh_save_state();
            }
            Err(err) => self.notifications.warning(format!("Load failed: {}", err)),
        }
    }

    fn delete_save(&mut self) {
        match delete_slot(&self.data.config.game_name, &self.data.config.save_slot) {
            Ok(()) => {
                self.notifications.info("Deleted save slot");
                self.refresh_save_state();
            }
            Err(err) => self.notifications.danger(format!("Delete failed: {}", err)),
        }
    }

    fn refresh_save_state(&mut self) {
        self.save_exists = slot_exists(&self.data.config.game_name, &self.data.config.save_slot);
        self.save_slots = get_save_slots(&self.data.config.game_name);
    }
}

fn capture_controls() -> ControlInput {
    let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
    let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
    let move_axis = match (left, right) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0.0,
    };

    ControlInput {
        move_axis,
        jump_pressed: is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::W)
            || is_key_pressed(KeyCode::Up),
        crouch_held: is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
        interact_pressed: is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter),
        retry_pressed: is_key_pressed(KeyCode::R),
    }
}
