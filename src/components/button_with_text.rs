use crate::components::MenuAction;
use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonWithText {
    pub text: String,
    pub action: MenuAction,
    pub size: Vec2,
    pub color: Color,
    pub font_size: f32,
    pub position: Vec3,
}

impl ButtonWithText {
    pub fn new(
        text: impl Into<String>,
        action: MenuAction,
        size: Vec2,
        color: Color,
        font_size: f32,
        position: Vec3,
    ) -> Self {
        Self {
            text: text.into(),
            action,
            size,
            color,
            font_size,
            position,
        }
    }
}
