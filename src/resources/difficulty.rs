use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct DifficultySettings {
    pub name: String,
    pub asteroid_spawn_rate: f32, // seconds between spawns
    pub asteroid_speed_multiplier: f32,
    pub player_speed_multiplier: f32,
    pub bullet_speed_multiplier: f32,
    pub enemy_health_multiplier: f32, // Multiplier for enemy health
    pub enemy_speed_multiplier: f32,  // Multiplier for enemy speed
    pub enemy_damage_multiplier: f32, // Multiplier for enemy damage
    pub boss_health_multiplier: f32,  // Multiplier for boss health
}

impl DifficultySettings {
    pub fn easy() -> Self {
        Self {
            name: "Easy".to_string(),
            asteroid_spawn_rate: 2.0,
            asteroid_speed_multiplier: 1.0,
            player_speed_multiplier: 1.0,
            bullet_speed_multiplier: 1.0,
            enemy_health_multiplier: 0.7, // 30% less health
            enemy_speed_multiplier: 0.8,  // 20% slower
            enemy_damage_multiplier: 0.8, // 20% less damage
            boss_health_multiplier: 0.7,  // 30% less health
        }
    }

    pub fn normal() -> Self {
        Self {
            name: "Normal".to_string(),
            asteroid_spawn_rate: 0.75,
            asteroid_speed_multiplier: 2.0,
            player_speed_multiplier: 0.6,
            bullet_speed_multiplier: 0.8,
            enemy_health_multiplier: 1.0, // Normal health
            enemy_speed_multiplier: 1.0,  // Normal speed
            enemy_damage_multiplier: 1.0, // Normal damage
            boss_health_multiplier: 1.0,  // Normal health
        }
    }

    pub fn hard() -> Self {
        Self {
            name: "Hard".to_string(),
            asteroid_spawn_rate: 0.4,
            asteroid_speed_multiplier: 2.5,
            player_speed_multiplier: 0.5,
            bullet_speed_multiplier: 0.7,
            enemy_health_multiplier: 1.3, // 30% more health
            enemy_speed_multiplier: 1.2,  // 20% faster
            enemy_damage_multiplier: 1.3, // 30% more damage
            boss_health_multiplier: 1.4,  // 40% more health
        }
    }

    pub fn insane() -> Self {
        Self {
            name: "Insane".to_string(),
            asteroid_spawn_rate: 0.2,
            asteroid_speed_multiplier: 3.0,
            player_speed_multiplier: 0.4,
            bullet_speed_multiplier: 0.6,
            enemy_health_multiplier: 1.6, // 60% more health
            enemy_speed_multiplier: 1.5,  // 50% faster
            enemy_damage_multiplier: 1.5, // 50% more damage
            boss_health_multiplier: 1.8,  // 80% more health
        }
    }
}

impl Default for DifficultySettings {
    fn default() -> Self {
        Self::normal()
    }
}
