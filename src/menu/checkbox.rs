use bevy::prelude::*;

use crate::{game::settings::GameSettings, palette};

pub const NORMAL_BUTTON: Color = palette::SLATE_800;
pub const PRESSED_BUTTON: Color = palette::LIME_500;
pub const BUTTON_BORDER_COLOR: Color = palette::WHITE;

#[derive(Component, Default)]
pub struct Checkbox {
    pub checked: bool,
}

/// Which settings field this checkbox controls.
#[derive(Component)]
pub enum CheckboxAction {
    Position,
    Color,
    Shape,
    Sound,
}

type CheckboxQuery<'w> = (
    &'w Interaction,
    &'w mut BackgroundColor,
    &'w mut Checkbox,
    &'w CheckboxAction,
);

pub fn checkbox_system(
    mut settings: ResMut<GameSettings>,
    mut query: Query<CheckboxQuery, (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, mut checkbox, action) in &mut query {
        if *interaction == Interaction::Pressed {
            checkbox.checked = !checkbox.checked;
            *color = if checkbox.checked {
                PRESSED_BUTTON
            } else {
                NORMAL_BUTTON
            }
            .into();

            match action {
                CheckboxAction::Position => settings.position = checkbox.checked,
                CheckboxAction::Color => settings.color = checkbox.checked,
                CheckboxAction::Shape => settings.shape = checkbox.checked,
                CheckboxAction::Sound => settings.sound = checkbox.checked,
            }
        }
    }
}
