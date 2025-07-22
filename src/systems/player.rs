use crate::assets::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    difficulty: Res<DifficultySettings>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Rotation controls
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            transform.rotate_z(PLAYER_ROTATION_SPEED * time.delta_secs());
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            transform.rotate_z(-PLAYER_ROTATION_SPEED * time.delta_secs());
        }

        // Forward thrust
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            // Get forward direction from rotation (up direction in ship's local space)
            let forward = transform.up().truncate();
            velocity.0 +=
                forward * PLAYER_SPEED * difficulty.player_speed_multiplier * time.delta_secs();
        }
        // Backward thrust
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            // Get backward direction from rotation (down direction in ship's local space)
            let backward = -transform.up().truncate();
            velocity.0 +=
                backward * PLAYER_SPEED * difficulty.player_speed_multiplier * time.delta_secs();
        }

        // Apply friction
        velocity.0 *= 0.995;

        // Apply velocity
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut fire_timer: ResMut<FireTimer>,
    difficulty: Res<DifficultySettings>,
    time: Res<Time>,
) {
    fire_timer.timer.tick(time.delta());

    if keyboard_input.pressed(KeyCode::Space) && fire_timer.timer.just_finished() {
        for player_transform in query.iter() {
            // Get forward direction from player rotation
            let forward = player_transform.up().truncate();
            let bullet_velocity = forward * BULLET_SPEED * difficulty.bullet_speed_multiplier;

            // Spawn bullet slightly ahead of player in the direction they're facing
            let spawn_offset = forward * 15.0;

            commands.spawn((
                Mesh2d(meshes.add(create_bullet_arrow_mesh())),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(8.0, 8.0, 2.0)))), // Very bright yellow for bloom effect
                Transform::from_translation(
                    player_transform.translation + spawn_offset.extend(0.0),
                )
                .with_rotation(player_transform.rotation),
                Bullet,
                BulletLifecycle::new(10.0, 8.0), // 10 second lifetime, initial intensity of 8.0
                Velocity(bullet_velocity),
                Wraparound,
            ));
        }
    }
}
