use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn player_asteroid_collision_system(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, Option<&mut Invincibility>),
        (With<Player>, Without<Asteroid>),
    >,
    asteroids: Query<(Entity, &Transform, &Asteroid), (With<Asteroid>, Without<Player>)>,
    mut player_lives: ResMut<PlayerLives>,
    score: Res<GameScore>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    if let Ok((player_entity, player_transform, mut invincibility_opt)) = player_query.single_mut()
    {
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
            for (asteroid_entity, asteroid_transform, asteroid) in asteroids.iter() {
                // Collision threshold based on asteroid size + player size
                let collision_radius = asteroid.size as f32 * 5.0 + 15.0; // asteroid radius + player buffer

                let distance = player_transform
                    .translation
                    .distance(asteroid_transform.translation);

                if distance < collision_radius {
                    // Player hit by asteroid
                    let game_over = player_lives.lose_life();

                    if game_over {
                        // Game over - despawn player and transition to game over state
                        commands.entity(player_entity).despawn();
                        next_state.set(GameState::GameOver);
                        println!("Game Over! Final Score: {}", score.score);
                    } else {
                        // Add invincibility frames
                        commands
                            .entity(player_entity)
                            .insert(Invincibility::default());
                        println!("Player hit! Lives remaining: {}", player_lives.lives);
                    }

                    // Remove the asteroid that hit the player
                    commands.entity(asteroid_entity).despawn();

                    break; // Only handle one collision per frame
                }
            }
        }
    }
}

pub fn invincibility_visual_system(
    mut player_query: Query<(&mut MeshMaterial2d<ColorMaterial>, &Invincibility), With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    if let Ok((material_handle, invincibility)) = player_query.single_mut() {
        if invincibility.is_active() {
            // Make player flicker during invincibility
            let flicker_rate = 10.0; // flickers per second
            let alpha = (time.elapsed_secs() * flicker_rate).sin().abs();

            if let Some(material) = materials.get_mut(material_handle.id()) {
                material.color.set_alpha(alpha * 0.5 + 0.5); // Alpha between 0.5 and 1.0
            }
        } else {
            // Restore full opacity
            if let Some(material) = materials.get_mut(material_handle.id()) {
                material.color.set_alpha(1.0);
            }
        }
    }
}
