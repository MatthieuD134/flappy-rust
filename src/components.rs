//! Component definitions for all game entities.
//!
//! This module contains all the ECS components used in the game.

use bevy::prelude::*;

/// Component for the bird/player entity.
///
/// Tracks the vertical velocity for physics simulation.
#[derive(Component)]
pub struct Bird {
    /// Current vertical velocity in pixels per second.
    pub velocity: f32,
}

impl Default for Bird {
    fn default() -> Self {
        Self { velocity: 0.0 }
    }
}

/// Marker component for pipe entities.
#[derive(Component)]
pub struct Pipe;

/// Component to track if a pipe has been scored.
///
/// Only attached to bottom pipes to avoid double-counting.
#[derive(Component)]
pub struct Scored(pub bool);

/// Marker component for the ground entity.
#[derive(Component)]
pub struct Ground;

/// Marker component for the score text UI element.
#[derive(Component)]
pub struct ScoreText;

/// Marker component for instruction/message text UI element.
#[derive(Component)]
pub struct InstructionText;
