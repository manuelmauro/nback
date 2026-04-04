use bevy::prelude::*;

use crate::game::tile::{color::TileColor, position::TilePosition, sound::TileSound};

use super::cue::CueChain;

/// The n-back game engine. Owns one [`CueChain`] per enabled stimulus channel.
#[derive(Component)]
pub struct CueEngine {
    n: usize,
    pub positions: Option<CueChain<TilePosition>>,
    pub colors: Option<CueChain<TileColor>>,
    pub sounds: Option<CueChain<TileSound>>,
}

impl CueEngine {
    pub fn new(n: usize, position: bool, color: bool, sound: bool) -> Self {
        CueEngine {
            n,
            positions: position.then(|| CueChain::with_n_back(n)),
            colors: color.then(|| CueChain::with_n_back(n)),
            sounds: sound.then(|| CueChain::with_n_back(n)),
        }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn new_cue(&mut self) -> (Option<TilePosition>, Option<TileColor>, Option<TileSound>) {
        let new_position = self.positions.as_mut().map(|p| p.next_cue());
        let new_color = self.colors.as_mut().map(|c| c.next_cue());
        let new_sound = self.sounds.as_mut().map(|s| s.next_cue());

        (new_position, new_color, new_sound)
    }
}

impl Default for CueEngine {
    fn default() -> Self {
        CueEngine::new(2, true, true, true)
    }
}
