use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerLives {
    pub lives: u32,
    #[allow(dead_code)]
    pub max_lives: u32,
}

impl PlayerLives {
    pub fn new(lives: u32) -> Self {
        Self {
            lives,
            max_lives: lives,
        }
    }

    pub fn lose_life(&mut self) -> bool {
        if self.lives > 0 {
            self.lives -= 1;
        }
        self.lives == 0 // Returns true if game over
    }

    #[allow(dead_code)]
    pub fn is_game_over(&self) -> bool {
        self.lives == 0
    }
}

impl Default for PlayerLives {
    fn default() -> Self {
        Self::new(3)
    }
}
