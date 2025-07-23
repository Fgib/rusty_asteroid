use bevy::prelude::*;

mod assets;
mod components;
mod constants;
mod resources;
mod systems;

use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroid Game".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(GameScore { score: 0 })
        .insert_resource(PlayerLives::default())
        .insert_resource(DifficultySettings::default())
        .insert_resource(PlayerPowerUps::default())
        .insert_resource(PowerUpSpawnTimer::default())
        .insert_resource(GameSettings::load())
        .insert_resource(SaveData::load())
        .insert_resource(AsteroidSizeGenerator::default())
        .insert_resource(AsteroidSpawnTimer {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .insert_resource(FireTimer {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        })
        // Startup systems
        .add_systems(Startup, setup_camera)
        // Menu state systems
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu_styled)
        .add_systems(OnExit(GameState::MainMenu), cleanup_styled_menu)
        .add_systems(OnEnter(GameState::Settings), setup_settings_menu)
        .add_systems(OnExit(GameState::Settings), cleanup_settings_menu)
        .add_systems(
            OnEnter(GameState::DifficultySelect),
            setup_difficulty_menu_styled,
        )
        .add_systems(OnExit(GameState::DifficultySelect), cleanup_styled_menu)
        .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
        .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
        // Game state systems
        .add_systems(
            OnEnter(GameState::Playing),
            (setup_game, reset_game_resources, reset_powerups_system),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_all_entities)
        .add_systems(
            Update,
            (
                player_movement,
                enhanced_player_shoot,
                move_entities,
                rotate_entities,
                wrap_around,
                collision_system,
                laser_collision_system, // New laser collision system
                player_asteroid_collision_system,
                invincibility_visual_system,
                update_bullet_lifecycle,
                despawn_asteroids,
                spawn_asteroids,
                spawn_powerup_system,      // New power-up spawning
                powerup_collection_system, // New power-up collection
                powerup_effect_system,     // New power-up effect management
                update_score_display,
                update_lives_display,
                update_heart_display,
                update_powerup_display, // New power-up UI
                save_game_progress,     // Save system
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            pause_input_system.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
        )
        // Game Over state systems
        .add_systems(OnEnter(GameState::GameOver), setup_game_over_menu_styled)
        .add_systems(OnExit(GameState::GameOver), cleanup_styled_menu)
        // Menu interaction systems (run in all menu states)
        .add_systems(
            Update,
            mesh_menu_button_system.run_if(
                in_state(GameState::MainMenu)
                    .or(in_state(GameState::DifficultySelect))
                    .or(in_state(GameState::GameOver))
                    .or(in_state(GameState::Paused))
                    .or(in_state(GameState::Settings)),
            ),
        )
        .add_systems(Update, apply_graphics_settings)
        .add_systems(OnEnter(GameState::GameOver), save_on_game_over)
        .run();
}
