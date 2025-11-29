//! Setup system for initializing the game world.
//!
//! This module contains the startup system that creates all initial entities.

use bevy::camera::{OrthographicProjection, Projection, ScalingMode};
use bevy::prelude::*;
use bevy::text::{Justify, LineBreak};

use crate::components::{
    Bird, EdgeFlash, EdgeType, FillScreen, Ground, InstructionText, MainCamera, ScoreText,
    ScreenFlash, Sky,
};
use crate::constants::{
    BIRD_SIZE, GAME_HEIGHT, GROUND_HEIGHT, SCORE_FLASH_BORDER_WIDTH, SCORE_FLASH_GRADIENT_STRIPS,
    SCORE_FLASH_SOLID_RATIO,
};
use crate::resources::GameViewport;

/// Sets up the initial game entities.
///
/// Creates the camera, bird, ground, sky background, and UI elements.
pub fn setup(mut commands: Commands, viewport: Res<GameViewport>) {
    spawn_camera(&mut commands, &viewport);
    spawn_bird(&mut commands);
    spawn_ground(&mut commands, &viewport);
    spawn_sky(&mut commands, &viewport);
    spawn_ui(&mut commands);
    spawn_screen_flash(&mut commands, &viewport);
    spawn_edge_flashes(&mut commands, &viewport);
}

/// Spawns the 2D camera with MainCamera marker and proper projection.
fn spawn_camera(commands: &mut Commands, viewport: &GameViewport) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: viewport.height,
            },
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
    ));
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
fn spawn_ground(commands: &mut Commands, viewport: &GameViewport) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            // Make ground wider than viewport to handle any aspect ratio
            custom_size: Some(Vec2::new(viewport.width * 2.0, GROUND_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, -GAME_HEIGHT / 2.0 + GROUND_HEIGHT / 2.0, 0.0),
        Ground,
    ));
}

/// Spawns the sky background (light blue rectangle).
fn spawn_sky(commands: &mut Commands, viewport: &GameViewport) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.8, 1.0),
            // Make sky larger than viewport to handle any aspect ratio
            custom_size: Some(Vec2::new(viewport.width * 2.0, viewport.height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        Sky,
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
        Transform::from_xyz(0.0, GAME_HEIGHT / 2.0 - 80.0, 2.0),
        ScoreText,
    ));

    // Instruction text
    let instruction_text = if cfg!(target_os = "ios") {
        "Tap to start"
    } else {
        "Click or press SPACE to start"
    };

    commands.spawn((
        Text2d::new(instruction_text),
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
fn spawn_screen_flash(commands: &mut Commands, viewport: &GameViewport) {
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(viewport.width * 2.0, viewport.height * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0), // High z-index to be on top
        ScreenFlash,
        FillScreen,
    ));
}

/// Spawns edge flash overlays for score effect (4 edges with gradient fade).
fn spawn_edge_flashes(commands: &mut Commands, viewport: &GameViewport) {
    let total_border = SCORE_FLASH_BORDER_WIDTH;
    let num_strips = SCORE_FLASH_GRADIENT_STRIPS as usize;
    let strip_width = total_border / num_strips as f32;

    for i in 0..num_strips {
        // t goes from 0.0 at edge to 1.0 at center
        let t = i as f32 / num_strips as f32;

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
                custom_size: Some(Vec2::new(viewport.width * 2.0, strip_width)),
                ..default()
            },
            Transform::from_xyz(0.0, viewport.half_height() - strip_offset, 9.0),
            EdgeFlash {
                alpha_multiplier,
                edge: EdgeType::Top,
                strip_index: i,
            },
        ));

        // Bottom edge strips
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(viewport.width * 2.0, strip_width)),
                ..default()
            },
            Transform::from_xyz(0.0, -viewport.half_height() + strip_offset, 9.0),
            EdgeFlash {
                alpha_multiplier,
                edge: EdgeType::Bottom,
                strip_index: i,
            },
        ));

        // Left edge strips
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(strip_width, viewport.height)),
                ..default()
            },
            Transform::from_xyz(-viewport.half_width() + strip_offset, 0.0, 9.0),
            EdgeFlash {
                alpha_multiplier,
                edge: EdgeType::Left,
                strip_index: i,
            },
        ));

        // Right edge strips
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(strip_width, viewport.height)),
                ..default()
            },
            Transform::from_xyz(viewport.half_width() - strip_offset, 0.0, 9.0),
            EdgeFlash {
                alpha_multiplier,
                edge: EdgeType::Right,
                strip_index: i,
            },
        ));
    }
}
