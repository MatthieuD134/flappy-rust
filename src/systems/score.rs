//! Score tracking systems.
//!
//! This module handles score calculation and display.

use bevy::prelude::*;

use crate::components::{Bird, Pipe, ScoreText, Scored};
use crate::resources::{Score, ScoreEvent};

/// Updates the score when bird passes pipes.
///
/// Only bottom pipes have the `Scored` component to avoid double-counting.
pub fn update_score(
    bird_query: Query<&Transform, With<Bird>>,
    mut pipe_query: Query<(&Transform, &mut Scored), With<Pipe>>,
    mut score: ResMut<Score>,
    mut text_query: Query<&mut Text2d, With<ScoreText>>,
    mut score_events: MessageWriter<ScoreEvent>,
) {
    let Ok(bird_transform) = bird_query.single() else {
        return;
    };
    let bird_x = bird_transform.translation.x;

    for (pipe_transform, mut scored) in pipe_query.iter_mut() {
        if !scored.0 && pipe_transform.translation.x < bird_x {
            scored.0 = true;
            score.increment();

            // Send score event for visual effects
            score_events.write(ScoreEvent);

            for mut text in text_query.iter_mut() {
                text.0 = score.0.to_string();
            }
        }
    }
}
