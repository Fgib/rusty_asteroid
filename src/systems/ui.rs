use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_score_display(score: Res<GameScore>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("Score: {}", score.score);
        }
    }
}

pub fn update_lives_display(lives: Res<PlayerLives>, mut query: Query<&mut Text, With<LivesText>>) {
    if lives.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("Lives: {}", lives.lives);
        }
    }
}

pub fn update_heart_display(
    lives: Res<PlayerLives>,
    mut heart_query: Query<(&mut Visibility, &HeartUI)>,
) {
    if lives.is_changed() {
        for (mut visibility, heart_ui) in heart_query.iter_mut() {
            // Show heart if the heart index is less than current lives
            *visibility = if heart_ui.heart_index < lives.lives as usize {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
