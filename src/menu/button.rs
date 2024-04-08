use bevy::prelude::*;

use crate::{game::settings::GameSettings, palette, state::AppState};

pub const NORMAL_BUTTON: Color = palette::SLATE_800;
pub const HOVERED_BUTTON: Color = palette::LIME_900;
pub const PRESSED_BUTTON: Color = palette::LIME_500;
pub const BUTTON_BORDER_COLOR: Color = palette::WHITE;
pub const PRESSED_BUTTON_BORDER_COLOR: Color = palette::WHITE;

#[derive(Component)]
pub struct PlayButton;

#[allow(clippy::type_complexity)]
pub fn play_button_system(
    mut app_state: ResMut<NextState<AppState>>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
pub struct IncreaseNButton;

#[allow(clippy::type_complexity)]
pub fn increase_n_button_system(
    mut settings: ResMut<GameSettings>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<IncreaseNButton>),
    >,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                settings.n += 1;
                settings.set_rounds_from_n();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
pub struct DecreaseNButton;

#[allow(clippy::type_complexity)]
pub fn decrease_n_button_system(
    mut settings: ResMut<GameSettings>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DecreaseNButton>),
    >,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                settings.n -= 1;
                settings.set_rounds_from_n();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
