use bevy::prelude::*;

use crate::state::AppState;

use super::core::round::Answer;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_system.run_if(in_state(AppState::Game)));
    }
}

fn input_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut answer: ResMut<Answer>) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        answer.position = true;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        answer.sound = true;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        answer.color = true;
    }
}
