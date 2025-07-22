use crate::components::*;
use crate::resources::*;
use crate::systems::spawning::spawn_asteroid_fragments;
use bevy::prelude::*;

pub fn collision_system(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), (With<Bullet>, Without<Asteroid>)>,
    mut asteroids: Query<
        (Entity, &Transform, &Asteroid, &mut Health),
        (With<Asteroid>, Without<Bullet>),
    >,
    mut score: ResMut<GameScore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bullet_entity, bullet_transform) in bullets.iter() {
        for (asteroid_entity, asteroid_transform, asteroid, mut health) in asteroids.iter_mut() {
            // Collision threshold based on asteroid size
            let collision_radius = asteroid.size as f32 * 5.0 + 5.0; // base radius + buffer

            let distance = bullet_transform
                .translation
                .distance(asteroid_transform.translation);

            if distance < collision_radius {
                // Store asteroid info before potentially despawning
                let asteroid_position = asteroid_transform.translation;
                let asteroid_size = asteroid.size;

                // Remove bullet
                commands.entity(bullet_entity).despawn();

                // Damage the asteroid (1 damage per bullet)
                let is_destroyed = health.take_damage(1);

                if is_destroyed {
                    // Remove asteroid
                    commands.entity(asteroid_entity).despawn();

                    // Add score based on asteroid size (smaller = more points)
                    score.score += (11 - asteroid_size) * 10;

                    // Spawn fragments if asteroid is large enough
                    spawn_asteroid_fragments(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        asteroid_position,
                        asteroid_size,
                    );
                } else {
                    // Asteroid still alive, add smaller score for hitting it
                    score.score += 5;
                }

                break; // Break inner loop since bullet is destroyed
            }
        }
    }
}
