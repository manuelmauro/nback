use self::{
    cue::{CueEngine, CueTimer},
    round::Round,
    score::Score,
    state::GameState,
};

use bevy::prelude::*;

pub mod cue;
pub mod round;
pub mod score;
pub mod state;

#[derive(Bundle, Default)]
pub struct DualNBackBundle {
    pub engine: CueEngine,
    pub state: GameState,
    pub timer: CueTimer,
    pub round: Round,
    pub score: Score,
}
