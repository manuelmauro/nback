use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct GameScore {
    pub n: usize,
    pub correct: usize,
    pub wrong: usize,
    pub f1_score: f32,
}
