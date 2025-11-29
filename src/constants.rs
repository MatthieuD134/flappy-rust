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
pub const PIPE_SPAWN_TIME: f32 = 2.0;

/// Pipe gap difficulty scaling
/// At score 0: gap ranges from PIPE_GAP_START_MIN to PIPE_GAP_START_MAX
/// At score PIPE_GAP_SCALE_SCORE: gap is fixed at PIPE_GAP_END (max difficulty)
pub const PIPE_GAP_START_MIN: f32 = 140.0; // Easier at start
pub const PIPE_GAP_START_MAX: f32 = 160.0; // Very easy at start
pub const PIPE_GAP_END: f32 = 110.0; // Hard at max difficulty (min = max)
pub const PIPE_GAP_SCALE_SCORE: u32 = 20; // Score at which max difficulty is reached

/// World scroll speed (how fast pipes/ground move)
pub const WORLD_SCROLL_SPEED: f32 = 150.0;

/// Ground dimensions
pub const GROUND_HEIGHT: f32 = 50.0;

// ============================================================================
// VISUAL EFFECTS CONSTANTS
// ============================================================================

/// Screen shake effect
pub const SCREEN_SHAKE_DURATION: f32 = 0.3;
pub const SCREEN_SHAKE_INTENSITY: f32 = 8.0;
pub const SCREEN_SHAKE_FREQUENCY: f32 = 30.0;

/// Screen flash effect
pub const DEATH_FLASH_DURATION: f32 = 0.15;
pub const DEATH_FLASH_COLOR: (f32, f32, f32) = (1.0, 0.3, 0.2); // Red-ish
pub const DEATH_FLASH_ALPHA: f32 = 0.6;

pub const SCORE_FLASH_DURATION: f32 = 0.1;
pub const SCORE_FLASH_COLOR: (f32, f32, f32) = (1.0, 0.9, 0.3); // Gold
pub const SCORE_FLASH_ALPHA: f32 = 0.3;

/// Bird squash/stretch animation
pub const FLAP_SQUASH_DURATION: f32 = 0.15;
pub const FLAP_SQUASH_SCALE: f32 = 0.65; // Compress horizontally more
pub const FLAP_STRETCH_SCALE: f32 = 1.5; // Stretch vertically more

/// Score pop animation
pub const SCORE_POP_DURATION: f32 = 0.2;
pub const SCORE_POP_SCALE: f32 = 1.4;

/// Particle effects - Flap (cloud puff effect)
pub const FLAP_PARTICLE_COUNT_MIN: u32 = 3;
pub const FLAP_PARTICLE_COUNT_MAX: u32 = 6;
pub const FLAP_PARTICLE_SIZE_MIN: f32 = 10.0;
pub const FLAP_PARTICLE_SIZE_MAX: f32 = 16.0;
pub const FLAP_PARTICLE_LIFETIME: f32 = 1.5;
pub const FLAP_PARTICLE_COLOR: (f32, f32, f32) = (1.0, 1.0, 1.0); // Pure white

/// Particle effects - Death
pub const DEATH_PARTICLE_COUNT: u32 = 15;
pub const DEATH_PARTICLE_SIZE_MIN: f32 = 4.0;
pub const DEATH_PARTICLE_SIZE_MAX: f32 = 10.0;
pub const DEATH_PARTICLE_SPEED: f32 = 200.0;
pub const DEATH_PARTICLE_LIFETIME: f32 = 0.6;
pub const DEATH_PARTICLE_COLORS: [(f32, f32, f32); 3] = [
    (1.0, 0.8, 0.0), // Yellow (bird color)
    (1.0, 0.6, 0.0), // Orange
    (1.0, 0.4, 0.0), // Dark orange
];

/// Edge flash border width for score effect
pub const SCORE_FLASH_BORDER_WIDTH: f32 = 40.0;
/// Number of gradient strips for edge flash fade effect
pub const SCORE_FLASH_GRADIENT_STRIPS: u32 = 8;
/// Ratio of the edge that stays solid (0.0 to 1.0) before fading starts
pub const SCORE_FLASH_SOLID_RATIO: f32 = 0.35;
