use crate::components::*;
use bevy::prelude::*;

pub fn move_entities(
    mut query: Query<(&mut Transform, &Velocity), Without<Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

pub fn rotate_entities(mut query: Query<(&mut Transform, &RotationVelocity)>, time: Res<Time>) {
    for (mut transform, rotation_velocity) in query.iter_mut() {
        transform.rotate_z(rotation_velocity.angular_velocity * time.delta_secs());
    }
}

pub fn wrap_around(mut query: Query<&mut Transform, With<Wraparound>>, windows: Query<&Window>) {
    if let Ok(window) = windows.single() {
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;

        for mut transform in query.iter_mut() {
            if transform.translation.x > half_width {
                transform.translation.x = -half_width;
            } else if transform.translation.x < -half_width {
                transform.translation.x = half_width;
            }

            if transform.translation.y > half_height {
                transform.translation.y = -half_height;
            } else if transform.translation.y < -half_height {
                transform.translation.y = half_height;
            }
        }
    }
}
