//! Resource definitions for global game state.
//!
//! This module contains all the ECS resources used in the game.

use bevy::prelude::*;

use crate::constants::PIPE_SPAWN_TIME;

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
