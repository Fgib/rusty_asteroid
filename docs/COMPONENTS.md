# Component Reference Guide

## 🎯 Core Game Components

### Player Components

#### `Player`

```rust
#[derive(Component)]
pub struct Player;
```

**Purpose**: Marker component for the player entity
**Systems**: Used by movement, collision, UI systems

#### `Invincibility`

```rust
#[derive(Component)]
pub struct Invincibility {
    pub timer: Timer,
}
```

**Purpose**: Temporary invincibility after taking damage
**Duration**: ~2 seconds of protection
**Visual**: Player flashes during invincibility period

### Projectile Components

#### `Bullet`

```rust
#[derive(Component)]
pub struct Bullet;
```

**Purpose**: Marker for player bullets
**Collision**: Interacts with asteroids, enemies, bosses, enemy bullets

#### `EnemyBullet`

```rust
#[derive(Component)]
pub struct EnemyBullet {
    pub damage: u32,
    pub is_explosive: bool,
}
```

**Purpose**: Enemy projectiles with damage properties
**Note**: damage/is_explosive fields reserved for future damage system

#### `BulletLifecycle`

```rust
#[derive(Component)]
pub struct BulletLifecycle {
    pub lifetime: Timer,
    pub fade_time: f32,
}
```

**Purpose**: Manages bullet lifespan and fade effects
**Behavior**: Bullets fade out before despawning

### Asteroid Components

#### `Asteroid`

```rust
#[derive(Component)]
pub struct Asteroid {
    pub size: u32,
    pub asteroid_type: AsteroidType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsteroidType {
    Normal,   // Standard asteroid
    Ice,      // Shatters into more pieces
    Metal,    // Tougher, more points
    Crystal,  // Rare, drops power-ups
}
```

**Purpose**: Defines asteroid properties and behavior
**Size Range**: 1-4 (affects health, points, fragments)
**Behavior Modifiers**: Each type has unique destruction patterns

### Enemy Components

#### `Enemy`

```rust
#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub last_shot_time: f32,
    pub shot_cooldown: f32,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub enum EnemyType {
    Hunter,     // Fast, frequent shots
    Bomber,     // Slow, powerful shots
    Interceptor, // Very fast, rare shots
}
```

**Purpose**: Enemy behavior and stats
**Stats**: Each type has different speed, health, shooting patterns

#### `AIBehavior`

```rust
#[derive(Component)]
pub struct AIBehavior {
    pub current_behavior: BehaviorState,
    pub behavior_timer: Timer,
    pub target_position: Vec2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BehaviorState {
    Hunting,   // Pursuing player
    Evading,   // Avoiding obstacles
    Attacking, // In combat range
}
```

**Purpose**: AI state machine for enemy behavior
**Timer**: Controls behavior switching frequency

### Boss Components

#### `Boss`

```rust
#[derive(Component)]
pub struct Boss {
    pub boss_type: BossType,
    pub phase: u32,
    pub max_phases: u32,
    pub phase_health: u32,
    pub max_phase_health: u32,
    pub attack_timer: Timer,
    pub phase_transition_timer: Option<Timer>,
    pub size_multiplier: f32,
}
```

**Purpose**: Multi-phase boss system
**Phases**: Bosses get stronger and change attacks as phases progress
**Health**: Each phase has separate health pool

#### `BossAttackPattern`

```rust
#[derive(Component)]
pub struct BossAttackPattern {
    pub pattern_type: AttackPattern,
    pub pattern_timer: Timer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttackPattern {
    CircularShot,     // Bullets in all directions
    TargetedBarrage,  // Multiple shots at player
    SpawnMinions,     // Spawns enemy ships
    AsteroidRain,     // Spawns asteroids
}
```

**Purpose**: Defines boss attack behaviors
**Switching**: Patterns change based on timer and boss phase

## 🎁 Power-up Components

#### `PowerUp`

```rust
#[derive(Component)]
pub struct PowerUp {
    pub power_type: PowerUpType,
    pub collection_timer: Timer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PowerUpType {
    ExtraLife,        // +1 life
    PiercingBullets,  // Bullets pass through enemies
    ExplosiveBullets, // Area damage
    LaserBeam,        // Continuous beam weapon
}
```

**Purpose**: Collectible temporary upgrades
**Timer**: Auto-despawn if not collected

#### `PowerUpEffect`

```rust
#[derive(Component)]
pub struct PowerUpEffect {
    pub effect_type: PowerUpType,
    pub duration: Timer,
}
```

**Purpose**: Active power-up effects on player
**Duration**: Most effects are temporary (10-15 seconds)

## ⚡ Physics Components

#### `Velocity`

```rust
#[derive(Component)]
pub struct Velocity(pub Vec2);
```

**Purpose**: Entity movement per frame
**Usage**: Applied by physics system each frame

#### `RotationVelocity`

```rust
#[derive(Component)]
pub struct RotationVelocity {
    pub angular_velocity: f32,
}
```

**Purpose**: Rotation speed in radians per second
**Common**: Used for asteroids and visual effects

#### `Wraparound`

```rust
#[derive(Component)]
pub struct Wraparound;
```

**Purpose**: Marker for entities that wrap around screen edges
**Behavior**: Teleports to opposite side when leaving screen

## 💚 Health System

#### `Health`

```rust
#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}
```

**Purpose**: Hit points for destructible entities
**Death**: Entity despawns when current reaches 0
**Healing**: Some power-ups can restore health

## ✨ Visual Effects

#### `PulsingEffect`

```rust
#[derive(Component)]
pub struct PulsingEffect {
    pub timer: Timer,
    pub base_scale: f32,
    pub pulse_amplitude: f32,
}
```

**Purpose**: Scale-based pulsing animation
**Usage**: Applied to enemies and enemy bullets for visibility
**Math**: Uses sine wave for smooth scaling

#### `ExplosionVisual`

```rust
#[derive(Component)]
pub struct ExplosionVisual;
```

**Purpose**: Marker for temporary explosion effects
**Cleanup**: Automatically removed after lifecycle expires

## 🎮 UI Components

#### `ScoreText`

```rust
#[derive(Component)]
pub struct ScoreText;
```

**Purpose**: Marker for score display element

#### `LivesText`

```rust
#[derive(Component)]
pub struct LivesText;
```

**Purpose**: Marker for lives counter display

#### `HeartUI`

```rust
#[derive(Component)]
pub struct HeartUI {
    pub heart_index: usize,
}
```

**Purpose**: Individual heart icons for life display
**Index**: Determines which heart this represents

## 🎯 Usage Patterns

### Adding New Components

1. Define in appropriate component file
2. Add to `components/mod.rs` exports
3. Update relevant systems
4. Add to cleanup systems if needed

### Component Queries

```rust
// Single component
Query<&Transform, With<Player>>

// Multiple components
Query<(&Transform, &Velocity), With<Enemy>>

// Excluding components
Query<&Transform, (With<Bullet>, Without<Player>)>

// Mutable access
Query<(&mut Transform, &Velocity)>
```

### Entity Spawning with Components

```rust
commands.spawn((
    Mesh2d(mesh),
    MeshMaterial2d(material),
    Transform::default(),
    Enemy::new(EnemyType::Hunter),
    Health::new(3),
    Velocity(Vec2::ZERO),
    AIBehavior::new(),
    Wraparound,
));
```
