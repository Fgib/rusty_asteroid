# Asteroid Game

A simple asteroid game built with the Bevy game engine in Rust.

## Project Structure

```
src/
├── main.rs              # Main entry point
├── assets/              # Game Assets
│   ├── mod.rs          # Assets module exports
│   └── meshes.rs       # Custom mesh creation functions
├── components/          # ECS Components
│   ├── mod.rs          # Component module exports
│   ├── player.rs       # Player component
│   ├── asteroid.rs     # Asteroid component
│   ├── bullet.rs       # Bullet component
│   ├── physics.rs      # Physics components (Velocity, Wraparound)
│   └── ui.rs           # UI components (ScoreText)
├── resources/          # ECS Resources
│   ├── mod.rs          # Resource module exports
│   ├── game_score.rs   # Game score resource
│   └── spawn_timer.rs  # Asteroid spawn timer resource
├── systems/            # ECS Systems
│   ├── mod.rs          # System module exports
│   ├── setup.rs        # Initial game setup
│   ├── player.rs       # Player movement and shooting
│   ├── physics.rs      # Movement and screen wrapping
│   ├── combat.rs       # Collision detection
│   ├── spawning.rs     # Asteroid spawning
│   ├── ui.rs           # UI updates
│   └── cleanup.rs      # Entity cleanup
└── constants/          # Game Constants
    ├── mod.rs          # Constants module exports
    └── game_constants.rs # Speed constants
```

## Game Controls

- **Rotate Left**: Left Arrow or A
- **Rotate Right**: Right Arrow or D
- **Thrust Forward**: Up Arrow or W
- **Shoot**: Spacebar
- **Exit**: Close the window

## Features

- **Custom Visuals**: Elongated triangle player ship and glowing arrow bullets
- **Bloom Effects**: HDR rendering with post-processing bloom for glowing projectiles
- **Rotation-based Movement**: Like classic Asteroids - rotate and thrust forward
- **Directional Shooting**: Bullets fire in the direction the ship is facing
- **Custom Mesh Rendering**: Player and bullets use custom 2D meshes
- **Modular Assets**: Centralized mesh creation in dedicated assets module
- **Asteroid Spawning**: Random asteroid generation with varied movement
- Collision detection between bullets and asteroids
- Score tracking and display
- Screen wrapping for the player
- Entity cleanup when off-screen

## How to Run

```bash
cargo run
```

## Architecture

This game follows the Entity-Component-System (ECS) pattern provided by Bevy:

- **Components**: Data containers (Player, Asteroid, Bullet, Velocity, etc.)
- **Resources**: Global state (GameScore, AsteroidSpawnTimer)
- **Systems**: Game logic functions that operate on components and resources
- **Constants**: Game configuration values

The modular structure makes it easy to add new features, modify existing behavior, and maintain the codebase.
