# System Architecture Guide

## 🏗️ System Organization

Systems in Rusty Asteroid are organized by functionality and run in specific phases of the game loop. Understanding the execution order and dependencies is crucial for development.

## 🎮 Game Loop Structure

### Startup Systems

Run once when the application starts:

- `setup_camera()` - Initializes HDR camera with bloom

### State Transition Systems

Run when entering/exiting game states:

- **OnEnter(Playing)**: `setup_game()`, `reset_game_resources()`, `reset_powerups_system()`
- **OnExit(Playing)**: `cleanup_all_entities()`

### Core Game Systems (Update Phase)

These run every frame during gameplay:

#### Movement & Physics

```rust
(
    player_movement,        // Handle player input
    enhanced_player_shoot,  // Process shooting input
    move_entities,         // Apply velocity to transforms
    rotate_entities,       // Apply rotation velocity
    wrap_around,          // Screen edge wrapping
)
```

#### Collision Detection

```rust
(
    collision_system,                    // Player bullets vs asteroids
    laser_collision_system,              // Laser beams vs targets
    player_asteroid_collision_system,    // Player vs asteroids
    player_enemy_bullet_collision_system,// Player vs enemy bullets
    bullet_enemy_collision_system,       // Player bullets vs enemies
    bullet_boss_collision_system,        // Player bullets vs bosses
    bullet_bullet_collision_system,      // Bullet interception
)
```

#### Entity Management

```rust
(
    invincibility_visual_system,  // Handle invincibility flashing
    update_bullet_lifecycle,      // Bullet aging and fading
    despawn_asteroids,           // Remove off-screen asteroids
    spawn_asteroids,             // Create new asteroids
)
```

#### AI & Enemies

```rust
(
    spawn_enemy_system,          // Create enemies based on score
    enemy_ai_system,             // Enemy behavior and movement
    enemy_shooting_system,       // Enemy projectile firing
    boss_spawn_system,           // Create bosses at thresholds
    boss_ai_system,             // Boss behavior and attacks
    pulsing_effect_system,      // Visual pulsing effects
)
```

#### Power-ups & UI

```rust
(
    spawn_powerup_system,        // Create power-up drops
    powerup_collection_system,   // Handle power-up pickup
    powerup_effect_system,       // Manage active power-ups
    update_score_display,        // Refresh score UI
    update_lives_display,        // Refresh lives UI
    update_heart_display,        // Update heart icons
    update_powerup_display,      // Show active power-ups
    save_game_progress,          // Persistent save data
)
```

## 🎯 Key System Details

### Combat System (`combat.rs`)

**Purpose**: All collision detection and damage calculation
**Key Functions**:

- `collision_system()` - Player bullets vs asteroids with special effects
- `bullet_bullet_collision_system()` - Bullet interception mechanics
- `create_ice_shatter_effect()` - Ice asteroid destruction
- `create_crystal_explosion_effect()` - Crystal asteroid rewards

**Performance Notes**:

- Uses spatial partitioning for collision optimization
- Batch processing for explosion effects
- Early termination for bullet-bullet collisions

### Enemy Boss System (`enemy_boss.rs`)

**Purpose**: AI behavior and boss mechanics
**Key Functions**:

- `enemy_ai_system()` - Predictive targeting and obstacle avoidance
- `boss_ai_system()` - Multi-phase boss behavior
- `spawn_enemy_system()` - Score-based enemy creation
- `pulsing_effect_system()` - Visual enhancement system

**AI Patterns**:

- **Predictive Targeting**: Calculates bullet lead time
- **Obstacle Avoidance**: Raycast-based navigation
- **Behavioral States**: Hunt/Evade/Attack state machine

### Player System (`player.rs`)

**Purpose**: Player input and abilities
**Key Functions**:

- `player_movement()` - Keyboard input processing
- `enhanced_player_shoot()` - Power-up aware shooting
- `invincibility_visual_system()` - Damage immunity effects

**Input Handling**:

- WASD/Arrow keys for movement
- Space/Mouse for shooting
- Supports multiple input methods simultaneously

### Spawning System (`spawning.rs`)

**Purpose**: Entity creation and lifecycle
**Key Functions**:

- `spawn_asteroids()` - Dynamic asteroid generation
- `spawn_asteroid_fragments()` - Realistic breakup physics
- Edge-based spawning for natural difficulty curve

**Spawn Logic**:

- Score-based difficulty scaling
- Type probability distributions
- Edge spawn positioning for natural entry

## 🔄 System Dependencies

### Data Flow

```
Input → Player System → Movement → Collision → Entity Updates → Rendering
```

### Resource Dependencies

```
GameScore ←→ Spawning Systems ←→ Difficulty Settings
PlayerPowerUps ←→ Combat Systems ←→ Visual Effects
```

### Component Dependencies

```
Transform + Velocity → Physics → Collision → Health → Despawn
```

## ⚡ Performance Considerations

### System Ordering

Systems are ordered to minimize frame-to-frame delays:

1. **Input Processing** - Immediate response
2. **Movement** - Apply changes
3. **Collision** - Detect interactions
4. **Spawning** - Create new entities
5. **Cleanup** - Remove dead entities

### Batch Operations

- Collision detection uses batch processing
- Entity spawning is deferred to avoid mid-frame changes
- Visual effects are pooled and reused

### Memory Management

- Entities are despawned immediately when no longer needed
- Resources are reset between game sessions
- No memory leaks in entity lifecycle

## 🔧 Development Patterns

### Adding New Systems

1. **Define the system function**:

```rust
pub fn my_new_system(
    query: Query<&MyComponent>,
    time: Res<Time>,
) {
    // System logic
}
```

2. **Register in main.rs**:

```rust
.add_systems(Update, my_new_system.run_if(in_state(GameState::Playing)))
```

3. **Consider system ordering**:

- Input systems run first
- Physics systems run before collision
- Cleanup systems run last

### Query Optimization

```rust
// Good: Specific queries
Query<&Transform, (With<Enemy>, Without<Player>)>

// Bad: Overly broad queries
Query<&Transform>
```

### Resource Usage

```rust
// Read-only resources
time: Res<Time>

// Mutable resources (use sparingly)
score: ResMut<GameScore>
```

## 🐛 Common Issues

### System Ordering Problems

- **Symptom**: Entities appear one frame late
- **Solution**: Ensure spawning systems run before rendering

### Query Conflicts

- **Symptom**: Compile-time borrowing errors
- **Solution**: Split queries or use `ParamSet`

### Resource Contention

- **Symptom**: Systems can't run in parallel
- **Solution**: Minimize mutable resource usage

### Entity Lifecycle Issues

- **Symptom**: Entities persist between game sessions
- **Solution**: Add to cleanup system queries

## 🚀 Performance Tips

1. **Use Markers**: Prefer marker components over data when possible
2. **Filter Queries**: Use `With`/`Without` to minimize iteration
3. **Batch Processing**: Group related operations together
4. **Avoid String Allocations**: Use constants for repeated text
5. **Profile Systems**: Use Bevy's diagnostic plugins for bottlenecks

## 🎯 Future Enhancements

### Planned System Improvements

1. **Spatial Partitioning**: Grid-based collision optimization
2. **Component Pooling**: Reuse common components
3. **System Scheduling**: More granular system dependencies
4. **Debug Systems**: Runtime system performance monitoring
5. **Hot Reloading**: Dynamic system code updates
