use bevy::prelude::*;

#[derive(Component)]
pub struct RotationVelocity {
    pub angular_velocity: f32, // radians per second
}

impl RotationVelocity {
    #[allow(dead_code)]
    pub fn new(angular_velocity: f32) -> Self {
        Self { angular_velocity }
    }

    pub fn random_slow() -> Self {
        // Random slow rotation between -1.0 and 1.0 radians per second
        let angular_velocity = (fastrand::f32() - 0.5) * 2.0;
        Self { angular_velocity }
    }
}
