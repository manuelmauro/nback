use bevy::prelude::*;

/// Tracks the player's response for the current round.
///
/// Lives as a component on the session entity so it is automatically
/// cleaned up when the game ends.
#[derive(Component, Debug, Default)]
pub struct Answer {
    pub position: bool,
    pub color: bool,
    pub sound: bool,
}

impl Answer {
    pub fn reset(&mut self) {
        info!("reset answer");
        self.position = false;
        self.color = false;
        self.sound = false;
    }
}
