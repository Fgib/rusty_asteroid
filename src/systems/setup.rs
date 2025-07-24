use crate::assets::*;
use crate::components::*;
use crate::resources::*;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
};

pub fn setup_camera(mut commands: Commands) {
    // Spawn camera with HDR and bloom enabled
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // HDR is required for bloom
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface, // Using a tonemapper that desaturates to white is recommended
        Bloom::default(),           // Enable bloom for the camera
    ));
}

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    // Spawn player with triangle mesh
    commands.spawn((
        Mesh2d(meshes.add(create_player_triangle_mesh())),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(2.0, 2.0, 2.0)))), // Bright white for slight bloom
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Player,
        Velocity(Vec2::ZERO),
        Wraparound,
    ));

    // Spawn UI text for score
    commands.spawn((
        Text::new("Score: 0"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreText,
        GameUI,
    ));

    // Spawn UI text for lives
    commands.spawn((
        Text::new("Lives: 3"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        },
        LivesText,
        GameUI,
    ));

    // Get window dimensions for heart positioning
    if let Ok(window) = windows.single() {
        let window_width = window.width();

        // Create heart mesh and material
        let heart_mesh = meshes.add(create_heart_mesh());
        let heart_material = materials.add(ColorMaterial::from(Color::srgb(2.0, 0.3, 0.3))); // Bright red for hearts

        // Spawn 3 hearts in the top right corner
        for i in 0..3 {
            let heart_x = window_width - 60.0 - (i as f32 * 40.0); // Position from right edge
            let heart_y = 280.0; // Near top of screen (adjust based on your window height)

            commands.spawn((
                Mesh2d(heart_mesh.clone()),
                MeshMaterial2d(heart_material.clone()),
                Transform::from_translation(Vec3::new(heart_x, heart_y, 1.0))
                    .with_scale(Vec3::splat(1.5)), // Scale up the hearts
                HeartUI { heart_index: i },
                GameUI,
            ));
        }
    }
}

pub fn cleanup_all_entities(
    mut commands: Commands,
    entity_query: Query<
        Entity,
        Or<(
            With<Player>,
            With<Asteroid>,
            With<Bullet>,
            With<ScoreText>,
            With<LivesText>,
            With<HeartUI>,
            With<PowerUp>,         // Add power-ups to cleanup
            With<PowerUpEffect>,   // Add power-up effects
            With<LaserBeam>,       // Add laser beams
            With<ExplosionVisual>, // Add explosion visuals
            With<Enemy>,           // Add enemies to cleanup
            With<EnemyBullet>,     // Add enemy bullets to cleanup
            With<Boss>,            // Add bosses to cleanup
            With<BossHealthBar>,   // Add boss health bar to cleanup
            With<BossPhaseText>,   // Add boss phase text to cleanup
        )>,
    >,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn reset_game_resources(
    mut score: ResMut<GameScore>,
    mut lives: ResMut<PlayerLives>,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
    mut fire_timer: ResMut<FireTimer>,
    mut player_powerups: ResMut<PlayerPowerUps>, // Add powerup resource reset
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, // Add enemy spawn timer reset
    mut boss_spawn_manager: ResMut<BossSpawnManager>, // Add boss spawn manager reset
    difficulty: Res<DifficultySettings>,
) {
    // Reset score and lives
    score.score = 0;
    *lives = PlayerLives::default();

    // Reset powerups
    player_powerups.clear_all();

    // Reset enemy spawn timer
    enemy_spawn_timer.last_spawn_score = 0;

    // Reset boss spawn manager
    boss_spawn_manager.bosses_spawned.clear();
    boss_spawn_manager.active_boss = None;

    // Reset timers with difficulty settings
    spawn_timer.timer = Timer::from_seconds(difficulty.asteroid_spawn_rate, TimerMode::Repeating);
    fire_timer.timer = Timer::from_seconds(0.05, TimerMode::Repeating);
}
