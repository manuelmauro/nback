use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    pub n: usize,
    pub rounds: usize,
    pub round_time: f32,
    pub position: bool,
    pub color: bool,
    pub sound: bool,
}

impl GameSettings {
    pub fn set_rounds_from_n(&mut self) {
        self.rounds = 20 + self.n.pow(2);
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            n: 2,
            rounds: 24,
            round_time: 3.0,
            position: true,
            color: true,
            sound: true,
        }
    }
}
