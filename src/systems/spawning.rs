use crate::assets::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn spawn_asteroid_fragments(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    parent_size: u32,
    parent_type: &AsteroidType,
) {
    if parent_size > 1 {
        let fragment_size = parent_size - 1;
        let fragment_radius = fragment_size as f32 * 5.0;

        // Create an asteroid of the same type as parent (with some chance to change for ice)
        let fragment_type = match parent_type {
            AsteroidType::Ice if fastrand::f32() < 0.3 => AsteroidType::Normal, // 30% chance ice becomes normal
            _ => parent_type.clone(),
        };

        let asteroid = Asteroid::new(fragment_size, fragment_type.clone());
        let fragment_count = asteroid.fragment_count();

        for i in 0..fragment_count {
            // Random velocity for fragments
            let angle = (i as f32 / fragment_count as f32) * 2.0 * std::f32::consts::PI
                + fastrand::f32() * 0.5;
            let speed = ASTEROID_SPEED * 0.5 + fastrand::f32() * ASTEROID_SPEED * 0.5;
            let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

            // Spawn fragment slightly offset from original position
            let offset = Vec2::new(
                (fastrand::f32() - 0.5) * 20.0,
                (fastrand::f32() - 0.5) * 20.0,
            );

            let fragment_asteroid = Asteroid::new(fragment_size, fragment_type.clone());
            let fragment_color = fragment_asteroid.get_color();
            let fragment_health = fragment_asteroid.max_health();

            commands.spawn((
                Mesh2d(meshes.add(create_asteroid_mesh(fragment_size, fragment_radius))),
                MeshMaterial2d(materials.add(ColorMaterial::from(fragment_color))),
                Transform::from_translation(position + offset.extend(0.0)),
                fragment_asteroid,
                Health::new(fragment_health),
                Velocity(velocity),
                RotationVelocity::random_slow(),
                Wraparound,
            ));
        }
    }
}

pub fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    difficulty: Res<DifficultySettings>,
    phase_manager: Res<GamePhaseManager>,
    asteroid_generator: Res<AsteroidSizeGenerator>,
    asteroid_type_generator: Res<AsteroidTypeGenerator>,
) {
    // Apply phase-based multiplier to timer tick speed
    let effective_delta = time
        .delta()
        .mul_f32(phase_manager.asteroid_spawn_multiplier);
    spawn_timer.timer.tick(effective_delta);

    if spawn_timer.timer.just_finished() {
        if let Ok(window) = windows.single() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;

            // Choose random side: 0=top, 1=right, 2=bottom, 3=left
            let side: u8 = fastrand::u8(0..4);
            let spawn_offset = 50.0; // Distance outside screen edge to spawn

            let (spawn_pos, velocity) = match side {
                0 => {
                    // Top side - spawn above screen, move towards center with downward bias
                    let x = (fastrand::f32() - 0.5) * window.width();
                    let y = half_height + spawn_offset;
                    let velocity_x = (fastrand::f32() - 0.5)
                        * ASTEROID_SPEED
                        * difficulty.asteroid_speed_multiplier;
                    let velocity_y = -ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        - fastrand::f32()
                            * ASTEROID_SPEED
                            * difficulty.asteroid_speed_multiplier
                            * 0.5;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
                1 => {
                    // Right side - spawn right of screen, move towards center with leftward bias
                    let x = half_width + spawn_offset;
                    let y = (fastrand::f32() - 0.5) * window.height();
                    let velocity_x = -ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        - fastrand::f32()
                            * ASTEROID_SPEED
                            * difficulty.asteroid_speed_multiplier
                            * 0.5;
                    let velocity_y = (fastrand::f32() - 0.5)
                        * ASTEROID_SPEED
                        * difficulty.asteroid_speed_multiplier;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
                2 => {
                    // Bottom side - spawn below screen, move towards center with upward bias
                    let x = (fastrand::f32() - 0.5) * window.width();
                    let y = -half_height - spawn_offset;
                    let velocity_x = (fastrand::f32() - 0.5)
                        * ASTEROID_SPEED
                        * difficulty.asteroid_speed_multiplier;
                    let velocity_y = ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        + fastrand::f32()
                            * ASTEROID_SPEED
                            * difficulty.asteroid_speed_multiplier
                            * 0.5;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
                _ => {
                    // Left side - spawn left of screen, move towards center with rightward bias
                    let x = -half_width - spawn_offset;
                    let y = (fastrand::f32() - 0.5) * window.height();
                    let velocity_x = ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        + fastrand::f32()
                            * ASTEROID_SPEED
                            * difficulty.asteroid_speed_multiplier
                            * 0.5;
                    let velocity_y = (fastrand::f32() - 0.5)
                        * ASTEROID_SPEED
                        * difficulty.asteroid_speed_multiplier;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
            };

            let size: u32 = asteroid_generator.generate();
            let asteroid_type = asteroid_type_generator.generate();
            let radius = (size * 5).min(50) as f32; // Base radius of 5 units per size level, max 50

            let asteroid = Asteroid::new(size, asteroid_type);
            let asteroid_color = asteroid.get_color();
            let asteroid_health = asteroid.max_health();

            // Apply behavior modifier to velocity
            let behavior_modifier = asteroid.get_behavior_modifier();
            let modified_velocity = velocity * behavior_modifier;

            // Add some erratic movement for crystal asteroids
            let final_velocity = if asteroid.asteroid_type == AsteroidType::Crystal {
                let erratic_factor = Vec2::new(
                    (fastrand::f32() - 0.5) * 40.0,
                    (fastrand::f32() - 0.5) * 40.0,
                );
                modified_velocity + erratic_factor
            } else {
                modified_velocity
            };

            commands.spawn((
                Mesh2d(meshes.add(create_asteroid_mesh(size, radius))),
                MeshMaterial2d(materials.add(ColorMaterial::from(asteroid_color))),
                Transform::from_translation(Vec3::new(spawn_pos.x, spawn_pos.y, 0.0)),
                asteroid,
                Health::new(asteroid_health),
                Velocity(final_velocity),
                RotationVelocity::random_slow(), // Add random rotation to asteroids
                Wraparound,                      // Enable wraparound for asteroids
            ));
        }
    }
}
