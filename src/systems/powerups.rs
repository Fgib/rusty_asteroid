use crate::assets::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use bevy::prelude::*;
use std::f32::consts::PI;

// System to reset power-ups when game starts
pub fn reset_powerups_system(
    mut commands: Commands,
    mut player_powerups: ResMut<PlayerPowerUps>,
    powerup_effects: Query<Entity, With<PowerUpEffect>>,
) {
    // Clear all active power-ups
    player_powerups.clear_all();

    // Remove all active power-up effect entities
    for entity in powerup_effects.iter() {
        commands.entity(entity).despawn();
    }
}

// System to spawn power-ups randomly
pub fn spawn_powerup_system(
    mut commands: Commands,
    mut spawn_timer: ResMut<PowerUpSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        // Random chance to spawn a power-up (70% chance)
        if fastrand::f32() < 0.7 {
            spawn_random_powerup(&mut commands, &mut meshes, &mut materials, None);
        }
    }
}

pub fn spawn_random_powerup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Option<Vec3>,
) {
    let power_types = [
        PowerUpType::MultiShot,
        PowerUpType::RapidFire,
        PowerUpType::PiercingBullets,
        PowerUpType::ExplosiveBullets,
        PowerUpType::LaserBeam,
    ];

    let random_type = power_types[fastrand::usize(..power_types.len())].clone();

    // Use provided position or random position on screen
    let (x, y) = if let Some(pos) = position {
        (pos.x, pos.y)
    } else {
        ((fastrand::f32() - 0.5) * 760.0, (fastrand::f32() - 0.5) * 560.0)
    };

    let (color, mesh) = get_powerup_visual(&random_type);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(color))),
        Transform::from_translation(Vec3::new(x, y, 0.0)),
        PowerUp {
            power_type: random_type,
        },
        RotationVelocity::new(2.0), // Slow rotation for visual appeal
        Wraparound,
    ));
}
fn get_powerup_visual(power_type: &PowerUpType) -> (Color, Mesh) {
    match power_type {
        PowerUpType::MultiShot => (Color::srgb(3.0, 5.0, 8.0), create_star_mesh(5, 16.0, 8.0)),
        PowerUpType::RapidFire => (Color::srgb(8.0, 3.0, 0.5), create_diamond_mesh(12.0)),
        PowerUpType::PiercingBullets => (Color::srgb(6.0, 0.5, 8.0), create_triangle_mesh(16.0)),
        PowerUpType::ExplosiveBullets => (Color::srgb(8.0, 1.0, 1.0), create_hexagon_mesh(14.0)),
        PowerUpType::LaserBeam => (Color::srgb(1.0, 8.0, 1.0), create_cross_mesh(16.0)),
    }
}

// System to handle power-up collection
pub fn powerup_collection_system(
    mut commands: Commands,
    powerups: Query<(Entity, &Transform, &PowerUp), With<PowerUp>>,
    players: Query<&Transform, (With<Player>, Without<PowerUp>)>,
    mut player_powerups: ResMut<PlayerPowerUps>,
) {
    for player_transform in players.iter() {
        for (powerup_entity, powerup_transform, powerup) in powerups.iter() {
            let distance = player_transform
                .translation
                .distance(powerup_transform.translation);

            // Collection radius - increased for bigger power-ups
            if distance < 30.0 {
                // Add power-up effect to player
                player_powerups.add_effect(powerup.power_type.clone());

                // Spawn power-up effect component on player
                commands.spawn(PowerUpEffect::new(powerup.power_type.clone(), 15.0)); // 15 second duration

                // Remove power-up from world
                commands.entity(powerup_entity).despawn();
            }
        }
    }
}

// System to manage power-up effect timers
pub fn powerup_effect_system(
    mut commands: Commands,
    mut effects: Query<(Entity, &mut PowerUpEffect)>,
    mut player_powerups: ResMut<PlayerPowerUps>,
    time: Res<Time>,
) {
    for (entity, mut effect) in effects.iter_mut() {
        effect.timer.tick(time.delta());

        if effect.timer.finished() {
            // Remove the effect from player
            player_powerups.remove_effect(&effect.power_type);

            // Remove the effect entity
            commands.entity(entity).despawn();
        }
    }
}

// Enhanced player shooting system with power-ups
pub fn enhanced_player_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fire_timer: ResMut<FireTimer>,
    player_powerups: Res<PlayerPowerUps>,
    difficulty: Res<DifficultySettings>,
    time: Res<Time>,
) {
    // Apply rapid fire effect to timer
    let fire_rate_multiplier = player_powerups.rapid_fire_multiplier;
    fire_timer
        .timer
        .tick(time.delta().mul_f32(fire_rate_multiplier));

    if keyboard_input.pressed(KeyCode::Space) && fire_timer.timer.just_finished() {
        for player_transform in query.iter() {
            let forward = player_transform.up().truncate();

            // Multi-shot logic
            for i in 0..player_powerups.multi_shot_count {
                let angle_offset = if player_powerups.multi_shot_count > 1 {
                    let spread = PI / 6.0; // 30 degree spread
                    let step = spread / (player_powerups.multi_shot_count - 1) as f32;
                    -spread / 2.0 + step * i as f32
                } else {
                    0.0
                };

                // Rotate the bullet direction by the angle offset
                let cos_angle = angle_offset.cos();
                let sin_angle = angle_offset.sin();
                let rotated_forward = Vec2::new(
                    forward.x * cos_angle - forward.y * sin_angle,
                    forward.x * sin_angle + forward.y * cos_angle,
                );

                let bullet_velocity =
                    rotated_forward * BULLET_SPEED * difficulty.bullet_speed_multiplier;
                let spawn_offset = rotated_forward * 15.0;

                let mut bullet_entity = commands.spawn((
                    Mesh2d(meshes.add(create_bullet_arrow_mesh())),
                    MeshMaterial2d(materials.add(get_bullet_material(&player_powerups))),
                    Transform::from_translation(
                        player_transform.translation + spawn_offset.extend(0.0),
                    )
                    .with_rotation(player_transform.rotation * Quat::from_rotation_z(angle_offset)),
                    Bullet,
                    BulletLifecycle::new(10.0, 8.0),
                    Velocity(bullet_velocity),
                    Wraparound,
                ));

                // Add special bullet components based on active power-ups
                if player_powerups.has_piercing {
                    bullet_entity.insert(PiercingBullet::new(10)); // Can pierce through 10 asteroids
                }

                if player_powerups.has_explosive {
                    bullet_entity.insert(ExplosiveBullet::new(50.0, 3)); // 50 pixel radius, 3 damage
                }
            }

            // Handle laser beam separately - fire continuously while space is held
            if player_powerups.has_laser {
                spawn_laser_beam(&mut commands, &mut meshes, &mut materials, player_transform);
            }
        }
    }
}

fn get_bullet_material(player_powerups: &PlayerPowerUps) -> ColorMaterial {
    if player_powerups.has_explosive {
        ColorMaterial::from(Color::srgb(10.0, 2.0, 2.0)) // Red explosive bullets
    } else if player_powerups.has_piercing {
        ColorMaterial::from(Color::srgb(8.0, 2.0, 8.0)) // Purple piercing bullets
    } else {
        ColorMaterial::from(Color::srgb(8.0, 8.0, 2.0)) // Default yellow bullets
    }
}

fn spawn_laser_beam(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_transform: &Transform,
) {
    let forward = player_transform.up().truncate();
    let laser_length = 400.0; // Screen-spanning laser

    // Create laser mesh (elongated rectangle)
    let laser_mesh = Mesh::from(Rectangle::new(4.0, laser_length));

    commands.spawn((
        Mesh2d(meshes.add(laser_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.0, 10.0, 0.0)))), // Bright green
        Transform::from_translation(
            player_transform.translation + (forward * laser_length / 2.0).extend(0.0),
        )
        .with_rotation(player_transform.rotation),
        LaserBeam::new(50.0, laser_length, 4.0), // 50 DPS, 400 range, 4 width
        BulletLifecycle::new(0.05, 10.0),        // Very short lived (50ms)
    ));
}
