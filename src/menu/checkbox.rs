use bevy::prelude::*;

use crate::{game::settings::GameSettings, palette};

pub const NORMAL_BUTTON: Color = palette::SLATE_800;
pub const HOVERED_BUTTON: Color = palette::LIME_900;
pub const PRESSED_BUTTON: Color = palette::LIME_500;
pub const BUTTON_BORDER_COLOR: Color = palette::WHITE;
pub const PRESSED_BUTTON_BORDER_COLOR: Color = palette::WHITE;

#[derive(Component, Default)]
pub struct Checkbox {
    pub checked: bool,
}

#[derive(Component)]
pub struct PositionCheckBox;

#[allow(clippy::type_complexity)]
pub fn position_checkbox_system(
    mut settings: ResMut<GameSettings>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut Checkbox),
        (Changed<Interaction>, With<PositionCheckBox>),
    >,
) {
    for (interaction, mut color, mut checkbox) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                if checkbox.checked {
                    *color = NORMAL_BUTTON.into();
                    checkbox.checked = false;
                } else {
                    *color = PRESSED_BUTTON.into();
                    checkbox.checked = true;
                }

                settings.position = checkbox.checked;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Component)]
pub struct SoundCheckbox;

#[allow(clippy::type_complexity)]
pub fn sound_checkbox_system(
    mut settings: ResMut<GameSettings>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut Checkbox),
        (Changed<Interaction>, With<SoundCheckbox>),
    >,
) {
    for (interaction, mut color, mut checkbox) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                if checkbox.checked {
                    *color = NORMAL_BUTTON.into();
                    checkbox.checked = false;
                } else {
                    *color = PRESSED_BUTTON.into();
                    checkbox.checked = true;
                }

                settings.sound = checkbox.checked;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Component)]
pub struct ColorCheckBox;

#[allow(clippy::type_complexity)]
pub fn color_checkbox_system(
    mut settings: ResMut<GameSettings>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut Checkbox),
        (Changed<Interaction>, With<ColorCheckBox>),
    >,
) {
    for (interaction, mut color, mut checkbox) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                if checkbox.checked {
                    *color = NORMAL_BUTTON.into();
                    checkbox.checked = false;
                } else {
                    *color = PRESSED_BUTTON.into();
                    checkbox.checked = true;
                }

                settings.color = checkbox.checked;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
