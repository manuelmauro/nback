use bevy::prelude::*;

#[derive(Component)]
pub struct Round {
    pub total: usize,
    pub current: usize,
    pub duration: f32,
}

impl Round {
    pub fn is_last(&self) -> bool {
        self.current >= self.total
    }
}
impl Default for Round {
    fn default() -> Self {
        Round {
            total: 10,
            current: 0,
            duration: 2.0,
        }
    }
}
