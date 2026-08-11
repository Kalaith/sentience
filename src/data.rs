//! Embedded campaign data and asset manifests.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{load_embedded_json, load_embedded_json_labeled};
use serde::{Deserialize, Serialize};

const GAME_CONFIG_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/game_config.json");
const LEVELS_JSON: &str = macroquad_toolkit::include_json_str!("../assets/data/levels.json");
const TEXTURE_MANIFEST_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/texture_manifest.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_name: String,
    pub display_name: String,
    pub save_slot: String,
    pub version: String,
    pub player_speed: f32,
    pub crouch_speed: f32,
    pub jump_velocity: f32,
    pub gravity: f32,
    pub air_control: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceDef {
    pub action: String,
    pub environment: String,
    pub enemy: String,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelDef {
    pub id: String,
    pub sector: String,
    pub title: String,
    pub puzzle: String,
    pub mechanic: String,
    pub tool: String,
    pub signal: String,
    pub savior: ChoiceDef,
    pub villain: ChoiceDef,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfig,
    pub levels: Vec<LevelDef>,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let levels = load_embedded_json_labeled("levels", LEVELS_JSON)?;
        let texture_manifest = load_embedded_json(TEXTURE_MANIFEST_JSON)?;

        Ok(Self {
            config,
            levels,
            texture_manifest,
        })
    }
}

#[cfg(test)]
mod tests;
