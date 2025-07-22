use bevy::prelude::*;

#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}
