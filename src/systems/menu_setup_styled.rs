use crate::assets::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::button_helpers::*;
use bevy::prelude::*;

pub fn setup_main_menu_styled(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let window_width = window.width();
        let window_height = window.height();

        commands.spawn((
            Text::new("ASTEROID BLASTER"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor::from(Color::srgb(3.0, 3.0, 3.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(window_height * 0.25),
                left: Val::Px(window_width * 0.5 - 180.0),
                ..default()
            },
            MainMenu,
            MenuText3D,
        ));

        // Create button meshes and materials
        let button_mesh = meshes.add(create_button_outline_mesh(200.0, 50.0));

        // Start Game button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "START GAME",
                MenuAction::DifficultySelect,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.0, 2.5, 2.0),
                24.0,
                Vec3::new(0.0, -20.0, 0.0),
            ),
            MainMenu,
        );

        // Exit button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "EXIT",
                MenuAction::Exit,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.5, 1.5, 1.5),
                24.0,
                Vec3::new(0.0, -90.0, 0.0),
            ),
            MainMenu,
        );
    }
}

pub fn setup_difficulty_menu_styled(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let window_width = window.width();
        let window_height = window.height();

        // Title
        commands.spawn((
            Text::new("SELECT DIFFICULTY"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor::from(Color::srgb(3.0, 3.0, 3.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(window_height * 0.2),
                left: Val::Px(window_width * 0.5 - 140.0),
                ..default()
            },
            DifficultyMenu,
            MenuText3D,
        ));

        // Create button mesh
        let button_mesh = meshes.add(create_button_outline_mesh(200.0, 50.0));

        // Difficulty buttons with colors
        let difficulties = [
            ("Easy", Color::srgb(1.5, 2.5, 1.5)),   // Green
            ("Normal", Color::srgb(1.5, 1.5, 2.5)), // Blue
            ("Hard", Color::srgb(2.5, 2.0, 1.5)),   // Orange
            ("Insane", Color::srgb(2.5, 1.5, 1.5)), // Red
        ];

        for (i, (difficulty, color)) in difficulties.iter().enumerate() {
            let y_pos = 100.0 - (i as f32 * 70.0);

            spawn_button_with_text(
                &mut commands,
                &mut meshes,
                &mut materials,
                window_width,
                window_height,
                button_mesh.clone(),
                ButtonWithText::new(
                    *difficulty,
                    MenuAction::SetDifficulty(difficulty.to_string()),
                    Vec2::new(200.0, 50.0),
                    *color,
                    24.0,
                    Vec3::new(0.0, y_pos, 0.0),
                ),
                DifficultyMenu,
            );
        }

        // Back button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "BACK",
                MenuAction::QuitToMenu,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.0, 2.0, 2.0),
                24.0,
                Vec3::new(0.0, -200.0, 0.0),
            ),
            DifficultyMenu,
        );
    }
}

pub fn setup_game_over_menu_styled(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    score: Res<GameScore>,
    difficulty: Res<DifficultySettings>,
) {
    if let Ok(window) = windows.single() {
        let window_width = window.width();
        let window_height = window.height();

        // Game Over title
        commands.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor::from(Color::srgb(3.0, 1.0, 1.0)), // Bright red glow
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(window_height * 0.15),
                left: Val::Px(window_width * 0.5 - 120.0),
                ..default()
            },
            GameOverMenu,
            MenuText3D,
        ));

        // Final score
        commands.spawn((
            Text::new(format!("Final Score: {}", score.score)),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor::from(Color::srgb(2.5, 2.5, 2.5)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(window_height * 0.25),
                left: Val::Px(window_width * 0.5 - 80.0),
                ..default()
            },
            GameOverMenu,
            MenuText3D,
        ));

        // Difficulty played
        commands.spawn((
            Text::new(format!("Difficulty: {}", difficulty.name)),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor::from(Color::srgb(2.0, 2.0, 2.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(window_height * 0.30),
                left: Val::Px(window_width * 0.5 - 60.0),
                ..default()
            },
            GameOverMenu,
            MenuText3D,
        ));

        // Create button mesh
        let button_mesh = meshes.add(create_button_outline_mesh(200.0, 50.0));

        // Play Again button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "PLAY AGAIN",
                MenuAction::PlayAgain,
                Vec2::new(200.0, 50.0),
                Color::srgb(1.5, 2.5, 1.5),
                24.0,
                Vec3::new(0.0, 50.0, 0.0),
            ),
            GameOverMenu,
        );

        // Main Menu button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "MAIN MENU",
                MenuAction::QuitToMenu,
                Vec2::new(200.0, 50.0),
                Color::srgb(1.5, 1.5, 2.5),
                24.0,
                Vec3::new(0.0, -20.0, 0.0),
            ),
            GameOverMenu,
        );

        // Exit button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "EXIT",
                MenuAction::Exit,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.5, 1.5, 1.5),
                24.0,
                Vec3::new(0.0, -90.0, 0.0),
            ),
            GameOverMenu,
        );
    }
}
