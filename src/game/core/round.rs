use bevy::prelude::*;

#[derive(Component)]
pub struct Round {
    pub total: usize,
    pub current: usize,
    pub answer: Answer,
}

impl Round {
    pub fn with_total(total: usize) -> Self {
        Round { total, ..default() }
    }

    pub fn is_last(&self) -> bool {
        self.current >= self.total
    }
}
impl Default for Round {
    fn default() -> Self {
        Round {
            total: 10,
            current: 0,
            answer: default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Answer {
    pub same_position: bool,
    pub same_color: bool,
}

impl Answer {
    pub fn reset(&mut self) {
        info!("reset answer");
        self.same_position = false;
        self.same_color = false;
    }
}
