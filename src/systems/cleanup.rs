use crate::components::*;
use bevy::prelude::*;

#[allow(dead_code)]
pub fn despawn_bullets(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let half_height = window.height() / 2.0;

        for (entity, transform) in bullets.iter() {
            if transform.translation.y > half_height + 50.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn despawn_asteroids(
    mut commands: Commands,
    asteroids: Query<(Entity, &Transform), With<Asteroid>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let half_height = window.height() / 2.0;

        for (entity, transform) in asteroids.iter() {
            if transform.translation.y < -half_height - 50.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
