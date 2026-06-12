//! Sentience: a morality-driven stealth puzzle-platformer.

use macroquad::prelude::*;

mod data;
mod entities;
mod game;
mod geometry;
mod levels;
mod progression;
mod render_textures;
#[cfg(test)]
mod route_tests;
mod state;
mod ui;
mod world_render;

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
