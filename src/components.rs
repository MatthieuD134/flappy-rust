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

/// Marker component for the main camera.
#[derive(Component)]
pub struct MainCamera;

/// Component for screen flash overlay.
#[derive(Component)]
pub struct ScreenFlash;

/// Which edge of the screen an edge flash belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
}

/// Component for edge flash overlays (score effect).
/// The alpha_multiplier controls the gradient fade (1.0 at edge, 0.0 at center).
#[derive(Component)]
pub struct EdgeFlash {
    /// Alpha multiplier for gradient effect (0.0 to 1.0)
    pub alpha_multiplier: f32,
    /// Which edge this flash strip belongs to
    pub edge: EdgeType,
    /// Strip index (0 = outermost, increases towards center)
    pub strip_index: usize,
}

/// Component for particle effects.
#[derive(Component)]
pub struct Particle {
    /// Particle drift velocity (local movement)
    pub velocity: Vec2,
    /// World scroll velocity (moves with pipes, stops on game over)
    pub world_velocity: Vec2,
    /// Particle lifetime remaining
    pub lifetime: f32,
    /// Initial lifetime for fade calculation
    pub initial_lifetime: f32,
}

/// Marker component for flap particles (small dust/air puffs).
#[derive(Component)]
pub struct FlapParticle;

/// Marker component for death particles (explosion effect).
#[derive(Component)]
pub struct DeathParticle;

/// Component for animating score text pop effect.
#[derive(Component)]
pub struct ScorePopAnimation {
    /// Current animation time
    pub timer: f32,
    /// Total animation duration
    pub duration: f32,
}

/// Component for bird squash/stretch animation.
#[derive(Component)]
pub struct BirdSquashStretch {
    /// Current animation time
    pub timer: f32,
    /// Animation duration
    pub duration: f32,
    /// Whether this is a squash (true) or stretch (false)
    #[allow(dead_code)]
    pub is_squash: bool,
}

/// Marker component for entities that should fill the entire screen.
#[derive(Component)]
pub struct FillScreen;

/// Marker for the sky background entity.
#[derive(Component)]
pub struct Sky;
