//! Sentience: a morality-driven stealth puzzle-platformer.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod data;
mod entities;
mod game;
mod geometry;
mod level_guards;
mod level_guards_late;
mod level_platforms;
mod level_setpieces;
mod level_setpieces_late;
mod levels;
#[cfg(test)]
mod map_spec_late_tests;
#[cfg(test)]
mod map_spec_tests;
mod progression;
mod render_textures;
#[cfg(test)]
mod route_tests;
mod state;
mod ui;
mod world_airlock_setpieces;
mod world_command_setpieces;
mod world_core_lift_setpieces;
mod world_cryo_setpieces;
mod world_early_setpieces;
mod world_effects;
mod world_firewall_setpieces;
mod world_hydroponic_setpieces;
mod world_life_support_setpieces;
mod world_map_art;
mod world_med_setpieces;
mod world_observatory_setpieces;
mod world_reactor_setpieces;
mod world_render;
mod world_security_setpieces;
mod world_services_setpieces;
mod world_setpieces;

use game::Game;

fn window_conf() -> Conf {
    capture::capture_window_conf(
        "SENTIENCE",
        "Sentience",
        ui::LOGICAL_WIDTH as i32,
        ui::LOGICAL_HEIGHT as i32,
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // Screenshot harness: when SENTIENCE_CAPTURE_PATH is set, seed a scene,
    // simulate deterministic frames, write a PNG, and exit.
    if let Some(configs) = capture::CaptureConfig::all_from_env("SENTIENCE") {
        for config in configs {
            game.begin_capture_scene(&config.scene);
            capture::run_capture_once(&config, |dt| {
                game.update(dt);
                game.draw();
            })
            .await;
        }
        return;
    }

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
