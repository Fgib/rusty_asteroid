use crate::assets::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use bevy::prelude::*;

// System to spawn enemies based on time and score
pub fn spawn_enemy_system(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    score: Res<GameScore>,
    difficulty: Res<DifficultySettings>,
    windows: Query<&Window>,
) {
    spawn_timer.timer.tick(time.delta());

    let should_spawn = spawn_timer.timer.just_finished()
        || (score.score >= spawn_timer.last_spawn_score + spawn_timer.spawn_score_interval);

    if should_spawn {
        if let Ok(window) = windows.single() {
            spawn_timer.last_spawn_score = score.score;

            // Choose enemy type based on score
            let enemy_type = if score.score < 1000 {
                EnemyType::Hunter
            } else if score.score < 3000 {
                if fastrand::f32() < 0.7 {
                    EnemyType::Hunter
                } else {
                    EnemyType::Bomber
                }
            } else {
                match fastrand::u32(0..3) {
                    0 => EnemyType::Hunter,
                    1 => EnemyType::Bomber,
                    _ => EnemyType::Interceptor,
                }
            };

            spawn_enemy(
                &mut commands,
                &mut meshes,
                &mut materials,
                &enemy_type,
                &window,
                &difficulty,
            );
        }
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    enemy_type: &EnemyType,
    window: &Window,
    difficulty: &DifficultySettings,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    let spawn_offset = 100.0;

    // Spawn from random edge
    let side = fastrand::u32(0..4);
    let spawn_pos = match side {
        0 => Vec2::new(
            (fastrand::f32() - 0.5) * window.width(),
            half_height + spawn_offset,
        ), // Top
        1 => Vec2::new(
            half_width + spawn_offset,
            (fastrand::f32() - 0.5) * window.height(),
        ), // Right
        2 => Vec2::new(
            (fastrand::f32() - 0.5) * window.width(),
            -half_height - spawn_offset,
        ), // Bottom
        _ => Vec2::new(
            -half_width - spawn_offset,
            (fastrand::f32() - 0.5) * window.height(),
        ), // Left
    };

    let enemy = Enemy::new_with_difficulty(enemy_type.clone(), difficulty);
    let enemy_color = enemy.get_color();
    let enemy_health = enemy.max_health_with_difficulty(difficulty);

    commands.spawn((
        Mesh2d(meshes.add(create_enemy_ship_mesh(enemy_type))),
        MeshMaterial2d(materials.add(ColorMaterial::from(enemy_color))),
        Transform::from_translation(spawn_pos.extend(0.0)),
        enemy,
        Health::new(enemy_health),
        Velocity(Vec2::ZERO), // Will be set by AI system
        AIBehavior::new(),
        PulsingEffect::new(0.5, 0.15), // Subtle pulse for enemy ships
        Wraparound,
    ));
}

// AI system for enemy behavior - Enhanced with more sophisticated AI
pub fn enemy_ai_system(
    mut enemy_query: Query<
        (&mut Transform, &mut Velocity, &mut AIBehavior, &Enemy),
        (With<Enemy>, Without<Player>),
    >,
    player_query: Query<(&Transform, &Velocity), (With<Player>, Without<Enemy>)>,
    asteroid_query: Query<&Transform, (With<Asteroid>, Without<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok((player_transform, player_velocity)) = player_query.single() {
        let player_pos = player_transform.translation.truncate();
        let player_vel = player_velocity.0;

        for (mut enemy_transform, mut velocity, mut ai_behavior, enemy) in enemy_query.iter_mut() {
            ai_behavior.behavior_timer.tick(time.delta());

            let enemy_pos = enemy_transform.translation.truncate();
            let distance_to_player = player_pos.distance(enemy_pos);
            let to_player = (player_pos - enemy_pos).normalize_or_zero();

            // Update AI behavior based on context
            if ai_behavior.behavior_timer.just_finished() {
                ai_behavior.update_behavior(distance_to_player, &enemy.enemy_type, player_vel);
                ai_behavior.target_position = player_pos;
            }

            // Predictive targeting - anticipate player movement
            let prediction_time = distance_to_player / (enemy.speed * 1.5);
            let predicted_player_pos = player_pos + player_vel * prediction_time;
            let to_predicted_player = (predicted_player_pos - enemy_pos).normalize_or_zero();

            // Obstacle avoidance - basic asteroid avoidance
            let mut avoidance_vector = Vec2::ZERO;
            for asteroid_transform in asteroid_query.iter() {
                let asteroid_pos = asteroid_transform.translation.truncate();
                let to_asteroid = enemy_pos - asteroid_pos;
                let asteroid_distance = to_asteroid.length();

                if asteroid_distance < 80.0 && asteroid_distance > 0.1 {
                    avoidance_vector += to_asteroid.normalize() * (80.0 - asteroid_distance) / 80.0;
                }
            }

            // Apply behavior based on current state
            let desired_velocity = match ai_behavior.state {
                AIState::Hunting => {
                    // Move towards predicted player position
                    to_predicted_player * enemy.speed
                }
                AIState::Attacking => {
                    // Aggressive movement with slight randomness
                    let aggression_bonus = 1.0 + (enemy.get_evasion_factor() * 0.5);
                    let random_offset =
                        Vec2::new((fastrand::f32() - 0.5) * 0.3, (fastrand::f32() - 0.5) * 0.3);
                    (to_predicted_player + random_offset) * enemy.speed * aggression_bonus
                }
                AIState::Evading => {
                    // Evasive maneuvers - move perpendicular to player direction
                    let perpendicular = Vec2::new(-to_player.y, to_player.x);
                    let evasion_strength = enemy.get_evasion_factor();
                    (-to_player + perpendicular * 2.0) * enemy.speed * evasion_strength
                }
                AIState::Circling => {
                    // Circle around player at optimal range
                    let optimal_range = enemy.get_engagement_range();
                    let range_factor = if distance_to_player < optimal_range {
                        -0.5
                    } else {
                        0.3
                    };
                    let perpendicular = Vec2::new(-to_player.y, to_player.x);
                    (to_player * range_factor + perpendicular * 1.5) * enemy.speed
                }
                AIState::Retreating => {
                    // Move away to safe distance
                    -to_player * enemy.speed * 1.2
                }
                AIState::Ambushing => {
                    // Move slowly to ambush position
                    let ambush_factor = if distance_to_player > enemy.get_engagement_range() * 1.5 {
                        0.3
                    } else {
                        1.5
                    };
                    to_predicted_player * enemy.speed * ambush_factor
                }
            };

            // Combine desired movement with obstacle avoidance
            let final_velocity = desired_velocity + avoidance_vector * enemy.speed * 0.5;
            velocity.0 = final_velocity;

            // Rotate to face movement direction
            if velocity.0.length() > 0.1 {
                let angle = velocity.0.y.atan2(velocity.0.x) - std::f32::consts::PI / 2.0;
                enemy_transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}

// System for enemy shooting
pub fn enemy_shooting_system(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_pos = player_transform.translation.truncate();
        let current_time = time.elapsed_secs();

        for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if current_time - enemy.last_shot_time >= enemy.shot_cooldown {
                let enemy_pos = enemy_transform.translation.truncate();
                let to_player = (player_pos - enemy_pos).normalize();

                // Only shoot if player is relatively close and in front
                let distance = player_pos.distance(enemy_pos);
                if distance < 600.0 {
                    // Increased from 400.0 to make enemies more aggressive
                    let bullet_velocity = to_player * BULLET_SPEED * 0.8;
                    let enemy_bullet = EnemyBullet::new(&enemy.enemy_type);

                    let bullet_color = match enemy.enemy_type {
                        EnemyType::Hunter => Color::srgb(6.0, 2.0, 2.0), // Much brighter red
                        EnemyType::Bomber => Color::srgb(6.0, 4.0, 1.0), // Much brighter orange
                        EnemyType::Interceptor => Color::srgb(2.0, 6.0, 2.0), // Much brighter green
                    };

                    commands.spawn((
                        Mesh2d(meshes.add(create_enemy_bullet_mesh())),
                        MeshMaterial2d(materials.add(ColorMaterial::from(bullet_color))),
                        Transform::from_translation(enemy_transform.translation).with_rotation(
                            Quat::from_rotation_z(
                                to_player.y.atan2(to_player.x) - std::f32::consts::FRAC_PI_2,
                            ),
                        ),
                        enemy_bullet,
                        Velocity(bullet_velocity),
                        BulletLifecycle::new(3.0, 5.0),
                        PulsingEffect::new(0.3, 0.3), // Quick pulse for visibility
                    ));

                    enemy.last_shot_time = current_time;
                }
            }
        }
    }
}

// System for boss spawning
pub fn boss_spawn_system(
    mut commands: Commands,
    mut boss_manager: ResMut<BossSpawnManager>,
    mut phase_manager: ResMut<GamePhaseManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    score: Res<GameScore>,
    difficulty: Res<DifficultySettings>,
    windows: Query<&Window>,
) {
    if let Some(boss_type) = boss_manager.should_spawn_boss(score.score) {
        if let Ok(window) = windows.single() {
            // Enter boss encounter phase when spawning a boss
            phase_manager.enter_boss_encounter();
            
            let spawn_pos = Vec2::new(0.0, window.height() / 2.0 + 150.0); // Spawn above screen center

            let boss = Boss::new_with_difficulty(boss_type.clone(), &difficulty);
            let boss_color = boss.get_color();
            let boss_health = boss.phase_health;

            let entity = commands
                .spawn((
                    Mesh2d(meshes.add(create_boss_mesh(&boss_type, boss.size_multiplier))),
                    MeshMaterial2d(materials.add(ColorMaterial::from(boss_color))),
                    Transform::from_translation(spawn_pos.extend(0.0)),
                    boss,
                    Health::new(boss_health),
                    Velocity(Vec2::new(0.0, -30.0)), // Slow descent
                    BossAttackPattern::new(AttackPattern::CircularShot),
                ))
                .id();

            boss_manager.mark_boss_spawned(boss_type, entity);
        }
    }
}

// System to manage game phases based on boss presence
pub fn game_phase_manager_system(
    mut phase_manager: ResMut<GamePhaseManager>,
    mut boss_manager: ResMut<BossSpawnManager>,
    boss_query: Query<Entity, With<Boss>>,
    time: Res<Time>,
) {
    phase_manager.update(time.delta());
    
    // Check if there are any bosses still alive
    let boss_count = boss_query.iter().count();
    
    if boss_count == 0 && phase_manager.current_phase == GamePhase::BossEncounter {
        // No bosses left, return to normal phase
        phase_manager.enter_normal_phase();
        boss_manager.active_boss = None; // Clear active boss reference
    } else if boss_count > 0 && phase_manager.current_phase == GamePhase::Normal {
        // Boss detected but we're in normal phase, switch to boss encounter
        phase_manager.enter_boss_encounter();
    }
}

// System to reset game phase when starting a new game
pub fn reset_game_phase_system(mut phase_manager: ResMut<GamePhaseManager>) {
    *phase_manager = GamePhaseManager::default();
}

// System for boss AI and attacks
pub fn boss_ai_system(
    mut commands: Commands,
    mut boss_query: Query<(
        Entity,
        &Transform,
        &mut Boss,
        &mut BossAttackPattern,
        &mut Velocity,
    )>,
    player_query: Query<&Transform, (With<Player>, Without<Boss>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    _boss_manager: ResMut<BossSpawnManager>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_pos = player_transform.translation.truncate();

        for (_boss_entity, boss_transform, mut boss, mut attack_pattern, mut velocity) in
            boss_query.iter_mut()
        {
            // Update timers
            boss.attack_timer.tick(time.delta());
            attack_pattern.pattern_timer.tick(time.delta());

            if let Some(ref mut transition_timer) = boss.phase_transition_timer {
                transition_timer.tick(time.delta());
            }

            // Skip behavior during phase transitions
            if boss.is_in_transition() {
                velocity.0 = Vec2::ZERO;
                continue;
            }

            let boss_pos = boss_transform.translation.truncate();

            // Enhanced movement pattern based on boss type and phase
            let movement_speed = boss.get_movement_speed();
            let to_player = player_pos - boss_pos;
            let distance = to_player.length();

            let desired_distance = match boss.boss_type {
                BossType::GiantAsteroid => 180.0 + (boss.phase as f32 * 20.0), // Gets closer each phase
                BossType::AlienMothership => 220.0, // Prefers longer range
            };

            // Dynamic movement based on distance and boss behavior
            let movement_vector = if distance > desired_distance + 50.0 {
                // Move toward player
                to_player.normalize() * movement_speed
            } else if distance < desired_distance - 50.0 {
                // Move away from player
                -to_player.normalize() * movement_speed * 0.8
            } else {
                // Orbital movement around optimal distance
                let perpendicular = Vec2::new(-to_player.y, to_player.x).normalize();
                let orbit_direction = if (time.elapsed_secs() * 0.3).sin() > 0.0 {
                    1.0
                } else {
                    -1.0
                };
                perpendicular * movement_speed * 0.6 * orbit_direction
            };

            // Add some erratic movement based on phase
            let erratic_factor = boss.phase as f32 * 0.2;
            let erratic_movement = Vec2::new(
                (time.elapsed_secs() * 2.0 + boss.phase as f32).sin() * erratic_factor * 30.0,
                (time.elapsed_secs() * 1.5 + boss.phase as f32).cos() * erratic_factor * 20.0,
            );

            velocity.0 = movement_vector + erratic_movement;

            // Execute attack patterns based on boss's current phase
            if boss.attack_timer.just_finished() {
                let current_pattern = boss.get_current_attack_pattern();

                // Update attack pattern if it has changed
                if attack_pattern.pattern_type != current_pattern {
                    *attack_pattern = BossAttackPattern::new(current_pattern);
                }

                execute_boss_attack(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut attack_pattern,
                    boss_transform,
                    player_pos,
                    &boss.boss_type,
                    boss.phase,
                );
            }

            // Change attack pattern when current one finishes
            if attack_pattern.pattern_timer.just_finished() {
                let new_pattern = match boss.phase {
                    1 => AttackPattern::CircularShot,
                    2 => AttackPattern::TargetedBarrage,
                    3 => AttackPattern::SpawnMinions,
                    _ => AttackPattern::AsteroidRain,
                };
                *attack_pattern = BossAttackPattern::new(new_pattern);
            }
        }
    }
}

fn execute_boss_attack(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    attack_pattern: &mut BossAttackPattern,
    boss_transform: &Transform,
    player_pos: Vec2,
    _boss_type: &BossType,
    phase: u32,
) {
    let boss_pos = boss_transform.translation.truncate();

    match &attack_pattern.pattern_type {
        AttackPattern::CircularShot => {
            // Shoot bullets in all directions - more bullets in higher phases
            let bullet_count = 6 + (phase * 2);
            for i in 0..bullet_count {
                let angle = (i as f32 / bullet_count as f32) * 2.0 * std::f32::consts::PI;
                let direction = Vec2::new(angle.cos(), angle.sin());
                let bullet_velocity = direction * BULLET_SPEED * (0.6 + phase as f32 * 0.1);

                commands.spawn((
                    Mesh2d(meshes.add(create_enemy_bullet_mesh())),
                    MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(6.0, 3.0, 1.0)))), // Brighter boss bullet
                    Transform::from_translation(boss_pos.extend(0.0))
                        .with_rotation(Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2)),
                    EnemyBullet {
                        damage: 2 + phase / 2,
                        is_explosive: phase > 2,
                    },
                    Velocity(bullet_velocity),
                    BulletLifecycle::new(4.0, 5.0),
                    PulsingEffect::new(0.2, 0.4), // Stronger pulse for boss bullets
                ));
            }
        }
        AttackPattern::TargetedBarrage => {
            // Shoot multiple bullets at player with some spread - more bullets in higher phases
            let to_player = (player_pos - boss_pos).normalize();
            let bullet_count = 3 + phase;

            for i in 0..bullet_count {
                let spread_range = 0.4 + (phase as f32 * 0.1);
                let spread_angle = (i as f32 - (bullet_count as f32 - 1.0) / 2.0) * spread_range
                    / bullet_count as f32;
                let direction = Vec2::new(
                    to_player.x * spread_angle.cos() - to_player.y * spread_angle.sin(),
                    to_player.x * spread_angle.sin() + to_player.y * spread_angle.cos(),
                );
                let bullet_velocity = direction * BULLET_SPEED * (0.8 + phase as f32 * 0.1);

                commands.spawn((
                    Mesh2d(meshes.add(create_enemy_bullet_mesh())),
                    MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(6.0, 2.0, 3.0)))), // Brighter boss bullet
                    Transform::from_translation(boss_pos.extend(0.0)).with_rotation(
                        Quat::from_rotation_z(
                            direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2,
                        ),
                    ),
                    EnemyBullet {
                        damage: 3 + phase / 2,
                        is_explosive: phase > 1,
                    },
                    Velocity(bullet_velocity),
                    BulletLifecycle::new(4.0, 5.0),
                    PulsingEffect::new(0.2, 0.4), // Stronger pulse for boss bullets
                ));
            }
        }
        AttackPattern::SpawnMinions => {
            // Spawn enemy ships near the boss
            let enemy_type = EnemyType::Hunter;
            let spawn_offset = Vec2::new(
                (fastrand::f32() - 0.5) * 100.0,
                (fastrand::f32() - 0.5) * 100.0,
            );

            let enemy = Enemy::new(enemy_type.clone());
            let enemy_color = enemy.get_color();

            commands.spawn((
                Mesh2d(meshes.add(create_enemy_ship_mesh(&enemy_type))),
                MeshMaterial2d(materials.add(ColorMaterial::from(enemy_color))),
                Transform::from_translation((boss_pos + spawn_offset).extend(0.0)),
                enemy,
                Health::new(1), // Weaker minions
                Velocity(Vec2::ZERO),
                AIBehavior::new(),
                PulsingEffect::new(0.4, 0.2), // More visible pulse for boss-spawned enemies
                Wraparound,
            ));
        }
        AttackPattern::AsteroidRain => {
            // Spawn small asteroids around the boss
            let spawn_offset = Vec2::new(
                (fastrand::f32() - 0.5) * 200.0,
                (fastrand::f32() - 0.5) * 200.0,
            );

            let asteroid = Asteroid::new(2, AsteroidType::Normal);
            let asteroid_color = asteroid.get_color();

            commands.spawn((
                Mesh2d(meshes.add(create_asteroid_mesh(2, 10.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(asteroid_color))),
                Transform::from_translation((boss_pos + spawn_offset).extend(0.0)),
                asteroid,
                Health::new(1),
                Velocity((player_pos - boss_pos - spawn_offset).normalize() * 80.0),
                RotationVelocity::random_slow(),
                Wraparound,
            ));
        }
    }
}

// System to handle pulsing effects on entities
pub fn pulsing_effect_system(
    mut pulsing_query: Query<(&mut Transform, &mut PulsingEffect)>,
    time: Res<Time>,
) {
    for (mut transform, mut pulsing) in pulsing_query.iter_mut() {
        pulsing.timer.tick(time.delta());

        // Calculate pulsing scale based on sine wave
        let pulse_factor = (pulsing.timer.elapsed_secs() * 10.0).sin() * pulsing.pulse_amplitude;
        let scale = pulsing.base_scale + pulse_factor;

        transform.scale = Vec3::splat(scale);
    }
}
