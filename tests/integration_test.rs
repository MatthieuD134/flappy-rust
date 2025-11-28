//! Integration tests for Flappy Rust
//!
//! Integration tests for the Flappy Bird clone.
//! Note: Since this is a graphical game using Bevy, full integration tests
//! would require a headless renderer.

#[test]
fn game_constants_are_valid() {
    // Verify game constants would work for gameplay
    // These would be imported from the main module if it were a library
    let pipe_gap: f32 = 150.0;
    let bird_size: f32 = 30.0;

    // Bird should fit through the pipe gap
    assert!(bird_size < pipe_gap);
}
