use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

#[allow(dead_code)]
pub fn menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut difficulty: ResMut<DifficultySettings>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match &menu_button.action {
                MenuAction::StartGame => {
                    next_state.set(GameState::Playing);
                }
                MenuAction::DifficultySelect => {
                    next_state.set(GameState::DifficultySelect);
                }
                MenuAction::SetDifficulty(difficulty_name) => {
                    *difficulty = match difficulty_name.as_str() {
                        "Easy" => DifficultySettings::easy(),
                        "Normal" => DifficultySettings::normal(),
                        "Hard" => DifficultySettings::hard(),
                        "Insane" => DifficultySettings::insane(),
                        _ => DifficultySettings::normal(),
                    };
                    next_state.set(GameState::Playing);
                }
                MenuAction::PlayAgain => {
                    next_state.set(GameState::Playing);
                }
                MenuAction::QuitToMenu => {
                    next_state.set(GameState::MainMenu);
                }
                MenuAction::Exit => {
                    exit.write(AppExit::Success);
                }
            },
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.9, 0.9, 0.9));
            }
            Interaction::None => {
                // Restore original color based on button type
                *color = match &menu_button.action {
                    MenuAction::SetDifficulty(diff) => BackgroundColor(match diff.as_str() {
                        "Easy" => Color::srgb(0.2, 0.8, 0.2),
                        "Normal" => Color::srgb(0.2, 0.2, 0.8),
                        "Hard" => Color::srgb(0.8, 0.6, 0.2),
                        "Insane" => Color::srgb(0.8, 0.2, 0.2),
                        _ => Color::srgb(0.2, 0.2, 0.2),
                    }),
                    MenuAction::PlayAgain => BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                    MenuAction::QuitToMenu => BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
                    MenuAction::Exit => BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    _ => BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                };
            }
        }
    }
}

#[allow(dead_code)]
pub fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, Or<(With<MainMenu>, With<DifficultyMenu>, With<GameOverMenu>)>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[allow(dead_code)]
pub fn cleanup_game_ui(mut commands: Commands, ui_query: Query<Entity, With<GameUI>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_all_entities(
    mut commands: Commands,
    entity_query: Query<
        Entity,
        Or<(
            With<Player>,
            With<Asteroid>,
            With<Bullet>,
            With<ScoreText>,
            With<LivesText>,
            With<HeartUI>,
        )>,
    >,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn reset_game_resources(
    mut score: ResMut<GameScore>,
    mut lives: ResMut<PlayerLives>,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
    mut fire_timer: ResMut<FireTimer>,
    difficulty: Res<DifficultySettings>,
) {
    // Reset score and lives
    score.score = 0;
    *lives = PlayerLives::default();

    // Reset timers with difficulty settings
    spawn_timer.timer = Timer::from_seconds(difficulty.asteroid_spawn_rate, TimerMode::Repeating);
    fire_timer.timer = Timer::from_seconds(0.05, TimerMode::Repeating);
}
