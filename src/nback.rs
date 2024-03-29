use crate::tile::{TileColor, TilePosition};
use bevy::prelude::{info, Resource};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::VecDeque;

#[derive(Default, Debug)]
pub struct Answer {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

impl Answer {
    pub fn w(&mut self) {
        self.w = true;
    }

    pub fn a(&mut self) {
        self.a = true;
    }

    pub fn s(&mut self) {
        self.s = true;
    }

    pub fn d(&mut self) {
        self.d = true;
    }

    pub fn reset(&mut self) {
        self.w = false;
        self.a = false;
        self.s = false;
        self.d = false;
    }
}

#[derive(Default)]
pub struct Score {
    false_pos: usize,
    true_pos: usize,
    false_neg: usize,
    true_neg: usize,
}

impl Score {
    pub fn record_fp(&mut self) {
        self.false_pos += 1;
    }

    pub fn record_tp(&mut self) {
        self.true_pos += 1;
    }

    pub fn record_fn(&mut self) {
        self.false_neg += 1;
    }

    pub fn record_tn(&mut self) {
        self.true_neg += 1;
    }

    pub fn correct(&self) -> usize {
        self.true_pos + self.true_neg
    }

    pub fn wrong(&self) -> usize {
        self.false_pos + self.false_neg
    }

    pub fn f1_score(&self) -> f32 {
        if self.true_pos + self.false_neg == 0 {
            1.0
        } else {
            self.true_pos as f32
                / (self.true_pos as f32 + 0.5 * (self.false_pos as f32 + self.false_neg as f32))
        }
    }
}

#[derive(Resource)]
pub struct NBack {
    pub rounds: usize,
    pub cur_round: usize,
    pub round_time: f32,
    pub score: Score,
    pub answer: Answer,
    pub positions: CueChain<TilePosition>,
    pub colors: CueChain<TileColor>,
}

impl NBack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn restart(&mut self) {
        self.score = Default::default();
        self.cur_round = 0;
        self.positions = CueChain::with_n_back(self.positions.n_back());
        self.colors = CueChain::with_n_back(self.colors.n_back());
    }

    pub fn n_back(&self) -> usize {
        self.positions.n_back()
    }

    pub fn is_over(&self) -> bool {
        self.cur_round >= self.rounds
    }

    pub fn check_answer(&mut self) {
        if self.answer.a {
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

        if self.answer.d {
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

impl Default for NBack {
    fn default() -> Self {
        NBack {
            rounds: 10,
            cur_round: 0,
            round_time: 1.0,
            score: Default::default(),
            answer: Default::default(),
            positions: CueChain::with_n_back(2),
            colors: CueChain::with_n_back(2),
        }
    }
}

impl Iterator for NBack {
    type Item = (TilePosition, TileColor);

    fn next(&mut self) -> Option<Self::Item> {
        self.cur_round += 1;
        Some((self.positions.gen(), self.colors.gen()))
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
            cc.short_memory.push_front(Default::default());
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

        let cue = if y < 0.25 && *self.short_memory.front().unwrap() != Default::default() {
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
        if self.short_memory.front() != Some(&Default::default()) {
            self.short_memory.back() == self.short_memory.front()
        } else {
            false
        }
    }
}
