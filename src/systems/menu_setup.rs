use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

#[allow(dead_code)]
pub fn setup_main_menu(mut commands: Commands) {
    // Spawn main menu UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("ASTEROID BLASTER"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor::from(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                MenuText,
            ));

            // Start button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton {
                        action: MenuAction::DifficultySelect,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("START GAME"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor::from(Color::WHITE),
                    ));
                });

            // Exit button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    MenuButton {
                        action: MenuAction::Exit,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("EXIT"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor::from(Color::WHITE),
                    ));
                });
        });
}

#[allow(dead_code)]
pub fn setup_difficulty_menu(mut commands: Commands) {
    commands
        .spawn((
            #[allow(dead_code)]
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DifficultyMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SELECT DIFFICULTY"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor::from(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                MenuText,
            ));

            // Difficulty buttons
            let difficulties = ["Easy", "Normal", "Hard", "Insane"];
            let colors = [
                Color::srgb(0.2, 0.8, 0.2), // Green for Easy
                Color::srgb(0.2, 0.2, 0.8), // Blue for Normal
                Color::srgb(0.8, 0.6, 0.2), // Orange for Hard
                Color::srgb(0.8, 0.2, 0.2), // Red for Insane
            ];

            for (difficulty, color) in difficulties.iter().zip(colors.iter()) {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                        BackgroundColor(*color),
                        MenuButton {
                            action: MenuAction::SetDifficulty(difficulty.to_string()),
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(*difficulty),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor::from(Color::WHITE),
                        ));
                    });
            }

            // Back button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(30.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                    MenuButton {
                        action: MenuAction::QuitToMenu,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("BACK"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor::from(Color::WHITE),
                    ));
                });
        });
}

#[allow(dead_code)]
pub fn setup_game_over_menu(
    mut commands: Commands,
    score: Res<GameScore>,
    difficulty: Res<DifficultySettings>,
) {
    #[allow(dead_code)]
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            // Game Over title
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor::from(Color::srgb(1.0, 0.3, 0.3)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
                MenuText,
            ));

            // Final score
            parent.spawn((
                Text::new(format!("Final Score: {}", score.score)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor::from(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                MenuText,
            ));

            // Difficulty played
            parent.spawn((
                Text::new(format!("Difficulty: {}", difficulty.name)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor::from(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
                MenuText,
            ));

            // Play Again button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                    MenuButton {
                        action: MenuAction::PlayAgain,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PLAY AGAIN"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor::from(Color::WHITE),
                    ));
                });

            // Main Menu button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
                    MenuButton {
                        action: MenuAction::QuitToMenu,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("MAIN MENU"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor::from(Color::WHITE),
                    ));
                });

            // Exit button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    MenuButton {
                        action: MenuAction::Exit,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("EXIT"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor::from(Color::WHITE),
                    ));
                });
        });
}
