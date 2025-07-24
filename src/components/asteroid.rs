use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq)]
pub enum AsteroidType {
    Normal,
    Ice,    // Shatters into more pieces
    Metal,  // Requires more hits, gives more points
    Crystal, // Rare, gives power-ups when destroyed
}

#[derive(Component)]
pub struct Asteroid {
    pub size: u32,
    pub asteroid_type: AsteroidType,
}

impl Asteroid {
    pub fn new(size: u32, asteroid_type: AsteroidType) -> Self {
        Self { size, asteroid_type }
    }

    pub fn points_value(&self) -> u32 {
        let base_points = (11 - self.size) * 10;
        match self.asteroid_type {
            AsteroidType::Normal => base_points,
            AsteroidType::Ice => base_points,
            AsteroidType::Metal => base_points * 2, // Double points for metal
            AsteroidType::Crystal => base_points * 3, // Triple points for crystal
        }
    }

    pub fn max_health(&self) -> u32 {
        match self.asteroid_type {
            AsteroidType::Normal => self.size,
            AsteroidType::Ice => self.size,
            AsteroidType::Metal => self.size * 3, // Triple health for metal
            AsteroidType::Crystal => self.size * 2, // Double health for crystal
        }
    }

    pub fn fragment_count(&self) -> u32 {
        match self.asteroid_type {
            AsteroidType::Normal => if self.size >= 5 { 3 } else { 2 },
            AsteroidType::Ice => {
                // Ice asteroids shatter into many more pieces
                if self.size >= 7 { 6 } else if self.size >= 4 { 5 } else { 4 }
            },
            AsteroidType::Metal => {
                // Metal asteroids are harder to break, fewer fragments
                if self.size >= 6 { 2 } else { 1 }
            },
            AsteroidType::Crystal => {
                // Crystal asteroids have normal fragmentation but higher chance of spawning power-ups
                if self.size >= 5 { 3 } else { 2 }
            }
        }
    }

    /// Returns the special behavior modifier for this asteroid type
    pub fn get_behavior_modifier(&self) -> f32 {
        match self.asteroid_type {
            AsteroidType::Normal => 1.0,
            AsteroidType::Ice => 0.8,    // Slightly slower
            AsteroidType::Metal => 0.6,  // Much slower but harder to destroy
            AsteroidType::Crystal => 1.2, // Slightly faster and more erratic
        }
    }

    /// Returns whether this asteroid type has special destruction effects
    pub fn has_special_destruction(&self) -> bool {
        matches!(self.asteroid_type, AsteroidType::Ice | AsteroidType::Crystal)
    }

    pub fn get_color(&self) -> Color {
        let glow_intensity = (self.size as f32 / 10.0) * 2.0 + 1.5;
        match self.asteroid_type {
            AsteroidType::Normal => Color::srgb(
                glow_intensity * 0.8,
                glow_intensity * 1.2,
                glow_intensity * 0.6,
            ),
            AsteroidType::Ice => Color::srgb(
                glow_intensity * 0.3,
                glow_intensity * 0.8,
                glow_intensity * 1.5, // Blue tint for ice
            ),
            AsteroidType::Metal => Color::srgb(
                glow_intensity * 1.2,
                glow_intensity * 1.2,
                glow_intensity * 1.2, // Silver/gray for metal
            ),
            AsteroidType::Crystal => Color::srgb(
                glow_intensity * 1.5,
                glow_intensity * 0.3,
                glow_intensity * 1.5, // Purple for crystal
            ),
        }
    }
}
