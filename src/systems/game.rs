//! Game state management systems.
//!
//! This module handles game state transitions (menu, playing, game over).

use bevy::prelude::*;

use crate::components::{Bird, InstructionText, Pipe, ScoreText};
use crate::resources::Score;
use crate::states::GameState;

/// Handles starting the game from the menu.
///
/// Waits for the player to press SPACE to begin playing.
pub fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut instruction_query: Query<&mut Visibility, With<InstructionText>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
        for mut visibility in instruction_query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

/// Handles restarting the game after game over.
///
/// Resets all game state including bird position, pipes, and score.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn restart_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    pipe_query: Query<Entity, With<Pipe>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut text_query: Query<&mut Text2d, With<ScoreText>>,
    mut instruction_query: Query<
        (&mut Visibility, &mut Text2d),
        (With<InstructionText>, Without<ScoreText>),
    >,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        reset_bird(&mut bird_query);
        despawn_all_pipes(&mut commands, &pipe_query);
        reset_score(&mut score, &mut text_query);
        hide_instructions(&mut instruction_query);
        next_state.set(GameState::Playing);
    }
}

/// Resets the bird to its starting position and state.
fn reset_bird(bird_query: &mut Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in bird_query.iter_mut() {
        bird.velocity = 0.0;
        transform.translation = Vec3::new(-50.0, 0.0, 1.0);
        transform.rotation = Quat::IDENTITY;
    }
}

/// Removes all pipe entities from the world.
fn despawn_all_pipes(commands: &mut Commands, pipe_query: &Query<Entity, With<Pipe>>) {
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Resets the score to zero and updates the display.
fn reset_score(score: &mut ResMut<Score>, text_query: &mut Query<&mut Text2d, With<ScoreText>>) {
    score.reset();
    for mut text in text_query.iter_mut() {
        text.0 = "0".to_string();
    }
}

/// Hides the instruction text.
#[allow(clippy::type_complexity)]
fn hide_instructions(
    instruction_query: &mut Query<
        (&mut Visibility, &mut Text2d),
        (With<InstructionText>, Without<ScoreText>),
    >,
) {
    for (mut visibility, _) in instruction_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}
