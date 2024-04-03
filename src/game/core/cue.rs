use std::collections::VecDeque;

use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::game::tile::{color::TileColor, position::TilePosition, sound::TileSound};

#[derive(Component, Deref, DerefMut)]
pub struct CueTimer(pub Timer);

impl CueTimer {
    pub fn with_duration(duration: f32) -> Self {
        CueTimer(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}

impl Default for CueTimer {
    fn default() -> Self {
        CueTimer(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

/// Memorization and generation of new cues.
pub struct CueChain<T> {
    short_memory: VecDeque<T>,
}

impl<T: Default> Default for CueChain<T> {
    fn default() -> Self {
        CueChain::with_n_back(2)
    }
}

impl<T: Default> CueChain<T> {
    pub fn with_n_back(n: usize) -> Self {
        let mut cc = CueChain {
            short_memory: VecDeque::new(),
        };

        for _ in 0..n + 1 {
            cc.short_memory.push_front(default());
        }

        cc
    }

    pub fn n_back(&self) -> usize {
        self.short_memory.len() - 1
    }
}

impl<T> CueChain<T>
where
    Standard: Distribution<T>,
    T: Clone + PartialEq + Default,
{
    pub fn gen(&mut self) -> T {
        let mut rng = rand::thread_rng();
        let y = rng.gen::<f64>();

        let cue = if y < 0.25 && *self.short_memory.front().unwrap() != default() {
            self.short_memory.front().unwrap().clone()
        } else {
            rand::random()
        };

        self.short_memory.push_back(cue);
        self.short_memory.pop_front();

        (*self.short_memory.back().unwrap()).clone()
    }
}

impl<T: PartialEq + Default> CueChain<T> {
    pub fn is_match(&self) -> bool {
        if self.short_memory.front() != Some(&default()) {
            self.short_memory.back() == self.short_memory.front()
        } else {
            false
        }
    }
}

#[derive(Component, Resource)]
pub struct CueEngine {
    n: usize,
    pub positions: Option<CueChain<TilePosition>>,
    pub colors: Option<CueChain<TileColor>>,
    pub sounds: Option<CueChain<TileSound>>,
}

impl CueEngine {
    pub fn with(n: usize, position: bool, color: bool, sound: bool) -> Self {
        let positions = if position {
            Some(CueChain::with_n_back(n))
        } else {
            None
        };

        let colors = if color {
            Some(CueChain::with_n_back(n))
        } else {
            None
        };

        let sounds = if sound {
            Some(CueChain::with_n_back(n))
        } else {
            None
        };

        CueEngine {
            n,
            positions,
            colors,
            sounds,
        }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn new_cue(&mut self) -> (Option<TilePosition>, Option<TileColor>, Option<TileSound>) {
        let new_position = if self.positions.is_some() {
            Some(self.positions.as_mut().unwrap().gen())
        } else {
            None
        };

        let new_color = if self.colors.is_some() {
            Some(self.colors.as_mut().unwrap().gen())
        } else {
            None
        };

        let new_sound = if self.sounds.is_some() {
            Some(self.sounds.as_mut().unwrap().gen())
        } else {
            None
        };

        (new_position, new_color, new_sound)
    }
}

impl Default for CueEngine {
    fn default() -> Self {
        let n = 2;

        CueEngine {
            n,
            positions: Some(CueChain::with_n_back(n)),
            colors: Some(CueChain::with_n_back(n)),
            sounds: Some(CueChain::with_n_back(n)),
        }
    }
}
