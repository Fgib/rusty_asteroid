use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub bloom_enabled: bool,
    pub vsync_enabled: bool,
    pub fullscreen: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            graphics: GraphicsSettings {
                bloom_enabled: true,
                vsync_enabled: true,
                fullscreen: false,
            },
            audio: AudioSettings {
                master_volume: 1.0,
                sfx_volume: 1.0,
                music_volume: 1.0,
            },
        }
    }
}

impl GameSettings {
    pub fn load() -> Self {
        if let Ok(data) = std::fs::read_to_string("settings.json") {
            if let Ok(settings) = serde_json::from_str(&data) {
                return settings;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write("settings.json", data);
        }
    }
}
