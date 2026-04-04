use bevy::prelude::*;

use crate::state::AppState;

use self::{
    button::menu_button_system, checkbox::checkbox_system, text::nback_text_system, ui::UiPlugin,
};

pub mod button;
pub mod checkbox;
pub mod text;
pub mod ui;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin).add_systems(
            Update,
            (
                nback_text_system.run_if(resource_changed::<crate::game::settings::GameSettings>),
                menu_button_system,
                checkbox_system,
            )
                .run_if(in_state(AppState::Menu)),
        );
    }
}
