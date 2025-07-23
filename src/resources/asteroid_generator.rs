use bevy::prelude::*;

#[derive(Resource)]
pub struct AsteroidSizeGenerator {
    min_size: u32,
    weights: Vec<f32>,
}

impl AsteroidSizeGenerator {
    pub fn new(min_size: u32, max_size: u32, rarity_factor: f32, base_probability: f32) -> Self {
        let range = max_size - min_size + 1;
        let mut weights = Vec::with_capacity(range as usize);
        let mut total_weight = 0.0;

        for i in 0..range {
            let weight = base_probability * (rarity_factor.powi(-(i as i32)));
            weights.push(weight);
            total_weight += weight;
        }

        let mut cumulative = 0.0;
        for weight in &mut weights {
            cumulative += *weight / total_weight;
            *weight = cumulative;
        }

        Self { min_size, weights }
    }

    pub fn generate(&self) -> u32 {
        let roll = fastrand::f32();

        for (i, &cumulative_prob) in self.weights.iter().enumerate() {
            if roll < cumulative_prob {
                return self.min_size + i as u32;
            }
        }

        self.min_size + self.weights.len() as u32 - 1
    }
}

impl Default for AsteroidSizeGenerator {
    fn default() -> Self {
        Self::new(1, 10, 2.1, 0.35)
    }
}
