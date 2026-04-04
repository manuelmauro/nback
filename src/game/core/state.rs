use bevy::prelude::*;

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}
