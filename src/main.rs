//! Sentience: a morality-driven stealth puzzle-platformer.

use macroquad::prelude::*;

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
    Conf {
        window_title: "Sentience".to_owned(),
        window_width: ui::LOGICAL_WIDTH as i32,
        window_height: ui::LOGICAL_HEIGHT as i32,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
