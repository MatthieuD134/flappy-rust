# Flappy Rust 🐦

A Flappy Bird clone built with the [Bevy](https://bevyengine.org/) game engine in Rust.

## Screenshot

The game uses simple geometric shapes:

- 🟨 Yellow square = Bird
- 🟩 Green rectangles = Pipes
- 🟫 Brown rectangle = Ground
- 🔵 Light blue = Sky

## Features

- Classic flappy bird gameplay
- Bird tilt animation based on velocity
- Random pipe gap positions and sizes
- Score tracking
- Game states (Menu, Playing, Game Over)

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

### Run the Game

```bash
cargo run
```

### Controls

- **SPACE** - Start game / Flap / Restart

## Development

### Setup

```bash
./bootstrap.sh  # Install dev tools
```

### Available Tasks

```bash
cargo make check   # Run all checks
cargo make lint    # Clippy linting
cargo make test    # Run tests
cargo make format  # Format code
```

## Project Structure

```text
src/
├── main.rs           # App setup
├── constants.rs      # Game parameters
├── states.rs         # GameState enum
├── components.rs     # ECS components
├── resources.rs      # Global resources
├── utils.rs          # Helpers (RNG)
└── systems/          # Game logic
    ├── setup.rs      # Entity spawning
    ├── bird.rs       # Bird mechanics
    ├── pipes.rs      # Pipe spawning
    ├── collision.rs  # Hit detection
    ├── score.rs      # Scoring
    └── game.rs       # State management
```

## Tuning

Edit `src/constants.rs` to adjust:

- `GRAVITY` / `FLAP_STRENGTH` - Bird physics
- `PIPE_GAP_MIN` / `PIPE_GAP_MAX` - Difficulty
- `PIPE_SPEED` / `PIPE_SPAWN_TIME` - Pacing

## License

MIT License - see [LICENSE](LICENSE)
