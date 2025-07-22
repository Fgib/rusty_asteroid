use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    DifficultySelect,
    Playing,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        Self::MainMenu
    }
}
