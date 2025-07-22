use crate::assets::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::button_helpers::*;
use bevy::prelude::*;

// Pause menu UI setup
pub fn setup_pause_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let window_width = window.width();
        let window_height = window.height();

        // Semi-transparent background
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(800.0, 600.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.0, 0.0, 0.0, 0.7)))),
            Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            PauseUI,
        ));

        // Pause menu background
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(400.0, 300.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.0, 0.0, 0.0)))),
            Transform::from_translation(Vec3::new(0.0, 0.0, 11.0)),
            PauseUI,
        ));

        // Title
        commands.spawn((
            Text::new("PAUSED"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, 100.0, 12.0)),
            PauseUI,
        ));

        // Create button mesh for reuse
        let button_mesh = meshes.add(create_button_outline_mesh(200.0, 50.0));

        // Resume button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "RESUME",
                MenuAction::Resume,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.0, 2.5, 2.0),
                24.0,
                Vec3::new(0.0, 20.0, 12.0),
            ),
            PauseUI,
        );

        // Settings button
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                "SETTINGS",
                MenuAction::Settings,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.0, 2.0, 2.5),
                24.0,
                Vec3::new(0.0, -30.0, 12.0),
            ),
            PauseUI,
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
                MenuAction::MainMenu,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.5, 1.5, 1.5),
                24.0,
                Vec3::new(0.0, -80.0, 12.0),
            ),
            PauseUI,
        );
    }
}

// Pause input system
pub fn pause_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

// Cleanup pause menu
pub fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
