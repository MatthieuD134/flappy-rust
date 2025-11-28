//! Bird-related systems.
//!
//! This module contains all systems that control the bird entity.

use bevy::prelude::*;

use crate::components::Bird;
use crate::constants::{FLAP_STRENGTH, GRAVITY, MAX_TILT_DOWN, MAX_TILT_UP, TILT_SPEED};

/// Handles bird flapping when space is pressed.
///
/// Sets the bird's vertical velocity to the flap strength, causing it to rise.
pub fn bird_flap(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Bird>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut bird in query.iter_mut() {
            bird.velocity = FLAP_STRENGTH;
        }
    }
}

/// Applies gravity and updates bird position.
///
/// This system runs every frame to simulate physics on the bird.
pub fn bird_physics(time: Res<Time>, mut query: Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in query.iter_mut() {
        bird.velocity += GRAVITY * time.delta_secs();
        transform.translation.y += bird.velocity * time.delta_secs();
    }
}

/// Tilts the bird based on its velocity.
///
/// The bird tilts upward when rising (after a flap) and downward when falling,
/// creating a natural-looking flight animation.
pub fn bird_tilt(time: Res<Time>, mut query: Query<(&Bird, &mut Transform)>) {
    for (bird, mut transform) in query.iter_mut() {
        // Calculate target tilt based on velocity
        let target_tilt = if bird.velocity > 0.0 {
            // Going up - tilt up
            (bird.velocity / FLAP_STRENGTH) * MAX_TILT_UP
        } else {
            // Falling - tilt down proportionally
            (bird.velocity / 500.0).clamp(MAX_TILT_DOWN, 0.0)
        };

        // Smoothly interpolate to target tilt
        let current_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        let new_rotation =
            current_rotation + (target_tilt - current_rotation) * TILT_SPEED * time.delta_secs();

        transform.rotation = Quat::from_rotation_z(new_rotation);
    }
}
