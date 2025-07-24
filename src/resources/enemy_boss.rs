use bevy::prelude::*;
use crate::components::*;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
    pub last_spawn_score: u32,
    pub spawn_score_interval: u32,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(10.0, TimerMode::Repeating), // Every 10 seconds (faster)
            last_spawn_score: 0,
            spawn_score_interval: 300, // Spawn enemy every 300 points (more frequent)
        }
    }
}

#[derive(Resource)]
pub struct BossSpawnManager {
    pub bosses_spawned: Vec<BossType>,
    pub active_boss: Option<Entity>,
}

impl Default for BossSpawnManager {
    fn default() -> Self {
        Self {
            bosses_spawned: Vec::new(),
            active_boss: None,
        }
    }
}

impl BossSpawnManager {
    pub fn should_spawn_boss(&self, score: u32) -> Option<BossType> {
        // Giant Asteroid boss at 2000 points
        if score >= 2000 && !self.bosses_spawned.contains(&BossType::GiantAsteroid) && self.active_boss.is_none() {
            return Some(BossType::GiantAsteroid);
        }
        
        // Alien Mothership boss at 5000 points
        if score >= 5000 && !self.bosses_spawned.contains(&BossType::AlienMothership) && self.active_boss.is_none() {
            return Some(BossType::AlienMothership);
        }
        
        None
    }

    pub fn mark_boss_spawned(&mut self, boss_type: BossType, entity: Entity) {
        self.bosses_spawned.push(boss_type);
        self.active_boss = Some(entity);
    }

    pub fn clear_active_boss(&mut self) {
        self.active_boss = None;
    }
}

#[derive(Resource)]
pub struct AsteroidTypeGenerator {
    pub weights: Vec<(AsteroidType, f32)>,
}

impl Default for AsteroidTypeGenerator {
    fn default() -> Self {
        Self {
            weights: vec![
                (AsteroidType::Normal, 0.6),   // 60% chance
                (AsteroidType::Ice, 0.25),     // 25% chance
                (AsteroidType::Metal, 0.12),   // 12% chance
                (AsteroidType::Crystal, 0.03), // 3% chance
            ],
        }
    }
}

impl AsteroidTypeGenerator {
    pub fn generate(&self) -> AsteroidType {
        let roll = fastrand::f32();
        let mut cumulative = 0.0;
        
        for (asteroid_type, weight) in &self.weights {
            cumulative += weight;
            if roll < cumulative {
                return asteroid_type.clone();
            }
        }
        
        // Fallback to normal
        AsteroidType::Normal
    }
}
