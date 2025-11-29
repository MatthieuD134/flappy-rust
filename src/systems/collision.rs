//! Collision detection systems.
//!
//! This module handles all collision-related logic.

use bevy::prelude::*;

use crate::components::{Bird, InstructionText, Pipe};
use crate::constants::{BIRD_SIZE, GROUND_HEIGHT};
use crate::resources::{DeathEvent, GameViewport};
use crate::states::GameState;

/// Checks for collisions between bird and pipes/ground/ceiling.
///
/// Triggers game over state when a collision is detected.
pub fn check_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<(&Transform, &Sprite), With<Pipe>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut instruction_query: Query<(&mut Visibility, &mut Text2d), With<InstructionText>>,
    mut death_events: MessageWriter<DeathEvent>,
    viewport: Res<GameViewport>,
) {
    let Ok(bird_transform) = bird_query.single() else {
        return;
    };
    let bird_pos = bird_transform.translation;

    // Check ground collision
    if check_ground_collision(bird_pos, &viewport) {
        trigger_game_over(
            &mut next_state,
            &mut instruction_query,
            &mut death_events,
            bird_pos,
        );
        return;
    }

    // Check ceiling collision
    if check_ceiling_collision(bird_pos, &viewport) {
        trigger_game_over(
            &mut next_state,
            &mut instruction_query,
            &mut death_events,
            bird_pos,
        );
        return;
    }

    // Check pipe collisions
    if check_pipe_collisions(bird_pos, &pipe_query) {
        trigger_game_over(
            &mut next_state,
            &mut instruction_query,
            &mut death_events,
            bird_pos,
        );
    }
}

/// Checks if the bird has hit the ground.
fn check_ground_collision(bird_pos: Vec3, viewport: &GameViewport) -> bool {
    let ground_top = -viewport.half_height() + GROUND_HEIGHT;
    bird_pos.y - BIRD_SIZE / 2.0 <= ground_top
}

/// Checks if the bird has hit the ceiling.
fn check_ceiling_collision(bird_pos: Vec3, viewport: &GameViewport) -> bool {
    bird_pos.y + BIRD_SIZE / 2.0 >= viewport.half_height()
}

/// Checks if the bird has collided with any pipe.
fn check_pipe_collisions(
    bird_pos: Vec3,
    pipe_query: &Query<(&Transform, &Sprite), With<Pipe>>,
) -> bool {
    for (pipe_transform, sprite) in pipe_query.iter() {
        let pipe_pos = pipe_transform.translation;
        let pipe_size = sprite.custom_size.unwrap_or(Vec2::ZERO);

        if check_aabb_collision(bird_pos, BIRD_SIZE, pipe_pos, pipe_size) {
            return true;
        }
    }
    false
}

/// Performs AABB (Axis-Aligned Bounding Box) collision detection.
fn check_aabb_collision(pos_a: Vec3, size_a: f32, pos_b: Vec3, size_b: Vec2) -> bool {
    let half_a = size_a / 2.0;
    let half_b_w = size_b.x / 2.0;
    let half_b_h = size_b.y / 2.0;

    pos_a.x + half_a > pos_b.x - half_b_w
        && pos_a.x - half_a < pos_b.x + half_b_w
        && pos_a.y + half_a > pos_b.y - half_b_h
        && pos_a.y - half_a < pos_b.y + half_b_h
}

/// Triggers the game over state and updates the UI.
fn trigger_game_over(
    next_state: &mut ResMut<NextState<GameState>>,
    instruction_query: &mut Query<(&mut Visibility, &mut Text2d), With<InstructionText>>,
    death_events: &mut MessageWriter<DeathEvent>,
    bird_pos: Vec3,
) {
    next_state.set(GameState::GameOver);

    // Send death event for visual effects
    death_events.write(DeathEvent { position: bird_pos });

    let game_over_text = if cfg!(target_os = "ios") {
        "Game Over!\nTap to restart"
    } else {
        "Game Over!\nClick or press SPACE to restart"
    };

    for (mut visibility, mut text) in instruction_query.iter_mut() {
        *visibility = Visibility::Visible;
        text.0 = game_over_text.to_string();
    }
}
