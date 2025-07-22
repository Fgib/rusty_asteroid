use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub high_score: u32,
    pub total_play_time: f32,
    pub games_played: u32,
    pub asteroids_destroyed: u32,
    pub power_ups_collected: u32,
    pub last_difficulty: String,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            high_score: 0,
            total_play_time: 0.0,
            games_played: 0,
            asteroids_destroyed: 0,
            power_ups_collected: 0,
            last_difficulty: "Normal".to_string(),
        }
    }
}

impl SaveData {
    pub fn load() -> Self {
        if let Ok(data) = std::fs::read_to_string("save_data.json") {
            if let Ok(save_data) = serde_json::from_str(&data) {
                return save_data;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write("save_data.json", data);
        }
    }

    pub fn update_high_score(&mut self, new_score: u32) {
        if new_score > self.high_score {
            self.high_score = new_score;
        }
    }

    pub fn increment_games_played(&mut self) {
        self.games_played += 1;
    }

    pub fn add_asteroids_destroyed(&mut self, count: u32) {
        self.asteroids_destroyed += count;
    }

    pub fn increment_power_ups_collected(&mut self) {
        self.power_ups_collected += 1;
    }

    pub fn add_play_time(&mut self, time: f32) {
        self.total_play_time += time;
    }
}
