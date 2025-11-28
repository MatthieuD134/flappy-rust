//! Pipe-related systems.
//!
//! This module contains all systems that control pipe entities.

use bevy::prelude::*;

use crate::components::{Pipe, Scored};
use crate::constants::{
    GROUND_HEIGHT, PIPE_GAP_MAX, PIPE_GAP_MIN, PIPE_SPEED, PIPE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::resources::PipeSpawnTimer;
use crate::states::GameState;
use crate::utils::rand_f32;

/// Spawns pipes at regular intervals.
///
/// Creates a pair of pipes (top and bottom) with a random gap position
/// and random gap size for variety in gameplay.
pub fn pipe_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PipeSpawnTimer>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        spawn_pipe_pair(&mut commands);
    }
}

/// Spawns a pair of pipes (top and bottom) with random gap.
fn spawn_pipe_pair(commands: &mut Commands) {
    // Random gap size between min and max
    let pipe_gap = PIPE_GAP_MIN + rand_f32() * (PIPE_GAP_MAX - PIPE_GAP_MIN);

    // Random gap position (vertical center of the gap)
    let gap_y = (rand_f32() - 0.5) * (WINDOW_HEIGHT - GROUND_HEIGHT - pipe_gap - 100.0);

    let spawn_x = WINDOW_WIDTH / 2.0 + PIPE_WIDTH;

    // Top pipe (green rectangle)
    let top_pipe_height = WINDOW_HEIGHT / 2.0 - gap_y - pipe_gap / 2.0;
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.2),
            custom_size: Some(Vec2::new(PIPE_WIDTH, top_pipe_height)),
            ..default()
        },
        Transform::from_xyz(spawn_x, WINDOW_HEIGHT / 2.0 - top_pipe_height / 2.0, 0.0),
        Pipe,
    ));

    // Bottom pipe (green rectangle)
    let bottom_pipe_height = WINDOW_HEIGHT / 2.0 + gap_y - pipe_gap / 2.0 - GROUND_HEIGHT;
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.7, 0.2),
            custom_size: Some(Vec2::new(PIPE_WIDTH, bottom_pipe_height)),
            ..default()
        },
        Transform::from_xyz(
            spawn_x,
            -WINDOW_HEIGHT / 2.0 + GROUND_HEIGHT + bottom_pipe_height / 2.0,
            0.0,
        ),
        Pipe,
        Scored(false),
    ));
}

/// Moves pipes from right to left and despawns them when off-screen.
///
/// This creates the scrolling effect of the game world.
pub fn pipe_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Pipe>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= PIPE_SPEED * time.delta_secs();

        // Despawn pipes that are off-screen
        if transform.translation.x < -WINDOW_WIDTH / 2.0 - PIPE_WIDTH {
            commands.entity(entity).despawn();
        }
    }
}
