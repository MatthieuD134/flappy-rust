# Copilot Instructions for Flappy Rust

This document provides guidance for AI agents working on this Bevy game project.

## Project Overview

**Flappy Rust** is a Flappy Bird clone built with the Bevy game engine (v0.17). It uses simple geometric shapes for visuals and implements classic flappy bird mechanics.

## Architecture

```text
src/
├── main.rs           # App setup and plugin configuration
├── constants.rs      # Game tuning parameters
├── states.rs         # GameState enum (Menu, Playing, GameOver)
├── components.rs     # ECS components (Bird, Pipe, Scored, etc.)
├── resources.rs      # Global resources (Score, PipeSpawnTimer)
├── utils.rs          # Helper functions (rand_f32)
└── systems/
    ├── mod.rs        # System re-exports
    ├── setup.rs      # Entity spawning
    ├── bird.rs       # Bird physics, flap, tilt
    ├── pipes.rs      # Pipe spawning and movement
    ├── collision.rs  # Collision detection
    ├── score.rs      # Score tracking
    └── game.rs       # State transitions
```

## Key Patterns

### Bevy ECS

- **Components**: Data attached to entities (Bird, Pipe, Scored)
- **Resources**: Global state (Score, PipeSpawnTimer)
- **Systems**: Functions that operate on queries of components
- **States**: GameState controls which systems run

### Game Loop

1. `Menu` → Press SPACE → `Playing`
2. `Playing` → Collision → `GameOver`
3. `GameOver` → Press SPACE → `Playing` (reset)

## Development

### Constants

All tunable values are in `src/constants.rs`. Adjust these for gameplay balance:

- `GRAVITY`, `FLAP_STRENGTH` - Bird physics
- `PIPE_GAP_MIN/MAX`, `WORLD_SCROLL_SPEED` - Difficulty
- `BIRD_SIZE`, `PIPE_WIDTH` - Collision bounds

### Adding Features

1. Add components to `components.rs`
2. Add systems to appropriate file in `systems/`
3. Register systems in `main.rs` with correct state conditions
4. Update `systems/mod.rs` exports

### Code Style

- Document all public items with `///`
- Use descriptive system function names
- Keep systems focused on single responsibility
- Use `Query` filters (`With<T>`, `Without<T>`) appropriately

## Tasks

Run with `cargo make <task>`:

- `check` - Run all checks (lint, test, audit)
- `lint` - Clippy with warnings as errors
- `test` - Run tests with nextest
- `format` - Format all files

## Commits

Follow conventional commits: `type(scope): description`

- `feat(bird)`: New bird features
- `fix(collision)`: Collision bug fixes
- `refactor(systems)`: Code restructuring
