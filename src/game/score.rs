use bevy::prelude::*;

#[derive(Default)]
pub struct GameScore {
    pub n: usize,
    pub total_rounds: usize,
    pub round_duration: f32,
    pub correct: usize,
    pub wrong: usize,
    pub f1_score: f32,
}

#[derive(Default, Resource)]
pub struct LatestGameScores(pub Vec<GameScore>);
