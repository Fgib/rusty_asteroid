use crate::components::powerup::PowerUpType;
use bevy::prelude::*;

#[derive(Resource)]
pub struct PowerUpSpawnTimer {
    pub timer: Timer,
}

impl Default for PowerUpSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(10.0, TimerMode::Repeating), // Spawn power-up every 10 seconds
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerPowerUps {
    pub active_effects: Vec<PowerUpType>,
    pub rapid_fire_multiplier: f32,
    pub multi_shot_count: u32,
    pub has_piercing: bool,
    pub has_explosive: bool,
    pub has_laser: bool,
}

impl PlayerPowerUps {
    pub fn new() -> Self {
        Self {
            active_effects: Vec::new(),
            rapid_fire_multiplier: 1.0,
            multi_shot_count: 1,
            has_piercing: false,
            has_explosive: false,
            has_laser: false,
        }
    }

    pub fn add_effect(&mut self, power_type: PowerUpType) {
        if !self.active_effects.contains(&power_type) {
            self.active_effects.push(power_type.clone());
        }

        match power_type {
            PowerUpType::RapidFire => self.rapid_fire_multiplier = 3.0,
            PowerUpType::MultiShot => self.multi_shot_count = 5,
            PowerUpType::PiercingBullets => self.has_piercing = true,
            PowerUpType::ExplosiveBullets => self.has_explosive = true,
            PowerUpType::LaserBeam => self.has_laser = true,
        }
    }

    pub fn remove_effect(&mut self, power_type: &PowerUpType) {
        self.active_effects.retain(|p| p != power_type);

        match power_type {
            PowerUpType::RapidFire => self.rapid_fire_multiplier = 1.0,
            PowerUpType::MultiShot => self.multi_shot_count = 1,
            PowerUpType::PiercingBullets => self.has_piercing = false,
            PowerUpType::ExplosiveBullets => self.has_explosive = false,
            PowerUpType::LaserBeam => self.has_laser = false,
        }
    }

    pub fn clear_all(&mut self) {
        self.active_effects.clear();
        self.rapid_fire_multiplier = 1.0;
        self.multi_shot_count = 1;
        self.has_piercing = false;
        self.has_explosive = false;
        self.has_laser = false;
    }
}
