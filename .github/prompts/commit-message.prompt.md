---
agent: 'agent'
description: 'Generate conventional commit messages based on staged changes and project commitlint configuration'
---

# Prompt: Create Conventional Commit

## Context

When code changes are ready to be committed, it's important to create a commit message that follows the conventional commit format and accurately describes the changes. This prompt helps a code agent to analyze staged changes and generate a precise, readable commit message that adheres to the project's commitlint configuration.

## Instructions

### Input

- The current git status showing staged/unstaged changes
- The git diff of the changes to be committed
- The project's commitlint configuration (conventional commits format)

### Task

As a code agent, your task is to:

1. **Analyze the staged changes** to understand what functionality has been modified, added, or removed.
2. **Determine the appropriate commit type** based on the nature of the changes (feat, fix, refactor, chore, docs, style, test, etc.).
3. **Identify the scope** of the changes (the module, component, or area affected).
4. **Generate a conventional commit message** that is:
   - Precise and descriptive
   - Follows the conventional commit format: `type(scope): description`
   - Concise but informative
   - Written in imperative mood (e.g., "add", "fix", "update", not "added", "fixed", "updated")

### Conventional Commit Format

The commit message should follow this structure:

```text
type(scope): description

[optional body]

[optional footer(s)]
```

#### Commit Types

- **feat**: A new feature for the user
- **fix**: A bug fix
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **chore**: Changes to the build process or auxiliary tools and libraries
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (formatting, rustfmt, etc.)
- **test**: Adding missing tests or correcting existing tests
- **perf**: A code change that improves performance
- **ci**: Changes to CI configuration files and scripts
- **build**: Changes that affect the build system or external dependencies

#### Scope Guidelines for Rust Projects

- Use the crate, module, or component name that best describes where the changes are made
- For library crates: use the module path (e.g., `parser`, `error`, `config`)
- For binary crates: `cli`, `main`, or specific command names
- For workspace changes: use the crate name
- For Cargo/build changes: `cargo`, `deps`, `build`
- For tooling: `clippy`, `rustfmt`, `ci`
- For global changes: `global` or omit scope
- Keep scopes consistent with existing patterns in the project

#### Description Guidelines

- Start with a lowercase letter
- Do not end with a period
- Use imperative mood ("add" not "added")
- Be concise but descriptive (ideally under 50 characters for the entire first line)
- Focus on what the change does, not how it was implemented
- For Rust-specific changes, use appropriate terminology (e.g., "implement trait", "derive macro", "add lifetime annotation")

### Process

1. **Stage the changes** (if not already staged) using `git add`
2. **Analyze the diff** to understand the changes
3. **Generate the commit message** following the conventional format
4. **Create the commit** with the generated message
5. **Verify** that the commit message passes commitlint validation

### Examples

Based on common Rust project patterns:

- `feat(parser): implement FromStr trait for config types`
- `fix(error): handle edge case in Result unwrapping`
- `refactor(lib): replace Box<dyn Error> with thiserror types`
- `chore(deps): update serde to 1.0.200`
- `docs(api): add examples to public function documentation`
- `test(utils): add property-based tests for parsing logic`
- `perf(core): use Vec::with_capacity for known-size allocations`
- `style: apply rustfmt formatting changes`
- `build(cargo): enable LTO for release builds`
- `ci: add cargo-deny check to CI pipeline`

### Output

1. **Commit message**: The properly formatted conventional commit message
2. **Explanation**: Brief explanation of why this type/scope/description was chosen
3. **Commit execution**: Actually create the commit with the generated message

### Important Notes

- **Precision over brevity**: The commit message should be precise enough that someone reading the git log understands what changed
- **Consistency**: Follow the existing patterns in the project's commit history
- **Validation**: Ensure the commit message passes the project's commitlint rules
- **Multiple changes**: If there are multiple unrelated changes, consider suggesting separate commits
- **Breaking changes**: If the changes include breaking changes, add `!` after the type/scope and include `BREAKING CHANGE:` in the footer
- **Rust-specific considerations**:
  - Mention if changes affect public API (`pub` items)
  - Note if changes involve `unsafe` code
  - Reference relevant clippy lints if addressing warnings
  - Mention MSRV (Minimum Supported Rust Version) impact if applicable

### Error Handling

- If no changes are staged, prompt to stage changes first
- If changes are too diverse for a single commit, suggest splitting into multiple commits
- If the scope is unclear, ask for clarification or use the most appropriate general scope
- Validate the commit message format before committing
- If clippy or rustfmt changes are mixed with logic changes, suggest separating them
