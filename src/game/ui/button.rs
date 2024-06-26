use bevy::prelude::*;

use crate::{game::core::round::Answer, palette, state::AppState};

pub const NORMAL_BUTTON: Color = palette::SLATE_800;
pub const HOVERED_BUTTON: Color = palette::TEAL_600;
pub const PRESSED_BUTTON: Color = palette::TEAL_700;
pub const BUTTON_BORDER_COLOR: Color = palette::WHITE;
pub const PRESSED_BUTTON_BORDER_COLOR: Color = palette::WHITE;

#[derive(Component)]
pub struct Shortcut(pub KeyCode);

#[derive(Component)]
pub enum ButtonAction {
    SamePosition,
    SameSound,
    SameColor,
}

#[derive(Bundle)]
pub struct GameButtonBundle {
    pub button: ButtonBundle,
    pub shortcut: Shortcut,
    pub action: ButtonAction,
}

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (button_system, button_shortcut_system).run_if(in_state(AppState::Game)),
        );
    }
}

#[allow(clippy::type_complexity)]
fn button_system(
    mut answer: ResMut<Answer>,
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, action) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = BUTTON_BORDER_COLOR;
                match action {
                    ButtonAction::SamePosition => answer.position = true,
                    ButtonAction::SameSound => answer.sound = true,
                    ButtonAction::SameColor => answer.color = true,
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = BUTTON_BORDER_COLOR;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = BUTTON_BORDER_COLOR;
            }
        }
    }
}

fn button_shortcut_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut BackgroundColor, &mut BorderColor, &Shortcut), With<Button>>,
) {
    for (mut color, mut border_color, shortcut) in &mut query {
        if keyboard_input.pressed(shortcut.0) {
            *color = PRESSED_BUTTON.into();
            border_color.0 = PRESSED_BUTTON_BORDER_COLOR;
        }

        if keyboard_input.just_released(shortcut.0) {
            *color = NORMAL_BUTTON.into();
            border_color.0 = BUTTON_BORDER_COLOR;
        }
    }
}
