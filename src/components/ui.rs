use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

#[derive(Component)]
pub struct HeartUI {
    pub heart_index: usize,
}

#[derive(Component)]
pub struct PowerUpDisplay;

// Menu UI components
#[derive(Component, Clone)]
pub struct MenuUI;

#[derive(Component, Clone)]
pub struct PauseUI;

// Settings button types
#[derive(Component)]
pub enum SettingsButton {
    BloomToggle,
    VsyncToggle,
    Back,
}

// Pause button types
#[derive(Component)]
pub enum PauseButton {
    Resume,
    Settings,
    MainMenu,
}
