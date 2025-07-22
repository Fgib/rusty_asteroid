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
    pub rapid_fire_count: u32,
    pub multi_shot_count_effects: u32,
    pub piercing_count: u32,
    pub explosive_count: u32,
    pub laser_count: u32,
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
            rapid_fire_count: 0,
            multi_shot_count_effects: 0,
            piercing_count: 0,
            explosive_count: 0,
            laser_count: 0,
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
            PowerUpType::RapidFire => {
                self.rapid_fire_count += 1;
                self.rapid_fire_multiplier = 3.0;
            }
            PowerUpType::MultiShot => {
                self.multi_shot_count_effects += 1;
                self.multi_shot_count = 5;
            }
            PowerUpType::PiercingBullets => {
                self.piercing_count += 1;
                self.has_piercing = true;
            }
            PowerUpType::ExplosiveBullets => {
                self.explosive_count += 1;
                self.has_explosive = true;
            }
            PowerUpType::LaserBeam => {
                self.laser_count += 1;
                self.has_laser = true;
            }
        }
    }

    pub fn remove_effect(&mut self, power_type: &PowerUpType) {
        match power_type {
            PowerUpType::RapidFire => {
                if self.rapid_fire_count > 0 {
                    self.rapid_fire_count -= 1;
                    if self.rapid_fire_count == 0 {
                        self.rapid_fire_multiplier = 1.0;
                        self.active_effects.retain(|p| p != power_type);
                    }
                }
            }
            PowerUpType::MultiShot => {
                if self.multi_shot_count_effects > 0 {
                    self.multi_shot_count_effects -= 1;
                    if self.multi_shot_count_effects == 0 {
                        self.multi_shot_count = 1;
                        self.active_effects.retain(|p| p != power_type);
                    }
                }
            }
            PowerUpType::PiercingBullets => {
                if self.piercing_count > 0 {
                    self.piercing_count -= 1;
                    if self.piercing_count == 0 {
                        self.has_piercing = false;
                        self.active_effects.retain(|p| p != power_type);
                    }
                }
            }
            PowerUpType::ExplosiveBullets => {
                if self.explosive_count > 0 {
                    self.explosive_count -= 1;
                    if self.explosive_count == 0 {
                        self.has_explosive = false;
                        self.active_effects.retain(|p| p != power_type);
                    }
                }
            }
            PowerUpType::LaserBeam => {
                if self.laser_count > 0 {
                    self.laser_count -= 1;
                    if self.laser_count == 0 {
                        self.has_laser = false;
                        self.active_effects.retain(|p| p != power_type);
                    }
                }
            }
        }
    }

    pub fn clear_all(&mut self) {
        self.active_effects.clear();
        self.rapid_fire_count = 0;
        self.multi_shot_count_effects = 0;
        self.piercing_count = 0;
        self.explosive_count = 0;
        self.laser_count = 0;
        self.rapid_fire_multiplier = 1.0;
        self.multi_shot_count = 1;
        self.has_piercing = false;
        self.has_explosive = false;
        self.has_laser = false;
    }
}
