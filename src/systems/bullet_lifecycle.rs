use crate::components::*;
use bevy::prelude::*;

/// Updates bullet lifecycle timers and adjusts their glow intensity over time
pub fn update_bullet_lifecycle(
    mut commands: Commands,
    mut bullets: Query<
        (Entity, &mut BulletLifecycle, &MeshMaterial2d<ColorMaterial>),
        With<Bullet>,
    >,
    mut explosions: Query<(Entity, &mut BulletLifecycle), (With<ExplosionVisual>, Without<Bullet>)>,
    mut lasers: Query<
        (Entity, &mut BulletLifecycle),
        (With<LaserBeam>, Without<Bullet>, Without<ExplosionVisual>),
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    // Handle bullets with glow effects
    for (bullet_entity, mut lifecycle, material_handle) in bullets.iter_mut() {
        // Update the timer
        lifecycle.lifetime.tick(time.delta());

        // Check if bullet should be despawned
        if lifecycle.is_expired() {
            commands.entity(bullet_entity).despawn();
            continue;
        }

        // Update the glow intensity
        let current_intensity = lifecycle.get_current_intensity();

        // Update the material color based on current intensity
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.color = Color::srgb(
                current_intensity,
                current_intensity,
                current_intensity * 0.25,
            );
        }
    }

    // Handle explosion visuals (no glow effects, just cleanup)
    for (explosion_entity, mut lifecycle) in explosions.iter_mut() {
        lifecycle.lifetime.tick(time.delta());
        if lifecycle.is_expired() {
            commands.entity(explosion_entity).despawn();
        }
    }

    // Handle laser beams (no glow effects, just cleanup)
    for (laser_entity, mut lifecycle) in lasers.iter_mut() {
        lifecycle.lifetime.tick(time.delta());
        if lifecycle.is_expired() {
            commands.entity(laser_entity).despawn();
        }
    }
}
