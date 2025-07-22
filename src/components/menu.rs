use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct MainMenu;

#[derive(Component, Clone)]
pub struct DifficultyMenu;

#[derive(Component, Clone)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct MenuButton {
    #[allow(dead_code)]
    pub action: MenuAction,
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    #[allow(dead_code)]
    StartGame,
    DifficultySelect,
    SetDifficulty(String), // difficulty name
    PlayAgain,
    QuitToMenu,
    Exit,
}

// Tag components for different UI screens
#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct MenuText;
