use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct DifficultySettings {
    pub name: String,
    pub asteroid_spawn_rate: f32, // seconds between spawns
    pub asteroid_speed_multiplier: f32,
    pub player_speed_multiplier: f32,
    pub bullet_speed_multiplier: f32,
}

impl DifficultySettings {
    pub fn easy() -> Self {
        Self {
            name: "Easy".to_string(),
            asteroid_spawn_rate: 3.0,
            asteroid_speed_multiplier: 0.7,
            player_speed_multiplier: 1.2,
            bullet_speed_multiplier: 1.2,
        }
    }

    pub fn normal() -> Self {
        Self {
            name: "Normal".to_string(),
            asteroid_spawn_rate: 2.0,
            asteroid_speed_multiplier: 1.0,
            player_speed_multiplier: 1.0,
            bullet_speed_multiplier: 1.0,
        }
    }

    pub fn hard() -> Self {
        Self {
            name: "Hard".to_string(),
            asteroid_spawn_rate: 1.2,
            asteroid_speed_multiplier: 1.5,
            player_speed_multiplier: 0.8,
            bullet_speed_multiplier: 0.9,
        }
    }

    pub fn insane() -> Self {
        Self {
            name: "Insane".to_string(),
            asteroid_spawn_rate: 0.8,
            asteroid_speed_multiplier: 2.0,
            player_speed_multiplier: 0.6,
            bullet_speed_multiplier: 0.8,
        }
    }
}

impl Default for DifficultySettings {
    fn default() -> Self {
        Self::normal()
    }
}
