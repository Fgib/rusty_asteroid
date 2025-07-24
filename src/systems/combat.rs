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
                    score.score += asteroid.points_value();

                    // Special effects for different asteroid types
                    if asteroid.has_special_destruction() {
                        match asteroid.asteroid_type {
                            AsteroidType::Ice => {
                                // Ice asteroids create a temporary freeze effect
                                create_ice_shatter_effect(&mut commands, &mut meshes, &mut materials, asteroid_position);
                            },
                            AsteroidType::Crystal => {
                                // Crystal asteroids have enhanced power-up spawn chance and visual effect
                                create_crystal_explosion_effect(&mut commands, &mut meshes, &mut materials, asteroid_position);
                                
                                // Higher chance to spawn power-up for crystal
                                if fastrand::f32() < 0.9 {
                                    use crate::systems::powerups::spawn_random_powerup;
                                    spawn_random_powerup(&mut commands, &mut meshes, &mut materials, Some(asteroid_position));
                                }
                            },
                            _ => {}
                        }
                    } else if asteroid.asteroid_type == AsteroidType::Crystal {
                        // Regular crystal power-up chance
                        if fastrand::f32() < 0.8 {
                            use crate::systems::powerups::spawn_random_powerup;
                            spawn_random_powerup(&mut commands, &mut meshes, &mut materials, Some(asteroid_position));
                        }
                    }

                    spawn_asteroid_fragments(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        asteroid_position,
                        asteroid.size,
                        &asteroid.asteroid_type,
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

                    commands.entity(asteroid_entity).despawn();
                    score.score += asteroid.points_value();

                    // Special effects for different asteroid types
                    if asteroid.has_special_destruction() {
                        match asteroid.asteroid_type {
                            AsteroidType::Ice => {
                                create_ice_shatter_effect(&mut commands, &mut meshes, &mut materials, asteroid_position);
                            },
                            AsteroidType::Crystal => {
                                create_crystal_explosion_effect(&mut commands, &mut meshes, &mut materials, asteroid_position);
                                
                                // Higher chance to spawn power-up for crystal
                                if fastrand::f32() < 0.9 {
                                    use crate::systems::powerups::spawn_random_powerup;
                                    spawn_random_powerup(&mut commands, &mut meshes, &mut materials, Some(asteroid_position));
                                }
                            },
                            _ => {}
                        }
                    } else if asteroid.asteroid_type == AsteroidType::Crystal {
                        // Regular crystal power-up chance
                        if fastrand::f32() < 0.8 {
                            use crate::systems::powerups::spawn_random_powerup;
                            spawn_random_powerup(&mut commands, &mut meshes, &mut materials, Some(asteroid_position));
                        }
                    }

                    spawn_asteroid_fragments(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        asteroid_position,
                        asteroid.size,
                        &asteroid.asteroid_type,
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

// System to handle player vs enemy bullet collisions
pub fn player_enemy_bullet_collision_system(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, Option<&mut Invincibility>),
        (With<Player>, Without<EnemyBullet>),
    >,
    enemy_bullets: Query<(Entity, &Transform, &EnemyBullet), (With<EnemyBullet>, Without<Player>)>,
    mut player_lives: ResMut<PlayerLives>,
    score: Res<GameScore>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if let Ok((player_entity, player_transform, mut invincibility_opt)) = player_query.single_mut() {
        // Update invincibility timer if active
        if let Some(ref mut invincibility) = invincibility_opt {
            invincibility.timer.tick(time.delta());

            // Remove invincibility component when timer finishes
            if invincibility.timer.finished() {
                commands.entity(player_entity).remove::<Invincibility>();
            }
        }

        // Check collisions only if player is not invincible
        let is_invincible = invincibility_opt
            .as_ref()
            .map_or(false, |inv| inv.is_active());

        if !is_invincible {
            for (bullet_entity, bullet_transform, _enemy_bullet) in enemy_bullets.iter() {
                let collision_radius = 20.0; // Player collision radius

                let distance = player_transform
                    .translation
                    .distance(bullet_transform.translation);

                if distance < collision_radius {
                    // Player hit by enemy bullet
                    let game_over = player_lives.lose_life();

                    if game_over {
                        // Game over
                        commands.entity(player_entity).despawn();
                        next_state.set(GameState::GameOver);
                        println!("Game Over! Final Score: {}", score.score);
                    } else {
                        // Add invincibility frames
                        commands
                            .entity(player_entity)
                            .insert(Invincibility::default());
                        println!("Player hit by enemy! Lives remaining: {}", player_lives.lives);
                    }

                    // Remove the bullet
                    commands.entity(bullet_entity).despawn();
                    break; // Only handle one collision per frame
                }
            }
        }
    }
}

// System to handle bullet vs enemy collisions
pub fn bullet_enemy_collision_system(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), (With<Bullet>, Without<Enemy>)>,
    mut enemies: Query<(Entity, &Transform, &mut Health, &Enemy), (With<Enemy>, Without<Bullet>)>,
    mut score: ResMut<GameScore>,
) {
    let mut bullets_to_remove = Vec::new();
    let mut enemies_to_remove = Vec::new();

    for (bullet_entity, bullet_transform) in bullets.iter() {
        for (enemy_entity, enemy_transform, mut health, enemy) in enemies.iter_mut() {
            let collision_radius = 25.0; // Enemy collision radius
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < collision_radius {
                let is_destroyed = health.take_damage(1);

                if is_destroyed {
                    enemies_to_remove.push(enemy_entity);
                    score.score += enemy.points_value();
                }

                bullets_to_remove.push(bullet_entity);
                break;
            }
        }
    }

    // Remove entities
    for bullet_entity in bullets_to_remove {
        if let Ok(mut entity_commands) = commands.get_entity(bullet_entity) {
            entity_commands.despawn();
        }
    }

    for enemy_entity in enemies_to_remove {
        if let Ok(mut entity_commands) = commands.get_entity(enemy_entity) {
            entity_commands.despawn();
        }
    }
}

// System to handle bullet vs boss collisions
pub fn bullet_boss_collision_system(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), (With<Bullet>, Without<Boss>)>,
    mut bosses: Query<(Entity, &Transform, &mut Health, &mut Boss), (With<Boss>, Without<Bullet>)>,
    mut score: ResMut<GameScore>,
    mut boss_manager: ResMut<BossSpawnManager>,
) {
    let mut bullets_to_remove = Vec::new();

    for (bullet_entity, bullet_transform) in bullets.iter() {
        for (boss_entity, boss_transform, mut health, mut boss) in bosses.iter_mut() {
            // Skip during phase transitions
            if boss.is_in_transition() {
                continue;
            }

            let collision_radius = 60.0 * boss.size_multiplier; // Boss collision radius
            let distance = bullet_transform
                .translation
                .distance(boss_transform.translation);

            if distance < collision_radius {
                let is_phase_destroyed = health.take_damage(1);

                if is_phase_destroyed {
                    score.score += boss.points_value();
                    
                    // Try to advance to next phase
                    if boss.advance_phase() {
                        // Boss advanced to next phase - update health
                        *health = Health::new(boss.phase_health);
                        
                        // Update visual appearance for new phase
                        // This would be handled by a separate system that updates materials
                    } else {
                        // Boss completely defeated
                        commands.entity(boss_entity).despawn();
                        boss_manager.clear_active_boss();
                    }
                }

                bullets_to_remove.push(bullet_entity);
                break;
            }
        }
    }

    // Remove bullets
    for bullet_entity in bullets_to_remove {
        if let Ok(mut entity_commands) = commands.get_entity(bullet_entity) {
            entity_commands.despawn();
        }
    }
}

// Special effect for ice asteroids shattering
fn create_ice_shatter_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
) {
    // Create multiple small ice particles
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * 2.0 * std::f32::consts::PI;
        let distance = 15.0 + fastrand::f32() * 25.0;
        let particle_pos = position + Vec3::new(
            angle.cos() * distance,
            angle.sin() * distance,
            0.0,
        );

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(2.0 + fastrand::f32() * 3.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.3, 0.8, 1.5, 0.8)))), // Ice blue
            Transform::from_translation(particle_pos),
            BulletLifecycle::new(1.0, 3.0), // Fade out over 1 second
            ExplosionVisual,
            Velocity(Vec2::new(
                angle.cos() * (50.0 + fastrand::f32() * 50.0),
                angle.sin() * (50.0 + fastrand::f32() * 50.0),
            )),
        ));
    }
}

// Special effect for crystal asteroids exploding
fn create_crystal_explosion_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
) {
    // Create a bright purple explosion
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(1.5, 0.3, 1.5, 0.6)))), // Bright purple
        Transform::from_translation(position),
        BulletLifecycle::new(0.8, 6.0), // Bright flash that fades
        ExplosionVisual,
    ));

    // Create sparkling particles
    for i in 0..12 {
        let angle = (i as f32 / 12.0) * 2.0 * std::f32::consts::PI + fastrand::f32() * 0.5;
        let distance = 20.0 + fastrand::f32() * 30.0;
        let particle_pos = position + Vec3::new(
            angle.cos() * distance,
            angle.sin() * distance,
            0.0,
        );

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(1.5))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(2.0, 1.0, 2.0)))), // Bright sparkle
            Transform::from_translation(particle_pos),
            BulletLifecycle::new(1.5, 4.0), // Long-lasting sparkles
            ExplosionVisual,
            Velocity(Vec2::new(
                angle.cos() * (30.0 + fastrand::f32() * 40.0),
                angle.sin() * (30.0 + fastrand::f32() * 40.0),
            )),
        ));
    }
}

// System to handle player bullet vs enemy bullet collisions
pub fn bullet_bullet_collision_system(
    mut commands: Commands,
    player_bullets: Query<(Entity, &Transform), (With<Bullet>, Without<EnemyBullet>)>,
    enemy_bullets: Query<(Entity, &Transform), (With<EnemyBullet>, Without<Bullet>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut player_bullets_to_remove = Vec::new();
    let mut enemy_bullets_to_remove = Vec::new();
    let mut sparks_to_create = Vec::new();

    for (player_bullet_entity, player_bullet_transform) in player_bullets.iter() {
        for (enemy_bullet_entity, enemy_bullet_transform) in enemy_bullets.iter() {
            // Skip if already marked for removal
            if player_bullets_to_remove.contains(&player_bullet_entity) 
                || enemy_bullets_to_remove.contains(&enemy_bullet_entity) {
                continue;
            }

            let collision_radius = 8.0; // Small collision radius for bullet vs bullet
            let distance = player_bullet_transform
                .translation
                .distance(enemy_bullet_transform.translation);

            if distance < collision_radius {
                // Mark both bullets for removal
                player_bullets_to_remove.push(player_bullet_entity);
                enemy_bullets_to_remove.push(enemy_bullet_entity);
                
                // Create spark effect at collision point
                let collision_pos = (player_bullet_transform.translation + enemy_bullet_transform.translation) / 2.0;
                sparks_to_create.push(collision_pos);
                
                // Only check first collision for each bullet
                break;
            }
        }
    }

    // Remove collided bullets
    for bullet_entity in player_bullets_to_remove {
        commands.entity(bullet_entity).despawn();
    }
    
    for bullet_entity in enemy_bullets_to_remove {
        commands.entity(bullet_entity).despawn();
    }

    // Create spark effects
    for collision_pos in sparks_to_create {
        create_bullet_collision_sparks(&mut commands, &mut meshes, &mut materials, collision_pos);
    }
}

// Helper function to create spark effects when bullets collide
fn create_bullet_collision_sparks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
) {
    // Create multiple small sparks
    for i in 0..6 {
        let angle = (i as f32 / 6.0) * 2.0 * std::f32::consts::PI + fastrand::f32() * 0.3;
        let speed = 80.0 + fastrand::f32() * 60.0;
        let spark_pos = position + Vec3::new(
            (fastrand::f32() - 0.5) * 4.0,
            (fastrand::f32() - 0.5) * 4.0,
            0.0,
        );

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(1.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(4.0, 3.0, 1.0)))), // Bright yellow-orange sparks
            Transform::from_translation(spark_pos),
            BulletLifecycle::new(0.8, 3.0), // Quick, bright sparks
            ExplosionVisual,
            Velocity(Vec2::new(
                angle.cos() * speed,
                angle.sin() * speed,
            )),
        ));
    }
}
