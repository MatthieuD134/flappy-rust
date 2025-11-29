//! Flappy Rust - A Flappy Bird Clone
//!
//! A simple Flappy Bird clone built with Bevy game engine.

use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod states;
mod systems;
mod utils;

use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use resources::{
    DeathEvent, EdgeFlashState, FlapEvent, GameViewport, PipeSpawnTimer, Score, ScoreEvent,
    ScreenFlashState, ScreenShake,
};
use states::GameState;
use systems::{
    bird_flap, bird_physics, bird_tilt, check_collisions, initial_viewport_setup, pipe_movement,
    pipe_spawner, restart_game, setup, spawn_death_particles, spawn_flap_particles, start_game,
    trigger_bird_squash, trigger_death_effects, trigger_score_effects, trigger_score_pop,
    update_bird_squash, update_edge_flash, update_edge_flash_positions,
    update_fill_screen_entities, update_fill_width_entities, update_particles, update_score,
    update_score_pop, update_screen_flash, update_screen_shake, update_viewport,
};

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    run();
    0
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Rust".to_string(),
                // On iOS, we want to ignore the resolution and let the OS handle it
                // This ensures the window takes up the full screen
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                mode: if cfg!(target_os = "ios") {
                    bevy::window::WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary)
                } else {
                    bevy::window::WindowMode::Windowed
                },
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        // Core resources
        .init_resource::<Score>()
        .init_resource::<PipeSpawnTimer>()
        .init_resource::<GameViewport>()
        // Effect resources
        .init_resource::<ScreenShake>()
        .init_resource::<ScreenFlashState>()
        .init_resource::<EdgeFlashState>()
        // Events/Messages
        .add_message::<FlapEvent>()
        .add_message::<ScoreEvent>()
        .add_message::<DeathEvent>()
        // Startup systems
        .add_systems(Startup, (setup, initial_viewport_setup).chain())
        // Viewport update systems (always running)
        .add_systems(
            Update,
            (
                update_viewport,
                update_fill_width_entities,
                update_fill_screen_entities,
                update_edge_flash_positions,
            ),
        )
        // Update systems
        .add_systems(
            Update,
            (
                // Menu state
                start_game.run_if(in_state(GameState::Menu)),
                // Playing state - core gameplay
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
                // Playing state - visual effects (respond to events)
                (
                    spawn_flap_particles,
                    trigger_bird_squash,
                    trigger_score_pop,
                    trigger_score_effects,
                )
                    .run_if(in_state(GameState::Playing)),
                // Game over state
                restart_game.run_if(in_state(GameState::GameOver)),
                // Death effects (run on game over transition)
                (spawn_death_particles, trigger_death_effects)
                    .run_if(in_state(GameState::GameOver)),
            ),
        )
        // Always-running effect systems
        .add_systems(Update, (update_particles, update_bird_squash, update_score_pop))
        .add_systems(Update, (update_screen_shake, update_screen_flash, update_edge_flash))
        .run();
}
