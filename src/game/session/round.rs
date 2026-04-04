use bevy::prelude::*;

#[derive(Component)]
pub struct Round {
    pub total: usize,
    pub current: usize,
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
        }
    }
}
