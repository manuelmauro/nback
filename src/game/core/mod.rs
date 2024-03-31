use self::{answer::Answer, cue::CueChain, round::Round, score::Score, state::GameState};

use super::{
    tile::{color::TileColor, position::TilePosition},
    CueTimer,
};
use bevy::prelude::*;

pub mod answer;
pub mod cue;
pub mod round;
pub mod score;
pub mod state;

#[derive(Bundle)]
pub struct DualNBackBundle {
    pub dual_n_back: DualNBack,
    pub state: GameState,
    pub timer: CueTimer,
}

impl Default for DualNBackBundle {
    fn default() -> Self {
        DualNBackBundle {
            dual_n_back: DualNBack::new(),
            timer: CueTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            state: GameState::Playing,
        }
    }
}

#[derive(Component, Resource)]
pub struct DualNBack {
    pub n: usize,
    pub round: Round,
    pub score: Score,
    pub answer: Answer,
    pub positions: CueChain<TilePosition>,
    pub colors: CueChain<TileColor>,
}

impl DualNBack {
    pub fn new() -> Self {
        default()
    }

    pub fn n_back(&self) -> usize {
        self.positions.n_back()
    }

    pub fn is_over(&self) -> bool {
        self.round.current >= self.round.total
    }

    pub fn check_answer(&mut self) {
        if self.answer.same_position {
            if self.positions.is_match() {
                self.score.record_tp();
                info!("true_positive");
            } else {
                self.score.record_fp();
                info!("false_positive");
            }
        } else if self.positions.is_match() {
            self.score.record_fn();
            info!("false_neg");
        } else {
            self.score.record_tn();
            info!("true_neg");
        }

        if self.answer.same_color {
            if self.colors.is_match() {
                self.score.record_tp();
                info!("true_positive");
            } else {
                self.score.record_fp();
                info!("false_positive");
            }
        } else if self.colors.is_match() {
            self.score.record_fn();
            info!("false_neg");
        } else {
            self.score.record_tn();
            info!("true_neg");
        }
    }
}

impl Default for DualNBack {
    fn default() -> Self {
        DualNBack {
            n: 2,
            round: default(),
            score: default(),
            answer: default(),
            positions: CueChain::with_n_back(2),
            colors: CueChain::with_n_back(2),
        }
    }
}

/// Generate cues for the n-back task.
/// The iterator will run indefinitely.
impl Iterator for DualNBack {
    type Item = (TilePosition, TileColor);

    fn next(&mut self) -> Option<Self::Item> {
        self.round.current += 1;
        Some((self.positions.gen(), self.colors.gen()))
    }
}
