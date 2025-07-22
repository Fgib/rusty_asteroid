use crate::components::*;
use bevy::prelude::*;

pub fn spawn_button_with_text(
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window_width: f32,
    window_height: f32,
    button_mesh: Handle<Mesh>,
    button_data: ButtonWithText,
    menu_marker: impl Component + Clone,
) {
    let ButtonWithText {
        text,
        action,
        size,
        color,
        font_size,
        position,
    } = button_data;

    // Spawn button mesh
    let button_material = materials.add(ColorMaterial::from(color));
    commands.spawn((
        Mesh2d(button_mesh.clone()),
        MeshMaterial2d(button_material),
        Transform::from_translation(position),
        MenuButtonMesh::new(action, size, color),
        menu_marker.clone(),
    ));

    // Calculate text position based on button position and text length
    let text_offset_x = text.len() as f32 * 7.0; // Approximate character width
    let text_y = window_height * 0.5 - position.y - 12.0;
    let text_x = window_width * 0.5 - text_offset_x;

    // Spawn button text
    commands.spawn((
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor::from(color),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(text_y),
            left: Val::Px(text_x),
            ..default()
        },
        menu_marker.clone(),
        MenuText3D,
    ));
}
