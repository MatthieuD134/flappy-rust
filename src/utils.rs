//! Utility functions.
//!
//! This module contains helper functions used throughout the game.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Global seed for the random number generator.
static SEED: AtomicU64 = AtomicU64::new(0);

/// Simple pseudo-random number generator returning a value in [0.0, 1.0).
///
/// Uses a static seed that gets updated each call using the xorshift64 algorithm
/// for better distribution than naive time-based approaches.
///
/// # Examples
///
/// ```ignore
/// let value = rand_f32();
/// assert!(value >= 0.0 && value < 1.0);
/// ```
pub fn rand_f32() -> f32 {
    // Initialize seed from time on first call
    let mut seed = SEED.load(Ordering::Relaxed);
    if seed == 0 {
        seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
    }

    // xorshift64 algorithm for better randomness
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;
    SEED.store(seed, Ordering::Relaxed);

    // Convert to float in range [0, 1)
    (seed % 10000) as f32 / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rand_f32_returns_value_in_range() {
        for _ in 0..100 {
            let value = rand_f32();
            assert!((0.0..1.0).contains(&value));
        }
    }

    #[test]
    fn rand_f32_produces_different_values() {
        let values: Vec<f32> = (0..10).map(|_| rand_f32()).collect();
        // Check that not all values are the same
        let first = values[0];
        assert!(values.iter().any(|&v| (v - first).abs() > f32::EPSILON));
    }
}
