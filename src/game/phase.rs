use bevy::prelude::*;

use crate::state::AppState;

/// Sub-state of [`AppState::Game`] that controls whether the session is
/// actively running or paused.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates)]
#[source(AppState = AppState::Game)]
pub enum GamePhase {
    #[default]
    Playing,
    Paused,
}
