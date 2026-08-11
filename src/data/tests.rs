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
