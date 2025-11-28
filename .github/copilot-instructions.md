# Copilot Instructions for Rust Template Repository

This document provides guidance for AI agents working on this Rust repository. Follow these instructions to ensure code quality, maintainability, and consistency.

## Project Overview

This is a **Rust template repository** using edition 2024 with a comprehensive development toolchain. It serves as a starting point for new Rust projects with pre-configured linting, formatting, testing, and security tooling.

### Key Files

- `Cargo.toml` - Project manifest and dependencies
- `Makefile.toml` - Task runner configuration (cargo-make)
- `rust-toolchain.toml` - Rust toolchain specification (stable channel)
- `bootstrap.sh` - Developer environment setup script

## Development Toolchain

The repository uses these tools (installed via `bootstrap.sh`):

| Tool | Purpose |
|------|---------|
| `cargo-make` | Task runner for development workflows |
| `clippy` | Linting with strict warnings |
| `rustfmt` | Code formatting |
| `cargo-nextest` | Fast test runner |
| `cargo-audit` | Security vulnerability scanning |
| `cargo-deny` | License and dependency checking |
| `taplo` | TOML formatting |
| `rumdl` | Markdown formatting and linting |
| `git-cliff` | Changelog generation |
| `commitlint` | Conventional commits validation |

## Rust Best Practices

### Code Style

1. **Follow Rust idioms** - Use pattern matching, iterators, and the type system effectively
2. **Prefer `impl Trait`** over explicit generics when the type doesn't need to be named
3. **Use `Result` and `Option`** - Never panic in library code; use proper error handling
4. **Avoid `.unwrap()` and `.expect()`** in production code - Use `?` operator or proper error handling
5. **Prefer owned types in public APIs** unless borrowing is clearly beneficial for performance

### Error Handling

```rust
// ✓ Good: Use thiserror for custom errors
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {msg}")]
    Parse { msg: String },
}

// ✓ Good: Use anyhow for application errors
fn main() -> anyhow::Result<()> {
    // ...
    Ok(())
}
```

### Documentation

1. **Document all public items** with `///` doc comments
2. **Include examples** in doc comments using ```` ```rust ```` blocks
3. **Use `#![deny(missing_docs)]`** at the crate root for libraries
4. **Document panics, errors, and safety** where applicable

```rust
/// Calculates the factorial of a number.
///
/// # Arguments
///
/// * `n` - The number to calculate factorial for
///
/// # Returns
///
/// The factorial of `n`, or `None` if the result would overflow.
///
/// # Examples
///
/// ```
/// assert_eq!(factorial(5), Some(120));
/// ```
pub fn factorial(n: u64) -> Option<u64> {
    // implementation
}
```

### Module Organization

```text
src/
├── lib.rs          # Library root (if applicable)
├── main.rs         # Binary entry point
├── error.rs        # Error types
├── config.rs       # Configuration
├── models/         # Data structures
│   └── mod.rs
├── services/       # Business logic
│   └── mod.rs
└── utils/          # Utility functions
    └── mod.rs
```

### Testing

1. **Write unit tests** in the same file using `#[cfg(test)]` module
2. **Write integration tests** in the `tests/` directory
3. **Use descriptive test names** that explain what is being tested
4. **Test edge cases** and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_input_returns_expected_value() {
        // Arrange
        let input = "42";

        // Act
        let result = parse(input);

        // Assert
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn parse_empty_input_returns_error() {
        let result = parse("");
        assert!(result.is_err());
    }
}
```

### Performance

1. **Use `&str` instead of `String`** for read-only string parameters
2. **Prefer `Vec::with_capacity`** when the size is known
3. **Use `Cow<str>`** when you might or might not need to clone
4. **Consider `#[inline]`** for small, frequently-called functions
5. **Use iterators** instead of index-based loops

### Safety

1. **Minimize `unsafe` code** - Justify and document all `unsafe` blocks
2. **Validate all external input** before processing
3. **Use `clippy::pedantic`** for stricter linting in critical code
4. **Run `cargo audit`** regularly to check for vulnerabilities

## Available Tasks

Run these with `cargo make <task>`:

| Task | Description |
|------|-------------|
| `lint` | Run clippy with warnings as errors |
| `test` | Run tests with nextest |
| `audit` | Check for security vulnerabilities |
| `deny` | Check licenses and dependencies |
| `check` | Run all checks (lint, test, audit, deny) |
| `format` | Format all Rust, TOML, and Markdown files |
| `format-staged` | Format staged files before commit |
| `docs` | Build documentation |
| `changelog` | Generate changelog from git commits |
| `changelog-unreleased` | Preview unreleased changelog entries |

## Git Workflow

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

**Examples:**

- `feat(parser): add support for nested expressions`
- `fix(auth): resolve token expiration issue`
- `docs: update API documentation`

### Pre-commit Hooks

The repository uses `cargo-husky` to run:

- Code formatting (rustfmt, taplo)
- Commit message validation (commitlint)

## When Making Changes

### Before Writing Code

1. Understand the existing code structure
2. Check for existing patterns and conventions
3. Consider backwards compatibility

### While Writing Code

1. Run `cargo check` frequently to catch errors early
2. Use `cargo clippy` to catch common mistakes
3. Format code with `cargo fmt`

### Before Committing

1. Run `cargo make check` to validate all checks pass
2. Ensure all new public items are documented
3. Add or update tests for changed functionality
4. Write a clear, conventional commit message

## Dependencies

### Adding Dependencies

1. **Prefer well-maintained crates** with active development
2. **Check license compatibility** using `cargo deny`
3. **Minimize dependency count** - only add what's necessary
4. **Pin versions appropriately** using semantic versioning

```toml
# ✓ Good: Specific minor version
serde = "1.0"

# ✓ Good: With features specified
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### Common Recommended Crates

| Purpose | Crate |
|---------|-------|
| Serialization | `serde`, `serde_json` |
| Error handling | `thiserror`, `anyhow` |
| Async runtime | `tokio` |
| HTTP client | `reqwest` |
| CLI parsing | `clap` |
| Logging | `tracing`, `tracing-subscriber` |
| Testing | `proptest`, `mockall` |

## Code Review Checklist

When reviewing or generating code, verify:

- [ ] Code compiles without warnings (`cargo build`)
- [ ] Clippy passes (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Tests pass (`cargo nextest run`)
- [ ] Public items are documented
- [ ] Error handling is appropriate
- [ ] No unnecessary `.unwrap()` or `.expect()` calls
- [ ] Dependencies are justified and compatible
