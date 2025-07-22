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
        .add_systems(
            OnEnter(GameState::DifficultySelect),
            setup_difficulty_menu_styled,
        )
        .add_systems(OnExit(GameState::DifficultySelect), cleanup_styled_menu)
        // Game state systems
        .add_systems(
            OnEnter(GameState::Playing),
            (setup_game, reset_game_resources),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_all_entities)
        .add_systems(
            Update,
            (
                player_movement,
                player_shoot,
                move_entities,
                rotate_entities,
                wrap_around,
                collision_system,
                player_asteroid_collision_system,
                invincibility_visual_system,
                update_bullet_lifecycle,
                despawn_asteroids,
                spawn_asteroids,
                update_score_display,
                update_lives_display,
                update_heart_display,
            )
                .run_if(in_state(GameState::Playing)),
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
                    .or(in_state(GameState::GameOver)),
            ),
        )
        .run();
}
