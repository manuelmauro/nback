use bevy::prelude::*;

use crate::{
    game::{score::LatestGameScores, settings::GameSettings, tile::position},
    state::{despawn_screen, AppState, OnMenuScreen},
};

use self::{
    button::{decrease_n_button_system, increase_n_button_system, play_button_system},
    checkbox::{color_checkbox_system, position_checkbox_system, sound_checkbox_system},
    text::nback_text_system,
    ui::UiPlugin,
};

pub mod button;
pub mod checkbox;
pub mod text;
pub mod ui;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin)
            .add_systems(OnEnter(AppState::Menu), setup)
            .add_systems(OnExit(AppState::Menu), despawn_screen::<OnMenuScreen>)
            .add_systems(
                Update,
                (
                    nback_text_system,
                    increase_n_button_system,
                    decrease_n_button_system,
                    position_checkbox_system,
                    sound_checkbox_system,
                    color_checkbox_system,
                    play_button_system,
                )
                    .run_if(in_state(AppState::Menu)),
            )
            .insert_resource(GameSettings::default())
            .insert_resource(LatestGameScores::default());
    }
}

fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {}
