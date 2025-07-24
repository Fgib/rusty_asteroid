use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn update_score_display(score: Res<GameScore>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("Score: {}", score.score);
        }
    }
}

pub fn update_lives_display(lives: Res<PlayerLives>, mut query: Query<&mut Text, With<LivesText>>) {
    if lives.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("Lives: {}", lives.lives);
        }
    }
}

pub fn update_heart_display(
    lives: Res<PlayerLives>,
    mut heart_query: Query<(&mut Visibility, &HeartUI)>,
) {
    if lives.is_changed() {
        for (mut visibility, heart_ui) in heart_query.iter_mut() {
            // Show heart if the heart index is less than current lives
            *visibility = if heart_ui.heart_index < lives.lives as usize {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

pub fn spawn_boss_health_bar(
    mut commands: Commands,
    boss_query: Query<&Boss, Added<Boss>>,
    health_bar_query: Query<Entity, With<BossHealthBar>>,
    windows: Query<&Window>,
) {
    // Only spawn health bar if a boss was just added and no health bar exists
    if !boss_query.is_empty() && health_bar_query.is_empty() {
        if let Ok(window) = windows.single() {
            let window_width = window.width();
            let bar_width = 400.0;
            let bar_height = 20.0;
            let bar_x = (window_width - bar_width) / 2.0;
            let bar_y = window.height() - 60.0; // Near top of screen

            // Spawn the health bar background
            let health_bar_bg = commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(bar_x),
                        top: Val::Px(bar_y),
                        width: Val::Px(bar_width),
                        height: Val::Px(bar_height),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                    BorderColor(Color::srgb(1.0, 1.0, 1.0)),
                    BossHealthBar,
                    GameUI,
                ))
                .id();

            // Spawn the health bar fill
            commands
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(2.0, 0.3, 0.3)), // Bright red for boss health
                    BossHealthBarFill,
                    GameUI,
                ))
                .insert(ChildOf(health_bar_bg));

            // Spawn boss phase text
            commands.spawn((
                Text::new("Boss - Phase 1"),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(bar_x),
                    top: Val::Px(bar_y - 25.0),
                    ..default()
                },
                TextColor(Color::srgb(2.0, 2.0, 2.0)), // Bright white
                BossPhaseText,
                GameUI,
            ));
        }
    }
}

pub fn update_boss_health_bar(
    boss_query: Query<(&Boss, &Health)>,
    mut health_bar_fill_query: Query<&mut Node, With<BossHealthBarFill>>,
    mut phase_text_query: Query<&mut Text, With<BossPhaseText>>,
) {
    // Handle case where there might be multiple bosses or no bosses
    for (boss, health) in boss_query.iter() {
        // Update health bar fill width
        let health_percentage = (health.current as f32 / health.max as f32).clamp(0.0, 1.0);

        if let Ok(mut fill_node) = health_bar_fill_query.single_mut() {
            fill_node.width = Val::Percent(health_percentage * 100.0);
        }

        // Update phase text
        if let Ok(mut text) = phase_text_query.single_mut() {
            let boss_name = match boss.boss_type {
                crate::components::BossType::GiantAsteroid => "Giant Asteroid",
                crate::components::BossType::AlienMothership => "Alien Mothership",
            };
            **text = format!("{} - Phase {} / {}", boss_name, boss.phase, boss.max_phases);
        }

        // Only handle the first boss if there are multiple
        break;
    }
}

pub fn despawn_boss_health_bar(
    mut commands: Commands,
    boss_query: Query<&Boss>,
    health_bar_query: Query<Entity, With<BossHealthBar>>,
    phase_text_query: Query<Entity, With<BossPhaseText>>,
) {
    // If no bosses exist, remove the health bar
    if boss_query.is_empty() {
        for entity in health_bar_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in phase_text_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
