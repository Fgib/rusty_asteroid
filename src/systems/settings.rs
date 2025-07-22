use crate::assets::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::button_helpers::*;
use bevy::prelude::*;

// Settings menu UI setup
pub fn setup_settings_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<GameSettings>,
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
            MenuUI,
        ));

        // Title
        commands.spawn((
            Text::new("SETTINGS"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor::from(Color::srgb(3.0, 3.0, 3.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(window_height * 0.2),
                left: Val::Px(window_width * 0.5 - 90.0),
                ..default()
            },
            MenuUI,
            MenuText3D,
        ));

        // Graphics section title
        commands.spawn((
            Text::new("GRAPHICS"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor::from(Color::srgb(2.0, 2.0, 1.0)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(window_height * 0.35),
                left: Val::Px(window_width * 0.5 - 80.0),
                ..default()
            },
            MenuUI,
            MenuText3D,
        ));

        // Create button mesh for reuse
        let button_mesh = meshes.add(create_button_outline_mesh(200.0, 50.0));

        // Bloom toggle button
        let bloom_text = if settings.graphics.bloom_enabled {
            "Bloom: ON"
        } else {
            "Bloom: OFF"
        };
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                bloom_text,
                MenuAction::BloomToggle,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.0, 2.5, 2.0),
                20.0,
                Vec3::new(0.0, -20.0, 11.0),
            ),
            MenuUI,
        );

        // VSync toggle button
        let vsync_text = if settings.graphics.vsync_enabled {
            "VSync: ON"
        } else {
            "VSync: OFF"
        };
        spawn_button_with_text(
            &mut commands,
            &mut meshes,
            &mut materials,
            window_width,
            window_height,
            button_mesh.clone(),
            ButtonWithText::new(
                vsync_text,
                MenuAction::VsyncToggle,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.0, 2.0, 2.5),
                20.0,
                Vec3::new(0.0, -80.0, 11.0),
            ),
            MenuUI,
        );

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
                MenuAction::Back,
                Vec2::new(200.0, 50.0),
                Color::srgb(2.5, 1.5, 1.5),
                24.0,
                Vec3::new(0.0, -150.0, 11.0),
            ),
            MenuUI,
        );
    }
}

// Cleanup settings menu
pub fn cleanup_settings_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
