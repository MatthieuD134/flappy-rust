//! Viewport and camera systems for responsive scaling.
//!
//! This module handles dynamic viewport sizing to support different screen sizes.

use bevy::camera::{Projection, ScalingMode};
use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::components::{EdgeFlash, EdgeType, FillScreen, Ground, MainCamera, Sky};
use crate::constants::{GROUND_HEIGHT, SCORE_FLASH_BORDER_WIDTH, SCORE_FLASH_GRADIENT_STRIPS};
use crate::resources::GameViewport;

/// System to update viewport and camera projection on window resize.
pub fn update_viewport(
    mut resize_events: MessageReader<WindowResized>,
    mut viewport: ResMut<GameViewport>,
    mut camera_query: Query<&mut Projection, With<MainCamera>>,
) {
    for event in resize_events.read() {
        viewport.update_from_window(event.width, event.height);

        // Update camera projection to match new viewport
        for mut projection in camera_query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                ortho.scaling_mode = ScalingMode::FixedVertical {
                    viewport_height: viewport.height,
                };
            }
        }
    }
}

/// System to update entities that should fill the screen width.
pub fn update_fill_width_entities(
    viewport: Res<GameViewport>,
    mut ground_query: Query<&mut Sprite, (With<Ground>, Without<Sky>)>,
) {
    if !viewport.is_changed() {
        return;
    }

    // Update ground width
    for mut sprite in ground_query.iter_mut() {
        // Make ground wider than viewport to handle any scrolling or edge cases
        sprite.custom_size = Some(Vec2::new(viewport.width * 2.0, GROUND_HEIGHT));
    }
}

/// System to update entities that should fill the entire screen.
#[allow(clippy::type_complexity)]
pub fn update_fill_screen_entities(
    viewport: Res<GameViewport>,
    mut sky_query: Query<&mut Sprite, (With<Sky>, Without<Ground>, Without<FillScreen>)>,
    mut flash_query: Query<&mut Sprite, (With<FillScreen>, Without<Sky>, Without<Ground>)>,
) {
    if !viewport.is_changed() {
        return;
    }

    // Update sky size
    for mut sprite in sky_query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(viewport.width * 2.0, viewport.height));
    }

    // Update screen flash overlay
    for mut sprite in flash_query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(viewport.width * 2.0, viewport.height * 2.0));
    }
}

/// System to update edge flash positions based on viewport.
pub fn update_edge_flash_positions(
    viewport: Res<GameViewport>,
    mut edge_query: Query<(&mut Transform, &mut Sprite, &EdgeFlash)>,
) {
    if !viewport.is_changed() {
        return;
    }

    let num_strips = SCORE_FLASH_GRADIENT_STRIPS as usize;
    let strip_width = SCORE_FLASH_BORDER_WIDTH / num_strips as f32;

    for (mut transform, mut sprite, edge_flash) in edge_query.iter_mut() {
        let strip_offset = edge_flash.strip_index as f32 * strip_width + strip_width / 2.0;

        match edge_flash.edge {
            EdgeType::Top => {
                sprite.custom_size = Some(Vec2::new(viewport.width * 2.0, strip_width));
                transform.translation.y = viewport.half_height() - strip_offset;
            }
            EdgeType::Bottom => {
                sprite.custom_size = Some(Vec2::new(viewport.width * 2.0, strip_width));
                transform.translation.y = -viewport.half_height() + strip_offset;
            }
            EdgeType::Left => {
                sprite.custom_size = Some(Vec2::new(strip_width, viewport.height));
                transform.translation.x = -viewport.half_width() + strip_offset;
            }
            EdgeType::Right => {
                sprite.custom_size = Some(Vec2::new(strip_width, viewport.height));
                transform.translation.x = viewport.half_width() - strip_offset;
            }
        }
    }
}
