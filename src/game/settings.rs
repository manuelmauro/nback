use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    pub n: usize,
    pub rounds: usize,
    pub round_time: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            n: 2,
            rounds: 24,
            round_time: 3.0,
        }
    }
}
