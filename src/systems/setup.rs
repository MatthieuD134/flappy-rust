//! Setup system for initializing the game world.
//!
//! This module contains the startup system that creates all initial entities.

use bevy::prelude::*;
use bevy::text::{Justify, LineBreak};

use crate::components::{Bird, Ground, InstructionText, ScoreText};
use crate::constants::{BIRD_SIZE, GROUND_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};

/// Sets up the initial game entities.
///
/// Creates the camera, bird, ground, sky background, and UI elements.
pub fn setup(mut commands: Commands) {
    spawn_camera(&mut commands);
    spawn_bird(&mut commands);
    spawn_ground(&mut commands);
    spawn_sky(&mut commands);
    spawn_ui(&mut commands);
}

/// Spawns the 2D camera.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d);
}

/// Spawns the bird entity (yellow square).
fn spawn_bird(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.0),
            custom_size: Some(Vec2::splat(BIRD_SIZE)),
            ..default()
        },
        Transform::from_xyz(-50.0, 0.0, 1.0),
        Bird::default(),
    ));
}

/// Spawns the ground entity (brown rectangle).
fn spawn_ground(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, GROUND_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + GROUND_HEIGHT / 2.0, 0.0),
        Ground,
    ));
}

/// Spawns the sky background (light blue rectangle).
fn spawn_sky(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.8, 1.0),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}

/// Spawns UI elements (score and instruction text).
fn spawn_ui(commands: &mut Commands) {
    // Score text
    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - 80.0, 2.0),
        ScoreText,
    ));

    // Instruction text
    commands.spawn((
        Text2d::new("Press SPACE to start"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new(Justify::Center, LineBreak::NoWrap),
        Transform::from_xyz(0.0, 0.0, 2.0),
        InstructionText,
    ));
}
