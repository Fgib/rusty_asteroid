use crate::assets::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use bevy::prelude::*;

struct WeightedGenerator {
    min_size: u32,
    weights: Vec<f32>,
}

impl WeightedGenerator {
    fn new(min_size: u32, max_size: u32, rarity_factor: f32, base_probability: f32) -> Self {
        let range = max_size - min_size + 1;
        let mut weights = Vec::with_capacity(range as usize);
        let mut total_weight = 0.0;

        for i in 0..range {
            let weight = base_probability * (rarity_factor.powi(-(i as i32)));
            weights.push(weight);
            total_weight += weight;
        }

        let mut cumulative = 0.0;
        for weight in &mut weights {
            cumulative += *weight / total_weight;
            *weight = cumulative;
        }

        Self { min_size, weights }
    }

    fn generate(&self) -> u32 {
        let roll = fastrand::f32();

        for (i, &cumulative_prob) in self.weights.iter().enumerate() {
            if roll < cumulative_prob {
                return self.min_size + i as u32;
            }
        }

        self.min_size + self.weights.len() as u32 - 1
    }
}

fn generate_weighted_random(
    min_size: u32,
    max_size: u32,
    rarity_factor: f32,
    base_probability: f32,
) -> u32 {
    let generator = WeightedGenerator::new(min_size, max_size, rarity_factor, base_probability);
    generator.generate()
}

fn generate_asteroid_size() -> u32 {
    generate_weighted_random(1, 10, 2.1, 0.35)
}

pub fn spawn_asteroid_fragments(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    parent_size: u32,
) {
    if parent_size > 1 {
        let fragment_size = parent_size - 1;
        let fragment_radius = fragment_size as f32 * 5.0;

        let fragment_count = if parent_size >= 5 { 3 } else { 2 };

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

            commands.spawn((
                Mesh2d(meshes.add(create_asteroid_mesh(fragment_size, fragment_radius))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(2.5, 1.5, 3.0)))), // Bright purple glow for fragments
                Transform::from_translation(position + offset.extend(0.0)),
                Asteroid {
                    size: fragment_size,
                },
                Health::new(fragment_size),
                Velocity(velocity),
                RotationVelocity::random_slow(), // Add random rotation to fragments
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
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        if let Ok(window) = windows.single() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;

            // Choose random side: 0=top, 1=right, 2=bottom, 3=left
            let side = fastrand::u32(0..4);
            let spawn_offset = 50.0; // Distance outside screen edge to spawn

            let (spawn_pos, velocity) = match side {
                0 => {
                    // Top side - spawn above screen, move towards center with downward bias
                    let x = (fastrand::f32() - 0.5) * window.width();
                    let y = half_height + spawn_offset;
                    let velocity_x = (fastrand::f32() - 0.5) * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier;
                    let velocity_y = -ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        - fastrand::f32() * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier * 0.5;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
                1 => {
                    // Right side - spawn right of screen, move towards center with leftward bias
                    let x = half_width + spawn_offset;
                    let y = (fastrand::f32() - 0.5) * window.height();
                    let velocity_x = -ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        - fastrand::f32() * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier * 0.5;
                    let velocity_y = (fastrand::f32() - 0.5) * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
                2 => {
                    // Bottom side - spawn below screen, move towards center with upward bias
                    let x = (fastrand::f32() - 0.5) * window.width();
                    let y = -half_height - spawn_offset;
                    let velocity_x = (fastrand::f32() - 0.5) * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier;
                    let velocity_y = ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        + fastrand::f32() * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier * 0.5;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
                _ => {
                    // Left side - spawn left of screen, move towards center with rightward bias
                    let x = -half_width - spawn_offset;
                    let y = (fastrand::f32() - 0.5) * window.height();
                    let velocity_x = ASTEROID_SPEED * difficulty.asteroid_speed_multiplier
                        + fastrand::f32() * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier * 0.5;
                    let velocity_y = (fastrand::f32() - 0.5) * ASTEROID_SPEED * difficulty.asteroid_speed_multiplier;
                    (Vec2::new(x, y), Vec2::new(velocity_x, velocity_y))
                }
            };

            let size = generate_asteroid_size();
            let radius = size as f32 * 5.0; // Base radius of 5 units per size level

            // Health based on size: size 1 = 1 HP, size 10 = 10 HP
            let health = Health::new(size);

            // Create glowing color based on asteroid size (bigger = more intense glow)
            let glow_intensity = (size as f32 / 10.0) * 2.0 + 1.5; // 1.5 to 3.5 intensity
            let asteroid_color = Color::srgb(
                glow_intensity * 0.8, // Red component
                glow_intensity * 1.2, // Green component (brighter)
                glow_intensity * 0.6, // Blue component
            );

            commands.spawn((
                Mesh2d(meshes.add(create_asteroid_mesh(size, radius))),
                MeshMaterial2d(materials.add(ColorMaterial::from(asteroid_color))),
                Transform::from_translation(Vec3::new(spawn_pos.x, spawn_pos.y, 0.0)),
                Asteroid { size },
                health,
                Velocity(velocity),
                RotationVelocity::random_slow(), // Add random rotation to asteroids
                Wraparound,                      // Enable wraparound for asteroids
            ));
        }
    }
}
