use bevy::prelude::*;

/// A snapshot of one completed game session for the history table.
#[derive(Clone, Default)]
pub struct ScoreRecord {
    pub n: usize,
    pub total_rounds: usize,
    pub round_duration: f32,
    pub correct: usize,
    pub wrong: usize,
    pub f1_score_percent: usize,
    /// Timestamp string filled by the persistence layer on insert.
    pub played_at: String,
}

/// Accumulated score history across game sessions.
#[derive(Clone, Default, Resource)]
pub struct ScoreHistory(pub Vec<ScoreRecord>);
