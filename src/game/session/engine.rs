use bevy::prelude::*;

use crate::game::tile::{
    color::TileColor, position::TilePosition, shape::TileShape, sound::TileSound,
};

use super::cue::CueChain;

/// Generated cues for one round.
pub struct Cue {
    pub position: Option<TilePosition>,
    pub color: Option<TileColor>,
    pub shape: Option<TileShape>,
    pub sound: Option<TileSound>,
}

/// The n-back game engine. Owns one [`CueChain`] per enabled stimulus channel.
#[derive(Component)]
pub struct CueEngine {
    n: usize,
    pub positions: Option<CueChain<TilePosition>>,
    pub colors: Option<CueChain<TileColor>>,
    pub shapes: Option<CueChain<TileShape>>,
    pub sounds: Option<CueChain<TileSound>>,
}

impl CueEngine {
    pub fn new(n: usize, position: bool, color: bool, shape: bool, sound: bool) -> Self {
        CueEngine {
            n,
            positions: position.then(|| CueChain::with_n_back(n)),
            colors: color.then(|| CueChain::with_n_back(n)),
            shapes: shape.then(|| CueChain::with_n_back(n)),
            sounds: sound.then(|| CueChain::with_n_back(n)),
        }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn new_cue(&mut self) -> Cue {
        Cue {
            position: self.positions.as_mut().map(|p| p.next_cue()),
            color: self.colors.as_mut().map(|c| c.next_cue()),
            shape: self.shapes.as_mut().map(|s| s.next_cue()),
            sound: self.sounds.as_mut().map(|s| s.next_cue()),
        }
    }
}

impl Default for CueEngine {
    fn default() -> Self {
        CueEngine::new(2, true, true, true, true)
    }
}
