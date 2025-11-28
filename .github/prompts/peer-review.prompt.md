---
agent: agent
argument-hint: 'branch name to compare against (e.g. origin/main)'
description: 'Review code changes in Flappy Rust'
---

# Code Review

Compare current branch to `${input:target_branch:Enter target branch}` and review changes.

## Review Process

1. Get the diff between branches
2. Identify all modified files and their purpose
3. Review each change against criteria below
4. Provide constructive, actionable feedback

## Review Criteria

### Bevy ECS Patterns

- Components are data-only (no behavior logic)
- Resources used for global state
- Systems are focused and single-purpose
- Queries use appropriate filters (`With<T>`, `Without<T>`)
- State conditions applied correctly in `main.rs`

### Game Logic

- Bird physics respects GRAVITY, FLAP_STRENGTH constants
- Pipe spawning uses proper randomness (rand_f32)
- Collision detection covers all cases
- State transitions work correctly (Menu → Playing → GameOver)
- Score only increments once per pipe pair

### Code Organization

- New systems added to appropriate file in `systems/`
- Systems exported in `systems/mod.rs`
- New components in `components.rs`
- New resources in `resources.rs`
- Tunable values in `constants.rs`, not hardcoded

### Rust Best Practices

- Proper error handling for `Query::single()`
- No unwrap() in game logic
- Clippy warnings addressed
- Public items documented with `///`
- Consistent naming (snake_case functions, PascalCase types)

## Common Issues to Check

- Systems not registered in `main.rs`
- Missing `.run_if(in_state(...))` conditions
- Hardcoded magic numbers
- Unused components or resources
- Query conflicts between parallel systems
- Timer not ticking in update systems

## Output Format

Group feedback by category:

- **Critical** - Must fix before merge
- **Suggestions** - Improvements to consider
- **Praise** - Good patterns to acknowledge

Provide file paths and line numbers when relevant.
