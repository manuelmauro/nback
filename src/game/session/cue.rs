use std::collections::VecDeque;

use bevy::prelude::*;
use rand::{
    RngExt,
    distr::{Distribution, StandardUniform},
};

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

/// Memorization and generation of new cues for one stimulus channel.
///
/// `CueChain` is generic over the cue type — the n-back algorithm doesn't
/// know about positions, colors, or sounds.
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
    StandardUniform: Distribution<T>,
    T: Clone + PartialEq + Default,
{
    pub fn next_cue(&mut self) -> T {
        let mut rng = rand::rng();
        let y: f64 = rng.random_range(0.0..1.0);

        let cue = if y < 0.25 && *self.short_memory.front().unwrap() != default() {
            self.short_memory.front().unwrap().clone()
        } else {
            rng.random()
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
