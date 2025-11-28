//! Game systems module.
//!
//! This module re-exports all game systems organized by functionality.

pub mod bird;
pub mod collision;
pub mod game;
pub mod pipes;
pub mod score;
pub mod setup;

// Re-export commonly used systems for convenient access
pub use bird::{bird_flap, bird_physics, bird_tilt};
pub use collision::check_collisions;
pub use game::{restart_game, start_game};
pub use pipes::{pipe_movement, pipe_spawner};
pub use score::update_score;
pub use setup::setup;
