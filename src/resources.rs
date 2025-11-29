//! Resource definitions for global game state.
//!
//! This module contains all the ECS resources used in the game.

use bevy::prelude::*;

use crate::constants::{DEFAULT_ASPECT_RATIO, GAME_HEIGHT, PIPE_SPAWN_TIME};

/// Resource to track the current game viewport dimensions.
/// The height is fixed at GAME_HEIGHT, width adjusts based on window aspect ratio.
#[derive(Resource)]
pub struct GameViewport {
    /// Current logical width of the game area (matches what camera shows)
    pub width: f32,
    /// Fixed logical height of the game area
    pub height: f32,
}

impl Default for GameViewport {
    fn default() -> Self {
        Self {
            width: GAME_HEIGHT * DEFAULT_ASPECT_RATIO,
            height: GAME_HEIGHT,
        }
    }
}

impl GameViewport {
    /// Updates the viewport dimensions based on window size.
    /// Maintains fixed height and adjusts width to match actual camera view.
    pub fn update_from_window(&mut self, window_width: f32, window_height: f32) {
        // Use actual aspect ratio to match what the camera shows with FixedVertical scaling
        let aspect_ratio = window_width / window_height;
        self.width = self.height * aspect_ratio;
    }

    /// Returns half the width (useful for positioning)
    pub fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    /// Returns half the height (useful for positioning)
    pub fn half_height(&self) -> f32 {
        self.height / 2.0
    }
}

/// Resource to track the player's current score.
#[derive(Resource, Default)]
pub struct Score(pub u32);

impl Score {
    /// Resets the score to zero.
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// Increments the score by one.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// Timer resource for spawning pipes at regular intervals.
#[derive(Resource)]
pub struct PipeSpawnTimer(pub Timer);

impl Default for PipeSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(PIPE_SPAWN_TIME, TimerMode::Repeating))
    }
}

// ============================================================================
// VISUAL EFFECTS RESOURCES
// ============================================================================

/// Resource for screen shake effect state.
#[derive(Resource, Default)]
pub struct ScreenShake {
    /// Remaining duration of the shake
    pub duration: f32,
    /// Current intensity of the shake
    pub intensity: f32,
    /// Elapsed time for wave calculation
    pub elapsed: f32,
}

impl ScreenShake {
    /// Triggers a new screen shake effect.
    pub fn trigger(&mut self, duration: f32, intensity: f32) {
        self.duration = duration;
        self.intensity = intensity;
        self.elapsed = 0.0;
    }

    /// Returns true if the shake effect is active.
    pub fn is_active(&self) -> bool {
        self.duration > 0.0
    }
}

/// Resource for screen flash effect state.
#[derive(Resource, Default)]
pub struct ScreenFlashState {
    /// Remaining duration of the flash
    pub duration: f32,
    /// Total duration for fade calculation
    pub total_duration: f32,
    /// Flash color (RGB)
    pub color: (f32, f32, f32),
    /// Maximum alpha value
    pub max_alpha: f32,
}

impl ScreenFlashState {
    /// Triggers a new screen flash effect.
    pub fn trigger(&mut self, duration: f32, color: (f32, f32, f32), alpha: f32) {
        self.duration = duration;
        self.total_duration = duration;
        self.color = color;
        self.max_alpha = alpha;
    }

    /// Returns true if the flash effect is active.
    pub fn is_active(&self) -> bool {
        self.duration > 0.0
    }

    /// Gets the current alpha based on remaining duration (fade out).
    pub fn current_alpha(&self) -> f32 {
        if self.total_duration <= 0.0 {
            return 0.0;
        }
        (self.duration / self.total_duration) * self.max_alpha
    }
}

/// Resource for edge flash effect state (score effect at screen edges).
#[derive(Resource, Default)]
pub struct EdgeFlashState {
    /// Remaining duration of the flash
    pub duration: f32,
    /// Total duration for fade calculation
    pub total_duration: f32,
    /// Flash color (RGB)
    pub color: (f32, f32, f32),
    /// Maximum alpha value
    pub max_alpha: f32,
}

impl EdgeFlashState {
    /// Triggers a new edge flash effect.
    pub fn trigger(&mut self, duration: f32, color: (f32, f32, f32), alpha: f32) {
        self.duration = duration;
        self.total_duration = duration;
        self.color = color;
        self.max_alpha = alpha;
    }

    /// Returns true if the flash effect is active.
    pub fn is_active(&self) -> bool {
        self.duration > 0.0
    }

    /// Gets the current alpha based on remaining duration (fade out).
    pub fn current_alpha(&self) -> f32 {
        if self.total_duration <= 0.0 {
            return 0.0;
        }
        (self.duration / self.total_duration) * self.max_alpha
    }
}

/// Message triggered when the player flaps.
#[derive(Message)]
pub struct FlapEvent {
    /// Position where the flap occurred
    pub position: Vec3,
}

/// Message triggered when the player scores.
#[derive(Message)]
pub struct ScoreEvent;

/// Message triggered when the player dies.
#[derive(Message)]
pub struct DeathEvent {
    /// Position where death occurred
    pub position: Vec3,
}
