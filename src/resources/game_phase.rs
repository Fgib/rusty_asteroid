use bevy::prelude::*;

#[derive(Resource, Debug, Clone, PartialEq)]
pub enum GamePhase {
    Normal,        // Regular gameplay with asteroids and regular enemies
    BossEncounter, // Boss is present - reduced asteroid spawns, heightened tension
}

impl Default for GamePhase {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Resource, Debug)]
pub struct GamePhaseManager {
    pub current_phase: GamePhase,
    pub phase_transition_timer: Option<Timer>,
    pub asteroid_spawn_multiplier: f32, // Multiplier for asteroid spawns during boss encounters
}

impl Default for GamePhaseManager {
    fn default() -> Self {
        Self {
            current_phase: GamePhase::Normal,
            phase_transition_timer: None,
            asteroid_spawn_multiplier: 1.0,
        }
    }
}

impl GamePhaseManager {
    /// Enter boss encounter phase with reduced asteroid spawns
    pub fn enter_boss_encounter(&mut self) {
        if self.current_phase != GamePhase::BossEncounter {
            self.current_phase = GamePhase::BossEncounter;
            self.asteroid_spawn_multiplier = 0.3; // Reduce asteroid spawns to 30% during boss fights
            self.phase_transition_timer = Some(Timer::from_seconds(1.0, TimerMode::Once));
        }
    }

    /// Return to normal phase when no bosses are present
    pub fn enter_normal_phase(&mut self) {
        if self.current_phase != GamePhase::Normal {
            self.current_phase = GamePhase::Normal;
            self.asteroid_spawn_multiplier = 1.0; // Return to normal spawn rates
            self.phase_transition_timer = Some(Timer::from_seconds(0.5, TimerMode::Once));
        }
    }

    /// Check if we're currently transitioning between phases
    pub fn is_transitioning(&self) -> bool {
        self.phase_transition_timer
            .as_ref()
            .map_or(false, |timer| !timer.finished())
    }

    /// Update the phase transition timer
    pub fn update(&mut self, delta: std::time::Duration) {
        if let Some(ref mut timer) = self.phase_transition_timer {
            timer.tick(delta);
            if timer.finished() {
                self.phase_transition_timer = None;
            }
        }
    }
}
