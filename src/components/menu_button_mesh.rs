use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MenuButtonMesh {
    pub action: MenuAction,
    pub bounds: Vec2,       // Width and height for collision detection
    pub base_color: Color,  // Base color when not hovered
    pub hover_color: Color, // Color when hovered
    pub is_hovered: bool,
}

impl MenuButtonMesh {
    pub fn new(action: MenuAction, bounds: Vec2, base_color: Color) -> Self {
        Self {
            action,
            bounds,
            base_color,
            hover_color: Color::srgb(
                base_color.to_srgba().red + 0.3,
                base_color.to_srgba().green + 0.3,
                base_color.to_srgba().blue + 0.3,
            ),
            is_hovered: false,
        }
    }
}

#[derive(Component)]
pub struct MenuText3D;

#[derive(Component)]
pub struct MenuBackground;
