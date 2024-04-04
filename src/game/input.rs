use bevy::prelude::*;

use crate::state::AppState;

use super::core::round::Round;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_system.run_if(in_state(AppState::Game)));
    }
}

fn input_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Round>) {
    if let Ok(mut round) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            round.answer.position = true;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            round.answer.sound = true;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            round.answer.color = true;
        }
    }
}
