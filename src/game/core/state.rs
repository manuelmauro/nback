use bevy::prelude::*;

#[derive(Component, Default, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}
