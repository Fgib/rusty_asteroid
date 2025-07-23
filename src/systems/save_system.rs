use crate::resources::*;
use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;

// System to save game progress
pub fn save_game_progress(
    mut save_data: ResMut<SaveData>,
    game_score: Res<GameScore>,
    time: Res<Time>,
) {
    // Update high score if current score is higher
    save_data.update_high_score(game_score.score);

    // Add to total play time
    save_data.add_play_time(time.delta_secs());

    // Save periodically (every 10 seconds)
    if time.elapsed_secs() % 10.0 < time.delta_secs() {
        save_data.save();
    }
}

// System to track game statistics
#[allow(dead_code)]
pub fn track_game_stats(
    _save_data: ResMut<SaveData>,
    _commands: Commands,
    // Track when asteroids are destroyed
    _asteroid_query: Query<
        Entity,
        (
            With<crate::components::Asteroid>,
            With<crate::components::Health>,
        ),
    >,
    // Track when power-ups are collected
    _powerup_events: EventReader<PowerUpCollectedEvent>,
) {
    // This system would need to be integrated with existing collision systems
    // to properly track when asteroids are destroyed and power-ups collected
}

// Event for tracking power-up collection
#[allow(dead_code)]
#[derive(Event)]
pub struct PowerUpCollectedEvent;

// System to initialize save data on game start
#[allow(dead_code)]
pub fn initialize_save_system(mut commands: Commands) {
    let save_data = SaveData::load();
    commands.insert_resource(save_data);
}

// System to save on game over
pub fn save_on_game_over(mut save_data: ResMut<SaveData>, game_score: Res<GameScore>) {
    save_data.update_high_score(game_score.score);
    save_data.increment_games_played();
    save_data.save();
}

// System to apply graphics settings
pub fn apply_graphics_settings(
    settings: Res<GameSettings>,
    mut camera_query: Query<(Entity, &Camera, Option<&Bloom>)>,
    mut commands: Commands,
) {
    if settings.is_changed() {
        for (entity, _camera, existing_bloom) in camera_query.iter_mut() {
            if settings.graphics.bloom_enabled {
                // Enable bloom by adding Bloom component if not present
                if existing_bloom.is_none() {
                    commands.entity(entity).insert(Bloom::default());
                }
            } else {
                // Disable bloom by removing Bloom component if present
                if existing_bloom.is_some() {
                    commands.entity(entity).remove::<Bloom>();
                }
            }
        }
    }
}
