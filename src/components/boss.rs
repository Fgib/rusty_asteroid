use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq)]
pub enum BossType {
    GiantAsteroid,   // Huge asteroid with multiple phases
    AlienMothership, // Alien ship with different attack patterns
}

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

impl Boss {
    pub fn new(boss_type: BossType) -> Self {
        let (max_phases, phase_health, size_multiplier) = match boss_type {
            BossType::GiantAsteroid => (4, 30, 3.5), // More phases and health
            BossType::AlienMothership => (5, 25, 3.0), // Even more phases
        };

        Self {
            boss_type,
            phase: 1,
            max_phases,
            phase_health,
            max_phase_health: phase_health,
            attack_timer: Timer::from_seconds(1.5, TimerMode::Repeating), // Faster initial attacks
            phase_transition_timer: None,
            size_multiplier,
        }
    }

    pub fn new_with_difficulty(boss_type: BossType, difficulty: &crate::resources::DifficultySettings) -> Self {
        let (max_phases, base_phase_health, size_multiplier) = match boss_type {
            BossType::GiantAsteroid => (4, 30, 3.5), // More phases and health
            BossType::AlienMothership => (5, 25, 3.0), // Even more phases
        };

        let phase_health = (base_phase_health as f32 * difficulty.boss_health_multiplier).round() as u32;

        Self {
            boss_type,
            phase: 1,
            max_phases,
            phase_health,
            max_phase_health: phase_health,
            attack_timer: Timer::from_seconds(1.5, TimerMode::Repeating), // Faster initial attacks
            phase_transition_timer: None,
            size_multiplier,
        }
    }

    pub fn points_value(&self) -> u32 {
        let base_points = match self.boss_type {
            BossType::GiantAsteroid => 1500, // Increased rewards
            BossType::AlienMothership => 2000,
        };
        base_points * self.phase
    }

    pub fn get_color(&self) -> Color {
        let intensity = 2.5 + (self.phase as f32 * 0.7); // More dramatic phase changes
        match self.boss_type {
            BossType::GiantAsteroid => {
                Color::srgb(intensity * 1.4, intensity * 0.8, intensity * 0.4)
            }
            BossType::AlienMothership => {
                Color::srgb(intensity * 0.8, intensity * 1.5, intensity * 2.2)
            }
        }
    }

    pub fn advance_phase(&mut self) -> bool {
        if self.phase < self.max_phases {
            self.phase += 1;
            self.phase_health = self.max_phase_health + (self.phase * 5); // Increase health each phase
            self.phase_transition_timer = Some(Timer::from_seconds(2.5, TimerMode::Once));

            // Dramatically increase attack speed with each phase
            let new_attack_interval = 1.5 / (self.phase as f32 * 0.8 + 1.0);
            self.attack_timer =
                Timer::from_seconds(new_attack_interval.max(0.3), TimerMode::Repeating);

            true
        } else {
            false
        }
    }

    pub fn is_in_transition(&self) -> bool {
        self.phase_transition_timer
            .as_ref()
            .map_or(false, |timer| !timer.finished())
    }

    /// Get the current attack pattern based on boss type and phase
    pub fn get_current_attack_pattern(&self) -> AttackPattern {
        match self.boss_type {
            BossType::GiantAsteroid => {
                match self.phase {
                    1 => AttackPattern::AsteroidRain,
                    2 => AttackPattern::CircularShot,
                    3 => AttackPattern::SpawnMinions,
                    _ => AttackPattern::TargetedBarrage, // Final phase
                }
            }
            BossType::AlienMothership => {
                match self.phase {
                    1 => AttackPattern::TargetedBarrage,
                    2 => AttackPattern::SpawnMinions,
                    3 => AttackPattern::CircularShot,
                    4 => AttackPattern::AsteroidRain,
                    _ => AttackPattern::TargetedBarrage, // Final desperate attacks
                }
            }
        }
    }

    /// Returns the movement pattern for the boss
    pub fn get_movement_speed(&self) -> f32 {
        match self.boss_type {
            BossType::GiantAsteroid => 20.0 + (self.phase as f32 * 5.0), // Gradually faster
            BossType::AlienMothership => 30.0 + (self.phase as f32 * 8.0), // More agile
        }
    }
}

#[derive(Component)]
pub struct BossAttackPattern {
    pub pattern_type: AttackPattern,
    pub pattern_timer: Timer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttackPattern {
    CircularShot,    // Shoots bullets in all directions
    TargetedBarrage, // Multiple shots at player
    SpawnMinions,    // Spawns enemy ships
    AsteroidRain,    // Spawns asteroids around the boss
}

impl BossAttackPattern {
    pub fn new(pattern_type: AttackPattern) -> Self {
        let duration = match pattern_type {
            AttackPattern::CircularShot => 1.0,
            AttackPattern::TargetedBarrage => 3.0,
            AttackPattern::SpawnMinions => 2.0,
            AttackPattern::AsteroidRain => 4.0,
        };

        Self {
            pattern_type,
            pattern_timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}
