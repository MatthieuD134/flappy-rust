//! Setup system for initializing the game world.
//!
//! This module contains the startup system that creates all initial entities.

use bevy::prelude::*;
use bevy::text::{Justify, LineBreak};

use crate::components::{
    Bird, EdgeFlash, Ground, InstructionText, MainCamera, ScoreText, ScreenFlash,
};
use crate::constants::{
    BIRD_SIZE, GROUND_HEIGHT, SCORE_FLASH_BORDER_WIDTH, SCORE_FLASH_GRADIENT_STRIPS,
    SCORE_FLASH_SOLID_RATIO, WINDOW_HEIGHT, WINDOW_WIDTH,
};

/// Sets up the initial game entities.
///
/// Creates the camera, bird, ground, sky background, and UI elements.
pub fn setup(mut commands: Commands) {
    spawn_camera(&mut commands);
    spawn_bird(&mut commands);
    spawn_ground(&mut commands);
    spawn_sky(&mut commands);
    spawn_ui(&mut commands);
    spawn_screen_flash(&mut commands);
    spawn_edge_flashes(&mut commands);
}

/// Spawns the 2D camera with MainCamera marker for screen shake.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn((Camera2d, MainCamera));
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

/// Spawns the screen flash overlay for visual effects.
fn spawn_screen_flash(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0), // High z-index to be on top
        ScreenFlash,
    ));
}

/// Spawns edge flash overlays for score effect (4 edges with gradient fade).
fn spawn_edge_flashes(commands: &mut Commands) {
    let total_border = SCORE_FLASH_BORDER_WIDTH;
    let strip_width = total_border / SCORE_FLASH_GRADIENT_STRIPS as f32;

    for i in 0..SCORE_FLASH_GRADIENT_STRIPS {
        // t goes from 0.0 at edge to 1.0 at center
        let t = i as f32 / SCORE_FLASH_GRADIENT_STRIPS as f32;

        // Keep solid for the first portion, then fade with quadratic falloff
        let alpha_multiplier = if t < SCORE_FLASH_SOLID_RATIO {
            1.0 // Solid section at the edge
        } else {
            // Remap t from [SOLID_RATIO, 1.0] to [0.0, 1.0] for the fade
            let fade_t = (t - SCORE_FLASH_SOLID_RATIO) / (1.0 - SCORE_FLASH_SOLID_RATIO);
            let fade = 1.0 - fade_t;
            fade * fade // Quadratic falloff
        };

        let strip_offset = i as f32 * strip_width + strip_width / 2.0;

        // Top edge strips
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, strip_width)),
                ..default()
            },
            Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - strip_offset, 9.0),
            EdgeFlash { alpha_multiplier },
        ));

        // Bottom edge strips
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, strip_width)),
                ..default()
            },
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + strip_offset, 9.0),
            EdgeFlash { alpha_multiplier },
        ));

        // Left edge strips
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(strip_width, WINDOW_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(-WINDOW_WIDTH / 2.0 + strip_offset, 0.0, 9.0),
            EdgeFlash { alpha_multiplier },
        ));

        // Right edge strips
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(strip_width, WINDOW_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(WINDOW_WIDTH / 2.0 - strip_offset, 0.0, 9.0),
            EdgeFlash { alpha_multiplier },
        ));
    }
}
