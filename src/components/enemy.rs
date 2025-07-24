use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq)]
pub enum EnemyType {
    Hunter,   // Basic AI ship that hunts the player
    Bomber,   // Shoots explosive projectiles
    Interceptor, // Fast ship that tries to ram the player
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub last_shot_time: f32,
    pub shot_cooldown: f32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        let (speed, shot_cooldown) = match enemy_type {
            EnemyType::Hunter => (80.0, 1.5),   // Faster shooting
            EnemyType::Bomber => (60.0, 2.5),   // Slower but powerful
            EnemyType::Interceptor => (150.0, 4.0), // Fast but shoots rarely
        };

        Self {
            enemy_type,
            speed,
            last_shot_time: 0.0,
            shot_cooldown,
        }
    }

    pub fn new_with_difficulty(enemy_type: EnemyType, difficulty: &crate::resources::DifficultySettings) -> Self {
        let (base_speed, shot_cooldown) = match enemy_type {
            EnemyType::Hunter => (80.0, 1.5),   // Faster shooting
            EnemyType::Bomber => (60.0, 2.5),   // Slower but powerful
            EnemyType::Interceptor => (150.0, 4.0), // Fast but shoots rarely
        };

        Self {
            enemy_type,
            speed: base_speed * difficulty.enemy_speed_multiplier,
            last_shot_time: 0.0,
            shot_cooldown: shot_cooldown / difficulty.enemy_speed_multiplier.max(0.5), // Faster shooting on higher difficulty
        }
    }

    pub fn points_value(&self) -> u32 {
        match self.enemy_type {
            EnemyType::Hunter => 150,    // Increased points
            EnemyType::Bomber => 250,    // Higher reward for tougher enemy
            EnemyType::Interceptor => 200, // Fast and dangerous
        }
    }

    pub fn max_health(&self) -> u32 {
        match self.enemy_type {
            EnemyType::Hunter => 2,
            EnemyType::Bomber => 4,      // Increased health for bomber
            EnemyType::Interceptor => 1, // Fast but fragile
        }
    }

    pub fn max_health_with_difficulty(&self, difficulty: &crate::resources::DifficultySettings) -> u32 {
        let base_health = match self.enemy_type {
            EnemyType::Hunter => 2,
            EnemyType::Bomber => 4,      // Increased health for bomber
            EnemyType::Interceptor => 1, // Fast but fragile
        };
        (base_health as f32 * difficulty.enemy_health_multiplier).round() as u32
    }

    pub fn get_color(&self) -> Color {
        match self.enemy_type {
            EnemyType::Hunter => Color::srgb(5.0, 1.0, 1.0),   // Much brighter red with bloom
            EnemyType::Bomber => Color::srgb(5.0, 3.0, 1.0),   // Much brighter orange with bloom
            EnemyType::Interceptor => Color::srgb(1.0, 5.0, 1.0), // Much brighter green with bloom
        }
    }

    /// Get the preferred engagement range for this enemy type
    pub fn get_engagement_range(&self) -> f32 {
        match self.enemy_type {
            EnemyType::Hunter => 300.0,    // Medium range hunter
            EnemyType::Bomber => 400.0,    // Long range bomber
            EnemyType::Interceptor => 150.0, // Close range interceptor
        }
    }

    /// Get the evasion tendency for this enemy type
    pub fn get_evasion_factor(&self) -> f32 {
        match self.enemy_type {
            EnemyType::Hunter => 0.3,      // Moderate evasion
            EnemyType::Bomber => 0.1,      // Low evasion, relies on armor
            EnemyType::Interceptor => 0.7, // High evasion, hit and run
        }
    }
}

#[derive(Component)]
pub struct EnemyBullet {
    pub damage: u32,
    pub is_explosive: bool,
}

impl EnemyBullet {
    pub fn new(enemy_type: &EnemyType) -> Self {
        let (damage, is_explosive) = match enemy_type {
            EnemyType::Hunter => (1, false),
            EnemyType::Bomber => (2, true),
            EnemyType::Interceptor => (1, false),
        };

        Self { damage, is_explosive }
    }
}

#[derive(Component)]
pub struct PulsingEffect {
    pub timer: Timer,
    pub base_scale: f32,
    pub pulse_amplitude: f32,
}

impl PulsingEffect {
    pub fn new(pulse_speed: f32, amplitude: f32) -> Self {
        Self {
            timer: Timer::from_seconds(pulse_speed, TimerMode::Repeating),
            base_scale: 1.0,
            pulse_amplitude: amplitude,
        }
    }
}

// AI behavior component
#[derive(Component)]
pub struct AIBehavior {
    pub target_position: Vec2,
    pub behavior_timer: Timer,
    pub state: AIState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AIState {
    Hunting,     // Actively pursuing the player
    Attacking,   // In combat range, aggressive
    Evading,     // Taking evasive action
    Circling,    // Circling around player
    Retreating,  // Moving to safe distance
    Ambushing,   // Waiting for opportune moment
}

impl AIBehavior {
    pub fn new() -> Self {
        Self {
            target_position: Vec2::ZERO,
            behavior_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            state: AIState::Hunting,
        }
    }

    /// Update AI behavior based on distance to player and enemy type
    pub fn update_behavior(&mut self, distance_to_player: f32, enemy_type: &EnemyType, player_velocity: Vec2) {
        let engagement_range = match enemy_type {
            EnemyType::Hunter => 300.0,
            EnemyType::Bomber => 400.0,
            EnemyType::Interceptor => 150.0,
        };

        // State machine logic based on distance and enemy type
        self.state = match enemy_type {
            EnemyType::Hunter => {
                if distance_to_player > engagement_range * 1.5 {
                    AIState::Hunting
                } else if distance_to_player > engagement_range * 0.8 {
                    AIState::Attacking
                } else if player_velocity.length() > 100.0 {
                    AIState::Evading
                } else {
                    AIState::Circling
                }
            },
            EnemyType::Bomber => {
                if distance_to_player > engagement_range {
                    AIState::Hunting
                } else if distance_to_player > engagement_range * 0.6 {
                    AIState::Attacking
                } else {
                    AIState::Retreating // Bombers prefer long range
                }
            },
            EnemyType::Interceptor => {
                if distance_to_player > engagement_range * 2.0 {
                    AIState::Ambushing
                } else if distance_to_player > engagement_range {
                    AIState::Hunting
                } else if distance_to_player < engagement_range * 0.5 {
                    AIState::Retreating
                } else {
                    AIState::Attacking
                }
            }
        };
    }
}
