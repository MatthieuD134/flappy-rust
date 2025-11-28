//! Integration tests for rust-template
//!
//! Integration tests live in the `tests/` directory and test the public API
//! of your crate as an external user would.

use std::process::Command;

#[test]
fn binary_runs_successfully() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust-template"))
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Hello from rust-template!"));
}

#[test]
fn binary_exits_with_zero() {
    let status = Command::new(env!("CARGO_BIN_EXE_rust-template"))
        .status()
        .expect("Failed to execute binary");

    assert!(status.success());
}
