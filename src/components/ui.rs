use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

#[derive(Component)]
pub struct HeartUI {
    pub heart_index: usize, // Which heart this is (0, 1, 2)
}
