use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: u32,
    #[allow(dead_code)]
    pub max: u32,
}

impl Health {
    pub fn new(hp: u32) -> Self {
        Self {
            current: hp,
            max: hp,
        }
    }

    pub fn take_damage(&mut self, damage: u32) -> bool {
        self.current = self.current.saturating_sub(damage);
        self.current == 0
    }

    #[allow(dead_code)]
    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}
