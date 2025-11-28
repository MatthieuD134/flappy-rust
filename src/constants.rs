//! Game constants and configuration values.
//!
//! This module contains all the tunable parameters for the game.

/// Window dimensions
pub const WINDOW_WIDTH: f32 = 400.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

/// Physics constants
pub const GRAVITY: f32 = -800.0;
pub const FLAP_STRENGTH: f32 = 350.0;

/// Bird dimensions
pub const BIRD_SIZE: f32 = 30.0;

/// Bird tilt animation
pub const MAX_TILT_UP: f32 = 0.5; // ~28 degrees up
pub const MAX_TILT_DOWN: f32 = -1.2; // ~68 degrees down
pub const TILT_SPEED: f32 = 5.0;

/// Pipe dimensions and spawning
pub const PIPE_WIDTH: f32 = 60.0;
pub const PIPE_GAP_MIN: f32 = 130.0;
pub const PIPE_GAP_MAX: f32 = 180.0;
pub const PIPE_SPEED: f32 = 150.0;
pub const PIPE_SPAWN_TIME: f32 = 2.0;

/// Ground dimensions
pub const GROUND_HEIGHT: f32 = 50.0;
