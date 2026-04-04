use bevy::prelude::*;

/// A snapshot of one completed game session for the history table.
#[derive(Default)]
pub struct ScoreRecord {
    pub n: usize,
    pub total_rounds: usize,
    pub round_duration: f32,
    pub correct: usize,
    pub wrong: usize,
    pub f1_score_percent: usize,
    /// ISO-8601 timestamp (filled by the database on insert).
    pub played_at: String,
}

/// Accumulated score history across game sessions.
#[derive(Default, Resource)]
pub struct ScoreHistory(pub Vec<ScoreRecord>);
