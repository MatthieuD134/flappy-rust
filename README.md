# Rust Template

A modern Rust project template with pre-configured development tooling for the best developer experience.

## Features

- 🦀 **Rust 2024 Edition** - Latest stable Rust features
- 🔧 **cargo-make** - Task runner for streamlined workflows
- 📝 **Conventional Commits** - Enforced via commitlint
- 🧹 **Code Quality** - Clippy linting with strict warnings
- 🎨 **Formatting** - rustfmt + taplo (TOML) auto-formatting
- 🧪 **Fast Testing** - cargo-nextest for parallel test execution
- 🔒 **Security** - cargo-audit vulnerability scanning
- ⚖️ **Compliance** - cargo-deny for license and dependency checks
- 📋 **Changelog** - git-cliff for automated changelog generation
- 🪝 **Git Hooks** - Pre-commit, commit-msg, and pre-push hooks via cargo-husky

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- Git

### Setup

1. **Clone or use this template:**

   ```bash
   # Using GitHub template feature (recommended)
   # Click "Use this template" on the repository page

   # Or clone directly
   git clone https://github.com/MatthieuD134/rust-template.git my-project
   cd my-project
   ```

2. **Run the bootstrap script:**

   ```bash
   ./bootstrap.sh
   ```

   This installs all required development tools and sets up git hooks.

3. **Start coding!**

   ```bash
   cargo run
   ```

## Available Tasks

Run tasks with `cargo make <task>`:

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

## Git Hooks

The following hooks are automatically installed:

- **pre-commit**: Formats staged Rust and config files
- **commit-msg**: Validates conventional commit format
- **pre-push**: Runs full check suite before pushing

## Commit Message Format

This project follows [Conventional Commits](https://www.conventionalcommits.org/):

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

## Project Structure

```text
.
├── .cargo-husky/       # Git hook scripts
├── .github/            # GitHub templates, workflows, and prompts
│   └── workflows/      # CI/CD workflows
├── .vscode/            # VS Code settings
├── src/
│   └── main.rs         # Application entry point
├── tests/
│   └── integration_test.rs  # Integration tests
├── Cargo.toml          # Project manifest
├── Makefile.toml       # Task definitions
├── rust-toolchain.toml # Toolchain configuration
├── clippy.toml         # Clippy lint configuration
├── deny.toml           # cargo-deny configuration
├── cliff.toml          # git-cliff configuration
└── rustfmt.toml        # Formatting configuration
```

## Customizing the Template

After creating a new project from this template:

1. Update `Cargo.toml` with your project name, description, and authors
2. Update this README with your project-specific information
3. Modify `deny.toml` if you need different license policies
4. Adjust `.github/PULL_REQUEST_TEMPLATE/default.md` for your workflow

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
