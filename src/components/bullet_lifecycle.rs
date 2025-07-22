use bevy::prelude::*;

#[derive(Component)]
pub struct BulletLifecycle {
    pub lifetime: Timer,
    pub initial_intensity: f32,
}

impl BulletLifecycle {
    pub fn new(lifetime_seconds: f32, initial_intensity: f32) -> Self {
        Self {
            lifetime: Timer::from_seconds(lifetime_seconds, TimerMode::Once),
            initial_intensity,
        }
    }

    pub fn get_current_intensity(&self) -> f32 {
        let progress = self.lifetime.elapsed_secs() / self.lifetime.duration().as_secs_f32();
        // Linear fade from initial intensity to almost invisible
        self.initial_intensity * (1.0 - progress * 0.95)
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime.finished()
    }
}
