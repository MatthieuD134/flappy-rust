//! Rust Template Application
//!
//! This is a template project demonstrating Rust best practices.

use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Application entry point.
///
/// # Errors
///
/// Returns an error if the application fails to run.
fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from rust-template!");

    // TODO: Add your application logic here

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_succeeds() {
        assert!(run().is_ok());
    }
}
