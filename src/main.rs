//! Flappy Rust - A Flappy Bird Clone
//!
//! A simple Flappy Bird clone built with Bevy game engine.
//!
//! ## Architecture
//!
//! The game is organized into the following modules:
//! - `components` - ECS component definitions
//! - `constants` - Game configuration values
//! - `resources` - Global game state resources
//! - `states` - Game state machine
//! - `systems` - Game logic systems
//! - `utils` - Helper functions

use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod states;
mod systems;
mod utils;

use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use resources::{PipeSpawnTimer, Score};
use states::GameState;
use systems::{
    bird_flap, bird_physics, bird_tilt, check_collisions, pipe_movement, pipe_spawner,
    restart_game, setup, start_game, update_score,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Rust".to_string(),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<Score>()
        .init_resource::<PipeSpawnTimer>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                start_game.run_if(in_state(GameState::Menu)),
                (
                    bird_flap,
                    bird_physics,
                    bird_tilt,
                    pipe_movement,
                    pipe_spawner,
                    check_collisions,
                    update_score,
                )
                    .run_if(in_state(GameState::Playing)),
                restart_game.run_if(in_state(GameState::GameOver)),
            ),
        )
        .run();
}
