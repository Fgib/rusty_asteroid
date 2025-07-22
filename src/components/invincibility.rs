use bevy::prelude::*;

#[derive(Component)]
pub struct Invincibility {
    pub timer: Timer,
}

impl Invincibility {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }

    pub fn is_active(&self) -> bool {
        !self.timer.finished()
    }
}

impl Default for Invincibility {
    fn default() -> Self {
        Self::new(2.0) // 2 seconds of invincibility
    }
}
