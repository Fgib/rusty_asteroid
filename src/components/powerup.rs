use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq)]
pub enum PowerUpType {
    MultiShot,
    RapidFire,
    PiercingBullets,
    ExplosiveBullets,
    LaserBeam,
}

#[derive(Component)]
pub struct PowerUp {
    pub power_type: PowerUpType,
}

#[derive(Component)]
pub struct PowerUpEffect {
    pub power_type: PowerUpType,
    pub timer: Timer,
    #[allow(dead_code)]
    pub duration: f32,
}

impl PowerUpEffect {
    pub fn new(power_type: PowerUpType, duration: f32) -> Self {
        Self {
            power_type,
            timer: Timer::from_seconds(duration, TimerMode::Once),
            duration,
        }
    }

    pub fn time_remaining(&self) -> f32 {
        self.timer.remaining_secs()
    }
}

// Enhanced bullet types
#[derive(Component)]
pub struct PiercingBullet {
    pub pierced_count: u32,
    pub max_pierces: u32,
}

impl PiercingBullet {
    pub fn new(max_pierces: u32) -> Self {
        Self {
            pierced_count: 0,
            max_pierces,
        }
    }
}

#[derive(Component)]
pub struct ExplosiveBullet {
    pub explosion_radius: f32,
    pub explosion_damage: u32,
}

impl ExplosiveBullet {
    pub fn new(radius: f32, damage: u32) -> Self {
        Self {
            explosion_radius: radius,
            explosion_damage: damage,
        }
    }
}

#[derive(Component)]
pub struct LaserBeam {
    pub damage_per_second: f32,
    pub max_range: f32,
    #[allow(dead_code)]
    pub width: f32,
}

impl LaserBeam {
    pub fn new(damage_per_second: f32, max_range: f32, width: f32) -> Self {
        Self {
            damage_per_second,
            max_range,
            width,
        }
    }
}

// Power-up spawn component
#[derive(Component)]
pub struct PowerUpSpawner;

// Explosion visual marker component
#[derive(Component)]
pub struct ExplosionVisual;
