use self::{
    answer::Answer,
    cue::{CueChain, CueTimer},
    round::Round,
    score::Score,
    state::GameState,
};

use super::tile::{color::TileColor, position::TilePosition};
use bevy::prelude::*;

pub mod answer;
pub mod cue;
pub mod round;
pub mod score;
pub mod state;

#[derive(Bundle, Default)]
pub struct DualNBackBundle {
    pub dual_n_back: DualNBack,
    pub state: GameState,
    pub timer: CueTimer,
    pub round: Round,
    pub score: Score,
}

#[derive(Component, Resource)]
pub struct DualNBack {
    pub n: usize,
    pub answer: Answer,
    pub positions: CueChain<TilePosition>,
    pub colors: CueChain<TileColor>,
}

impl DualNBack {
    pub fn new_cue(&mut self) -> (TilePosition, TileColor) {
        (self.positions.gen(), self.colors.gen())
    }

    pub fn n_back(&self) -> usize {
        self.positions.n_back()
    }
}

impl Default for DualNBack {
    fn default() -> Self {
        DualNBack {
            n: 2,
            answer: default(),
            positions: CueChain::with_n_back(2),
            colors: CueChain::with_n_back(2),
        }
    }
}
