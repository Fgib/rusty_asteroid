# Rusty Asteroid - Developer Documentation

## 🎮 Game Overview

Rusty Asteroid is a modern take on the classic Asteroids game, built with the Bevy game engine in Rust. The game features sophisticated enemy AI, multi-phase boss battles, power-ups, and enhanced visual effects with HDR bloom.

## 🏗️ Architecture Overview

### Core Design Pattern: ECS (Entity-Component-System)

The game uses Bevy's ECS architecture where:

- **Entities** are unique IDs that represent game objects
- **Components** are data structures that define what an entity has
- **Systems** are functions that operate on entities with specific components

### Project Structure

```
src/
├── main.rs              # App setup and system registration
├── assets/              # Mesh generation and asset management
│   ├── mod.rs
│   ├── meshes.rs
│   └── meshes_powerups.rs
├── components/          # ECS Components (data)
│   ├── mod.rs
│   ├── asteroid.rs      # Asteroid types and behaviors
│   ├── boss.rs          # Boss entities and attack patterns
│   ├── bullet.rs        # Bullet types and lifecycle
│   ├── enemy.rs         # Enemy types and AI states
│   ├── player.rs        # Player state and abilities
│   ├── powerup.rs       # Power-up types and effects
│   └── physics.rs       # Physics components (velocity, collision)
├── constants/           # Game configuration
│   ├── mod.rs
│   └── game_constants.rs
├── resources/           # Global game state
│   ├── mod.rs
│   ├── game_state.rs    # Game state management
│   ├── difficulty.rs    # Difficulty settings
│   ├── save_data.rs     # Persistence system
│   └── enemy_boss.rs    # Enemy spawning timers
└── systems/             # ECS Systems (behavior)
    ├── mod.rs
    ├── combat.rs        # All collision and damage systems
    ├── enemy_boss.rs    # Enemy AI and boss behavior
    ├── player.rs        # Player movement and shooting
    ├── spawning.rs      # Entity spawning logic
    ├── powerups.rs      # Power-up effects and management
    ├── physics.rs       # Movement and physics
    ├── setup.rs         # Game initialization and cleanup
    └── ui.rs            # User interface updates
```

## 🎯 Key Systems

### Combat System (`src/systems/combat.rs`)

Handles all collision detection and damage calculation:

- **Player vs Asteroid**: Asteroid destruction, fragments, scoring
- **Player vs Enemy**: Enemy destruction, damage to player
- **Player vs Boss**: Multi-phase boss damage system
- **Bullet vs Bullet**: Player bullets can intercept enemy fire
- **Special Effects**: Ice shatter, crystal explosions, spark effects

### Enemy AI System (`src/systems/enemy_boss.rs`)

Sophisticated AI with multiple behaviors:

- **Predictive Targeting**: Enemies lead their shots
- **Obstacle Avoidance**: Navigate around asteroids
- **Behavioral States**: Hunt, evade, attack patterns
- **Boss AI**: Multi-phase bosses with different attack patterns

### Spawning System (`src/systems/spawning.rs`)

Manages entity creation:

- **Dynamic Difficulty**: Spawn rates adjust with score
- **Asteroid Fragments**: Realistic breakup physics
- **Enemy Waves**: Score-based enemy spawning
- **Power-up Drops**: Rare beneficial items

## 🎨 Visual Systems

### HDR and Bloom

- Camera configured with HDR and bloom post-processing
- Bright colors (RGB values > 1.0) trigger bloom effects
- Used for bullets, power-ups, and special effects

### Custom Mesh Rendering

- All game objects use custom 2D meshes instead of sprites
- Procedurally generated shapes for asteroids
- Geometric designs for ships and bullets

### Pulsing Effects

- Enemies and their projectiles pulse for visibility
- Configurable pulse speed and amplitude
- Synchronized with game timing

## 🔧 Development Patterns

### Component Design

```rust
#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub last_shot_time: f32,
    pub shot_cooldown: f32,
}
```

### System Implementation

```rust
pub fn enemy_ai_system(
    mut enemy_query: Query<(&mut Transform, &mut Velocity, &Enemy)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    // System logic here
}
```

### Resource Management

```rust
#[derive(Resource)]
pub struct GameScore {
    pub score: u32,
}
```

## 🎮 Game Features

### Enhanced Gameplay Mechanics

- **Multiple Asteroid Types**: Normal, Ice (fragments more), Metal (tough), Crystal (power-ups)
- **Enemy Varieties**: Hunter (fast), Bomber (explosive), Interceptor (agile)
- **Boss Battles**: Multi-phase encounters with different attack patterns
- **Power-up System**: Temporary and permanent upgrades
- **Bullet Interception**: Player bullets can destroy enemy projectiles

### Player Abilities

- **Enhanced Shooting**: Power-ups affect bullet behavior
- **Piercing Bullets**: Pass through multiple targets
- **Explosive Bullets**: Area damage on impact
- **Laser Beams**: Continuous damage over time
- **Invincibility Frames**: Brief protection after taking damage

### Difficulty Scaling

- **Dynamic Spawn Rates**: More enemies as score increases
- **Boss Thresholds**: Bosses appear at specific score milestones
- **Adaptive AI**: Enemy behavior becomes more aggressive over time

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ with Cargo
- Compatible graphics drivers for HDR rendering

### Building and Running

```bash
cargo run --release
```

### Development Mode

```bash
cargo run  # Debug mode with additional logging
```

## 🔍 Debugging and Development

### Common Issues

- **Performance**: Use `cargo run --release` for optimal performance
- **Bloom Not Working**: Ensure HDR is enabled on camera
- **Collision Issues**: Check collision radii in combat systems
- **Entity Cleanup**: Verify entities are despawned in cleanup systems

### Useful Debug Information

- Enable `trace_location` in Cargo.toml for entity debugging
- Use `cargo check` for rapid compilation checking
- Monitor console output for entity lifecycle warnings

## 🎯 Future Development Areas

### Potential Enhancements

1. **Network Multiplayer**: Add cooperative or competitive modes
2. **Particle Systems**: Enhanced visual effects for explosions
3. **Sound System**: Audio feedback for actions and events
4. **Procedural Levels**: Dynamically generated asteroid fields
5. **Achievement System**: Track player accomplishments
6. **Weapon Variety**: Additional power-up types and combinations

### Architecture Improvements

1. **State Management**: More sophisticated game state transitions
2. **Configuration System**: Runtime tweaking of game parameters
3. **Asset Pipeline**: Better organization of visual and audio assets
4. **Performance Profiling**: Built-in performance monitoring
5. **Modding Support**: Plugin system for community content

---

_This documentation is intended for developers working on or extending the Rusty Asteroid game. For player instructions, see the README.md file._
