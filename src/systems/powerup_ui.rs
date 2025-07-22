use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct PowerUpUI;

// System to display active power-ups
pub fn update_powerup_display(
    mut commands: Commands,
    _player_powerups: Res<PlayerPowerUps>,
    powerup_effects: Query<&PowerUpEffect>,
    mut ui_query: Query<(Entity, &mut Text), With<PowerUpUI>>,
) {
    // Get active power-up info
    let mut powerup_text = String::new();
    
    for effect in powerup_effects.iter() {
        let remaining = effect.time_remaining();
        let power_name = match effect.power_type {
            PowerUpType::MultiShot => "Multi-Shot",
            PowerUpType::RapidFire => "Rapid Fire",
            PowerUpType::PiercingBullets => "Piercing",
            PowerUpType::ExplosiveBullets => "Explosive",
            PowerUpType::LaserBeam => "Laser",
        };
        powerup_text.push_str(&format!("{}: {:.1}s\n", power_name, remaining));
    }

    // Update or create UI
    if let Ok((_, mut text)) = ui_query.single_mut() {
        text.0 = if powerup_text.is_empty() {
            "No active power-ups".to_string()
        } else {
            format!("Active Power-ups:\n{}", powerup_text)
        };
    } else if powerup_text.is_empty() {
        // Spawn UI if it doesn't exist
        commands.spawn((
            Text::new("No active power-ups"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            PowerUpUI,
            GameUI,
        ));
    } else {
        commands.spawn((
            Text::new(format!("Active Power-ups:\n{}", powerup_text)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            PowerUpUI,
            GameUI,
        ));
    }
}
