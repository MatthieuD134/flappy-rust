//! Game state definitions.
//!
//! This module contains the game state machine.

use bevy::prelude::*;

/// Game state enumeration controlling the main game loop.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// Initial menu state, waiting for player input to start.
    #[default]
    Menu,
    /// Active gameplay state.
    Playing,
    /// Game over state, waiting for restart.
    GameOver,
}
