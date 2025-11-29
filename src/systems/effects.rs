//! Visual effects systems.
//!
//! This module handles all visual feedback effects:
//! - Screen shake
//! - Screen flash
//! - Particle effects
//! - Score pop animation
//! - Bird squash/stretch animation

use bevy::prelude::*;

use crate::components::{
    Bird, BirdSquashStretch, DeathParticle, EdgeFlash, FlapParticle, MainCamera, Particle,
    ScorePopAnimation, ScoreText, ScreenFlash,
};
use crate::constants::{
    BIRD_SIZE, DEATH_FLASH_ALPHA, DEATH_FLASH_COLOR, DEATH_FLASH_DURATION, DEATH_PARTICLE_COLORS,
    DEATH_PARTICLE_COUNT, DEATH_PARTICLE_LIFETIME, DEATH_PARTICLE_SIZE_MAX,
    DEATH_PARTICLE_SIZE_MIN, DEATH_PARTICLE_SPEED, FLAP_PARTICLE_COLOR, FLAP_PARTICLE_COUNT_MAX,
    FLAP_PARTICLE_COUNT_MIN, FLAP_PARTICLE_LIFETIME, FLAP_PARTICLE_SIZE_MAX,
    FLAP_PARTICLE_SIZE_MIN, FLAP_SQUASH_DURATION, FLAP_SQUASH_SCALE, FLAP_STRETCH_SCALE,
    SCORE_FLASH_ALPHA, SCORE_FLASH_COLOR, SCORE_FLASH_DURATION, SCORE_POP_DURATION,
    SCORE_POP_SCALE, SCREEN_SHAKE_DURATION, SCREEN_SHAKE_FREQUENCY, SCREEN_SHAKE_INTENSITY,
    WORLD_SCROLL_SPEED,
};
use crate::resources::{
    DeathEvent, EdgeFlashState, FlapEvent, ScoreEvent, ScreenFlashState, ScreenShake,
};
use crate::states::GameState;
use crate::utils::rand_f32;

// ============================================================================
// SCREEN SHAKE SYSTEM
// ============================================================================

/// Updates the screen shake effect and applies it to the camera.
pub fn update_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if !shake.is_active() {
        return;
    }

    let dt = time.delta_secs();
    shake.elapsed += dt;
    shake.duration -= dt;

    // Calculate shake offset using sine waves for smooth motion
    let decay = (shake.duration / SCREEN_SHAKE_DURATION).max(0.0);
    let offset_x =
        (shake.elapsed * SCREEN_SHAKE_FREQUENCY).sin() * shake.intensity * decay * rand_f32();
    let offset_y =
        (shake.elapsed * SCREEN_SHAKE_FREQUENCY * 1.3).cos() * shake.intensity * decay * rand_f32();

    for mut transform in camera_query.iter_mut() {
        if shake.duration > 0.0 {
            transform.translation.x = offset_x;
            transform.translation.y = offset_y;
        } else {
            // Reset camera position when shake ends
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}

// ============================================================================
// SCREEN FLASH SYSTEM
// ============================================================================

/// Updates the screen flash overlay effect.
pub fn update_screen_flash(
    time: Res<Time>,
    mut flash_state: ResMut<ScreenFlashState>,
    mut flash_query: Query<&mut Sprite, With<ScreenFlash>>,
) {
    if !flash_state.is_active() {
        // Ensure flash is invisible when not active
        for mut sprite in flash_query.iter_mut() {
            sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
        }
        return;
    }

    flash_state.duration -= time.delta_secs();
    let alpha = flash_state.current_alpha();

    for mut sprite in flash_query.iter_mut() {
        let (r, g, b) = flash_state.color;
        sprite.color = Color::srgba(r, g, b, alpha);
    }
}

/// Updates the edge flash overlay effects (score feedback).
pub fn update_edge_flash(
    time: Res<Time>,
    mut flash_state: ResMut<EdgeFlashState>,
    mut flash_query: Query<(&mut Sprite, &EdgeFlash)>,
) {
    if !flash_state.is_active() {
        // Ensure flash is invisible when not active
        for (mut sprite, _) in flash_query.iter_mut() {
            sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
        }
        return;
    }

    flash_state.duration -= time.delta_secs();
    let base_alpha = flash_state.current_alpha();

    for (mut sprite, edge_flash) in flash_query.iter_mut() {
        let (r, g, b) = flash_state.color;
        // Apply the gradient multiplier to create fade-to-center effect
        let alpha = base_alpha * edge_flash.alpha_multiplier;
        sprite.color = Color::srgba(r, g, b, alpha);
    }
}

// ============================================================================
// PARTICLE SYSTEMS
// ============================================================================

/// Updates all particles (movement, lifetime, and cleanup).
pub fn update_particles(
    time: Res<Time>,
    game_state: Res<State<GameState>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Particle, &mut Transform)>,
) {
    let dt = time.delta_secs();
    let is_playing = *game_state.get() == GameState::Playing;

    for (entity, mut particle, mut transform) in query.iter_mut() {
        // Update lifetime
        particle.lifetime -= dt;

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Apply drift movement (always active)
        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;

        // Apply world velocity only during gameplay (stops on game over)
        if is_playing {
            transform.translation.x += particle.world_velocity.x * dt;
            transform.translation.y += particle.world_velocity.y * dt;
        }

        // Slow down drift over time (air resistance)
        particle.velocity *= 0.98;

        // Scale animation: quick grow at start, slow shrink afterward
        let life_ratio = particle.lifetime / particle.initial_lifetime;
        let grow_phase = 0.15; // First 15% of life is grow phase

        let scale = if life_ratio > (1.0 - grow_phase) {
            // Grow phase: quickly expand from 0 to 1
            let grow_progress = (1.0 - life_ratio) / grow_phase;
            // Use ease-out for snappy growth
            1.0 - (1.0 - grow_progress) * (1.0 - grow_progress)
        } else {
            // Shrink phase: slowly shrink from 1 to 0
            life_ratio / (1.0 - grow_phase)
        };

        transform.scale = Vec3::splat(scale);
    }
}

/// Spawns flap particles when the player flaps - creates a small cloud puff effect.
/// Particles spawn at the flap position and drift gently, fading away.
pub fn spawn_flap_particles(
    mut commands: Commands,
    mut flap_events: MessageReader<FlapEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in flap_events.read() {
        // Spawn position is fixed at where the flap happened
        let flap_pos = event.position;

        // Randomize particle count
        let particle_count = FLAP_PARTICLE_COUNT_MIN
            + (rand_f32() * (FLAP_PARTICLE_COUNT_MAX - FLAP_PARTICLE_COUNT_MIN + 1) as f32) as u32;

        // Random base direction for this flap's particles (all particles offset from this)
        let base_angle = rand_f32() * std::f32::consts::TAU;

        for i in 0..particle_count {
            // Spread circles evenly around the base angle with some randomness
            let angle = base_angle
                + (i as f32 / particle_count as f32) * std::f32::consts::TAU
                + (rand_f32() - 0.5) * 0.6; // Random jitter
            let offset_distance = 5.0 + rand_f32() * 8.0;

            let spawn_offset = Vec3::new(
                angle.cos() * offset_distance,
                angle.sin() * offset_distance - BIRD_SIZE * 0.3, // Slightly below center
                0.5,
            );

            // Gentle drift velocity - small outward drift for cloud effect
            let drift_speed = 10.0 + rand_f32() * 15.0;
            let velocity = Vec2::new(
                angle.cos() * drift_speed + (rand_f32() - 0.5) * 8.0,
                angle.sin() * drift_speed - 3.0, // Slight downward drift
            );

            // World velocity - moves with the world (pipes), stops on game over
            let world_velocity = Vec2::new(-WORLD_SCROLL_SPEED, 0.0);

            // Vary sizes
            let size = FLAP_PARTICLE_SIZE_MIN
                + rand_f32() * (FLAP_PARTICLE_SIZE_MAX - FLAP_PARTICLE_SIZE_MIN);

            let (r, g, b) = FLAP_PARTICLE_COLOR;

            // Create a circle mesh
            let circle = Circle::new(size / 2.0);
            let mesh_handle = meshes.add(circle);
            let material_handle =
                materials.add(ColorMaterial::from_color(Color::srgba(r, g, b, 0.7)));

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                Transform::from_translation(flap_pos + spawn_offset).with_scale(Vec3::ZERO), // Start at scale 0 for grow animation
                Particle {
                    velocity,
                    world_velocity,
                    lifetime: FLAP_PARTICLE_LIFETIME * (0.7 + rand_f32() * 0.3),
                    initial_lifetime: FLAP_PARTICLE_LIFETIME,
                },
                FlapParticle,
            ));
        }
    }
}

/// Spawns death particles when the player dies.
pub fn spawn_death_particles(mut commands: Commands, mut death_events: MessageReader<DeathEvent>) {
    for event in death_events.read() {
        let base_pos = event.position;

        for _ in 0..DEATH_PARTICLE_COUNT {
            // Random angle in all directions
            let angle = rand_f32() * std::f32::consts::TAU;
            let speed = DEATH_PARTICLE_SPEED * (0.3 + rand_f32() * 0.7);

            let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed * 1.5); // More upward momentum
            let size = DEATH_PARTICLE_SIZE_MIN
                + rand_f32() * (DEATH_PARTICLE_SIZE_MAX - DEATH_PARTICLE_SIZE_MIN);

            // Random color from death particle colors
            let color_idx = (rand_f32() * DEATH_PARTICLE_COLORS.len() as f32) as usize;
            let (r, g, b) = DEATH_PARTICLE_COLORS[color_idx.min(DEATH_PARTICLE_COLORS.len() - 1)];

            commands.spawn((
                Sprite {
                    color: Color::srgb(r, g, b),
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                Transform::from_translation(base_pos + Vec3::new(0.0, 0.0, 2.0)),
                Particle {
                    velocity,
                    world_velocity: Vec2::ZERO, // Death particles don't move with world
                    lifetime: DEATH_PARTICLE_LIFETIME * (0.6 + rand_f32() * 0.4),
                    initial_lifetime: DEATH_PARTICLE_LIFETIME,
                },
                DeathParticle,
            ));
        }
    }
}

// ============================================================================
// SCORE POP ANIMATION
// ============================================================================

/// Triggers score pop animation when score event occurs.
pub fn trigger_score_pop(
    mut score_events: MessageReader<ScoreEvent>,
    mut commands: Commands,
    query: Query<Entity, With<ScoreText>>,
) {
    for _ in score_events.read() {
        for entity in query.iter() {
            commands.entity(entity).insert(ScorePopAnimation {
                timer: 0.0,
                duration: SCORE_POP_DURATION,
            });
        }
    }
}

/// Updates score pop animation.
pub fn update_score_pop(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ScorePopAnimation, &mut Transform), With<ScoreText>>,
) {
    for (entity, mut anim, mut transform) in query.iter_mut() {
        anim.timer += time.delta_secs();

        if anim.timer >= anim.duration {
            // Animation complete, reset scale and remove component
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<ScorePopAnimation>();
        } else {
            // Calculate bounce scale using sine wave
            let progress = anim.timer / anim.duration;
            // Quick scale up, then ease back down
            let scale = 1.0 + (progress * std::f32::consts::PI).sin() * (SCORE_POP_SCALE - 1.0);
            transform.scale = Vec3::splat(scale);
        }
    }
}

// ============================================================================
// BIRD SQUASH/STRETCH ANIMATION
// ============================================================================

/// Triggers bird squash/stretch animation on flap.
pub fn trigger_bird_squash(
    mut flap_events: MessageReader<FlapEvent>,
    mut commands: Commands,
    query: Query<Entity, With<Bird>>,
) {
    for _ in flap_events.read() {
        for entity in query.iter() {
            commands.entity(entity).insert(BirdSquashStretch {
                timer: 0.0,
                duration: FLAP_SQUASH_DURATION,
                is_squash: true,
            });
        }
    }
}

/// Updates bird squash/stretch animation with smooth elastic easing.
pub fn update_bird_squash(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BirdSquashStretch, &mut Sprite, &mut Transform), With<Bird>>,
) {
    for (entity, mut anim, mut sprite, mut transform) in query.iter_mut() {
        anim.timer += time.delta_secs();

        if anim.timer >= anim.duration {
            // Animation complete, reset size and scale
            sprite.custom_size = Some(Vec2::splat(BIRD_SIZE));
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<BirdSquashStretch>();
        } else {
            let progress = anim.timer / anim.duration;

            // Use elastic out easing for a bouncy, organic feel
            // This creates a quick snap then gentle settle effect
            let elastic_ease = elastic_out(progress);

            // Inverse elastic for the squash (starts deformed, returns to normal)
            let deform_amount = 1.0 - elastic_ease;

            // Calculate squash (horizontal compress) and stretch (vertical expand)
            let squash = 1.0 + (FLAP_SQUASH_SCALE - 1.0) * deform_amount;
            let stretch = 1.0 + (FLAP_STRETCH_SCALE - 1.0) * deform_amount;

            // Apply the deformation via transform scale for smoother look
            // This gives a more organic curved appearance
            transform.scale = Vec3::new(squash, stretch, 1.0);
        }
    }
}

/// Elastic out easing function for smooth, bouncy animations.
/// Creates a spring-like overshoot effect.
fn elastic_out(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    let p = 0.3; // Period - lower = more oscillations
    let s = p / 4.0; // Amplitude adjustment

    (2.0_f32.powf(-10.0 * t) * ((t - s) * std::f32::consts::TAU / p).sin() + 1.0).clamp(0.0, 1.0)
}

// ============================================================================
// EFFECT TRIGGER SYSTEMS
// ============================================================================

/// Triggers death effects (shake, flash, particles).
pub fn trigger_death_effects(
    mut death_events: MessageReader<DeathEvent>,
    mut shake: ResMut<ScreenShake>,
    mut flash: ResMut<ScreenFlashState>,
) {
    for _ in death_events.read() {
        shake.trigger(SCREEN_SHAKE_DURATION, SCREEN_SHAKE_INTENSITY);
        flash.trigger(DEATH_FLASH_DURATION, DEATH_FLASH_COLOR, DEATH_FLASH_ALPHA);
    }
}

/// Triggers score effects (edge flash).
pub fn trigger_score_effects(
    mut score_events: MessageReader<ScoreEvent>,
    mut flash: ResMut<EdgeFlashState>,
) {
    for _ in score_events.read() {
        flash.trigger(SCORE_FLASH_DURATION, SCORE_FLASH_COLOR, SCORE_FLASH_ALPHA);
    }
}
