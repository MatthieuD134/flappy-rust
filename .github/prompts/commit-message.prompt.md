---
agent: 'agent'
description: 'Generate conventional commit messages for Flappy Rust changes'
---

# Generate and Create Commit

Analyze changes, generate a conventional commit message, and create the commit.

## Process

1. Check for unstaged changes with `git status`
2. Stage all changes with `git add -A` if there are unstaged files
3. Analyze the staged diff with `git diff --cached`
4. Generate a commit message following the format below
5. Create the commit with `git commit -m "message"` (use multi-line for body)

## Commit Command Examples

Single line:

```bash
git commit -m "feat(bird): add double jump ability"
```

With body:

```bash
git commit -m "feat(bird): add double jump ability

Allows the bird to jump again while in mid-air.
Limited to one extra jump per flap cycle."
```

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
