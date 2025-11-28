---
agent: 'agent'
description: 'Generate conventional commit messages for Flappy Rust changes'
---

# Generate Commit Message

Analyze staged changes and generate a conventional commit message.

## Format

```text
type(scope): description

[optional body]

[optional footer(s)]
```

## Types

- `feat` - New feature (gameplay, mechanics)
- `fix` - Bug fix (collision, physics, state)
- `refactor` - Code restructuring without behavior change
- `docs` - Documentation changes
- `style` - Formatting (rustfmt, no logic change)
- `test` - Adding or updating tests
- `perf` - Performance improvements
- `chore` - Maintenance (deps, config)

## Scopes

Use the module or system being changed:

- `bird` - Bird mechanics (flap, physics, tilt)
- `pipes` - Pipe spawning and movement
- `collision` - Hit detection
- `score` - Score tracking
- `game` - State transitions (menu, playing, game over)
- `setup` - Entity spawning
- `constants` - Game tuning parameters
- `resources` - Global resources
- `components` - ECS components

## Guidelines

- Use imperative mood ("add" not "added")
- Keep first line under 50 characters
- Be specific about what changed
- For Bevy changes, mention systems, components, or resources affected

## Examples

- `feat(bird): add double jump ability`
- `fix(collision): correct pipe hitbox calculation`
- `refactor(systems): split pipe spawner into helper functions`
- `perf(pipes): use with_capacity for pipe batch spawning`
- `chore(deps): update bevy to 0.18`
- `docs(readme): add gameplay screenshot`

## Process

1. Analyze the git diff
2. Determine type based on nature of change
3. Identify scope from affected module/system
4. Write concise description
5. Add body if change needs explanation
