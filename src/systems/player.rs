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
