use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn mesh_menu_button_system(
    mut button_query: Query<(
        &Transform,
        &mut MenuButtonMesh,
        &mut MeshMaterial2d<ColorMaterial>,
    )>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut difficulty: ResMut<DifficultySettings>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok(window) = windows.single() {
        if let Ok((camera, camera_transform)) = camera_query.single() {
            // Get cursor position
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert screen coordinates to world coordinates
                let window_size = Vec2::new(window.width(), window.height());
                let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
                let ndc = Vec3::new(ndc.x, -ndc.y, 0.0);

                if let Some(world_pos) = camera.ndc_to_world(camera_transform, ndc) {
                    let world_pos_2d = world_pos.truncate();

                    for (transform, mut button, material_handle) in button_query.iter_mut() {
                        let button_pos = transform.translation.truncate();
                        let half_bounds = button.bounds * 0.5;

                        // Check if cursor is within button bounds
                        let is_hovering = world_pos_2d.x >= button_pos.x - half_bounds.x
                            && world_pos_2d.x <= button_pos.x + half_bounds.x
                            && world_pos_2d.y >= button_pos.y - half_bounds.y
                            && world_pos_2d.y <= button_pos.y + half_bounds.y;

                        // Update hover state and material color
                        if is_hovering != button.is_hovered {
                            button.is_hovered = is_hovering;

                            let color = if is_hovering {
                                button.hover_color
                            } else {
                                button.base_color
                            };

                            if let Some(material) = materials.get_mut(material_handle.id()) {
                                material.color = color;
                            }
                        }

                        // Handle clicks
                        if is_hovering && mouse_input.just_pressed(MouseButton::Left) {
                            match &button.action {
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
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn cleanup_styled_menu(
    mut commands: Commands,
    menu_query: Query<
        Entity,
        Or<(
            With<MainMenu>,
            With<DifficultyMenu>,
            With<GameOverMenu>,
            With<MenuText3D>,
            With<MenuButtonMesh>,
        )>,
    >,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
