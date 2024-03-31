use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Score {
    false_pos: usize,
    true_pos: usize,
    false_neg: usize,
    true_neg: usize,
}

impl Score {
    pub fn record_fp(&mut self) {
        info!("false_positive");
        self.false_pos += 1;
    }

    pub fn record_tp(&mut self) {
        info!("true_positive");
        self.true_pos += 1;
    }

    pub fn record_fn(&mut self) {
        info!("false_neg");
        self.false_neg += 1;
    }

    pub fn record_tn(&mut self) {
        info!("true_neg");
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

    pub fn f1_score_percent(&self) -> usize {
        (self.f1_score() * 100.0) as usize
    }
}
