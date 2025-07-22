use crate::components::*;
use crate::resources::*;
use crate::systems::spawning::spawn_asteroid_fragments;
use bevy::prelude::*;

pub fn collision_system(
    mut commands: Commands,
    mut bullets: Query<
        (
            Entity,
            &Transform,
            Option<&mut PiercingBullet>,
            Option<&ExplosiveBullet>,
        ),
        (With<Bullet>, Without<Asteroid>),
    >,
    mut asteroids: Query<
        (Entity, &Transform, &Asteroid, &mut Health),
        (With<Asteroid>, Without<Bullet>),
    >,
    mut score: ResMut<GameScore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut bullets_to_remove = Vec::new();
    let mut explosions_to_create = Vec::new();
    let mut asteroids_to_despawn = Vec::new();

    // First pass: check collisions and mark entities for removal
    for (bullet_entity, bullet_transform, mut piercing_bullet, explosive_bullet) in
        bullets.iter_mut()
    {
        let mut should_remove_bullet = false;

        for (asteroid_entity, asteroid_transform, asteroid, mut health) in asteroids.iter_mut() {
            // Skip if this asteroid is already marked for despawn
            if asteroids_to_despawn.contains(&asteroid_entity) {
                continue;
            }

            let collision_radius = asteroid.size as f32 * 5.0 + 5.0;
            let distance = bullet_transform
                .translation
                .distance(asteroid_transform.translation);

            if distance < collision_radius {
                let asteroid_position = asteroid_transform.translation;
                let asteroid_size = asteroid.size;

                // Handle explosive bullets
                if let Some(explosive) = explosive_bullet {
                    explosions_to_create.push((
                        asteroid_position,
                        explosive.explosion_radius,
                        explosive.explosion_damage,
                    ));
                }

                // Damage the asteroid
                let is_destroyed = health.take_damage(1);

                if is_destroyed {
                    asteroids_to_despawn.push(asteroid_entity);
                    score.score += (11 - asteroid_size) * 10;

                    spawn_asteroid_fragments(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        asteroid_position,
                        asteroid_size,
                    );
                } else {
                    score.score += 5;
                }

                // Handle piercing bullets
                if let Some(ref mut piercing) = piercing_bullet {
                    piercing.pierced_count += 1;
                    if piercing.pierced_count >= piercing.max_pierces {
                        should_remove_bullet = true;
                    }
                } else {
                    // Regular bullet - remove on first hit
                    should_remove_bullet = true;
                }

                if should_remove_bullet {
                    break;
                }
            }
        }

        if should_remove_bullet {
            bullets_to_remove.push(bullet_entity);
        }
    }

    // Remove bullets that should be destroyed
    for bullet_entity in bullets_to_remove {
        if let Ok(mut entity_commands) = commands.get_entity(bullet_entity) {
            entity_commands.despawn();
        }
    }

    // Remove asteroids that should be destroyed
    for asteroid_entity in asteroids_to_despawn {
        if let Ok(mut entity_commands) = commands.get_entity(asteroid_entity) {
            entity_commands.despawn();
        }
    }

    // Create explosions (but don't despawn more asteroids here to avoid double-despawn)
    for (explosion_center, explosion_radius, explosion_damage) in explosions_to_create {
        create_explosion_visual(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut asteroids,
            &mut score,
            explosion_center,
            explosion_radius,
            explosion_damage,
        );
    }
}

// System to handle laser beam collisions
pub fn laser_collision_system(
    mut commands: Commands,
    lasers: Query<(Entity, &Transform, &LaserBeam), With<LaserBeam>>,
    mut asteroids: Query<
        (Entity, &Transform, &Asteroid, &mut Health),
        (With<Asteroid>, Without<LaserBeam>),
    >,
    mut score: ResMut<GameScore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (_laser_entity, laser_transform, laser_beam) in lasers.iter() {
        let laser_start = laser_transform.translation.truncate();
        let laser_direction = laser_transform.up().truncate();
        let laser_end = laser_start + laser_direction * laser_beam.max_range;

        for (asteroid_entity, asteroid_transform, asteroid, mut health) in asteroids.iter_mut() {
            let asteroid_pos = asteroid_transform.translation.truncate();
            let asteroid_radius = asteroid.size as f32 * 5.0 + 5.0;

            // Check if asteroid intersects with laser line
            if line_circle_intersection(laser_start, laser_end, asteroid_pos, asteroid_radius) {
                let damage = (laser_beam.damage_per_second * time.delta_secs()) as u32;
                let is_destroyed = health.take_damage(damage.max(1));

                if is_destroyed {
                    let asteroid_position = asteroid_transform.translation;
                    let asteroid_size = asteroid.size;

                    commands.entity(asteroid_entity).despawn();
                    score.score += (11 - asteroid_size) * 10;

                    spawn_asteroid_fragments(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        asteroid_position,
                        asteroid_size,
                    );
                } else {
                    score.score += 1;
                }
            }
        }
    }
}

fn create_explosion_visual(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asteroids: &mut Query<
        (Entity, &Transform, &Asteroid, &mut Health),
        (With<Asteroid>, Without<Bullet>),
    >,
    score: &mut ResMut<GameScore>,
    explosion_center: Vec3,
    explosion_radius: f32,
    explosion_damage: u32,
) {
    // Spawn visual explosion effect
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(explosion_radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.0, 0.5, 0.0, 0.3)))),
        Transform::from_translation(explosion_center),
        BulletLifecycle::new(0.5, 5.0), // Short-lived explosion visual
        ExplosionVisual,                // Add marker component for cleanup
    ));

    // Only damage asteroids, don't despawn them (that's handled in the main collision system)
    for (_asteroid_entity, asteroid_transform, _asteroid, mut health) in asteroids.iter_mut() {
        let distance = explosion_center.distance(asteroid_transform.translation);

        if distance <= explosion_radius {
            let _is_destroyed = health.take_damage(explosion_damage);
            // Note: We don't despawn here to avoid double-despawn issues
            // The main collision system will handle despawning
            score.score += 5; // Small bonus for explosion damage
        }
    }
}

fn line_circle_intersection(
    line_start: Vec2,
    line_end: Vec2,
    circle_center: Vec2,
    circle_radius: f32,
) -> bool {
    let line_vec = line_end - line_start;
    let line_length = line_vec.length();

    if line_length == 0.0 {
        return false;
    }

    let line_unit = line_vec / line_length;
    let to_circle = circle_center - line_start;
    let projection = to_circle.dot(line_unit);

    // Clamp projection to line segment
    let clamped_projection = projection.clamp(0.0, line_length);
    let closest_point = line_start + line_unit * clamped_projection;

    let distance_to_circle = (circle_center - closest_point).length();
    distance_to_circle <= circle_radius
}
