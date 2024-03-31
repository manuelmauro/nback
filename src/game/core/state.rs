use bevy::prelude::*;

#[derive(Component, Eq, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
}
