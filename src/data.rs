//! Embedded campaign data and asset manifests.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{load_embedded_json, load_embedded_json_labeled};
use serde::{Deserialize, Serialize};

const GAME_CONFIG_JSON: &str = include_str!("../assets/data/game_config.json");
const LEVELS_JSON: &str = include_str!("../assets/data/levels.json");
const TEXTURE_MANIFEST_JSON: &str = include_str!("../assets/data/texture_manifest.json");

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
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::Path;

    #[test]
    fn embedded_data_loads_locked_campaign() {
        let data = GameData::load().unwrap();

        assert_eq!(data.config.game_name, "sentience");
        assert_eq!(data.levels.len(), 20);
        assert_eq!(data.levels[0].title, "Scrap Wake");
        assert_eq!(data.levels[0].tool, "Grabber Arm");
        assert_eq!(data.levels[19].puzzle, "Final Route Payoff");
    }

    #[test]
    fn each_level_has_a_map_art_texture() {
        let data = GameData::load().unwrap();
        let texture_keys = data
            .texture_manifest
            .iter()
            .map(|texture| texture.key.as_str())
            .collect::<HashSet<_>>();

        for level in &data.levels {
            let expected_key = format!("map_{}", level.id);
            assert!(
                texture_keys.contains(expected_key.as_str()),
                "missing texture manifest key {}",
                expected_key
            );

            let texture = data
                .texture_manifest
                .iter()
                .find(|texture| texture.key == expected_key)
                .expect("texture key checked above");
            let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(&texture.path);
            assert!(
                path.exists(),
                "missing map art file for {} at {}",
                level.id,
                path.display()
            );
        }
    }
}
