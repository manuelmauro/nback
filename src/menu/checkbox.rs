use bevy::prelude::*;

use crate::{game::settings::GameSettings, theme};

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

/// Marker for the "✓" text child so we can toggle its visibility.
#[derive(Component)]
pub struct CheckMark;

type CheckboxQuery<'w> = (
    &'w Interaction,
    &'w mut BackgroundColor,
    &'w mut Checkbox,
    &'w CheckboxAction,
    &'w Children,
);

pub fn checkbox_system(
    mut settings: ResMut<GameSettings>,
    mut query: Query<CheckboxQuery, (Changed<Interaction>, With<Button>)>,
    mut check_marks: Query<&mut TextColor, With<CheckMark>>,
) {
    for (interaction, mut color, mut checkbox, action, children) in &mut query {
        if *interaction == Interaction::Pressed {
            checkbox.checked = !checkbox.checked;
            *color = if checkbox.checked {
                theme::ACCENT
            } else {
                theme::SURFACE
            }
            .into();

            // Toggle check-mark visibility
            for child in children.iter() {
                if let Ok(mut text_color) = check_marks.get_mut(child) {
                    text_color.0 = if checkbox.checked {
                        theme::BG
                    } else {
                        Color::NONE
                    };
                }
            }

            match action {
                CheckboxAction::Position => settings.position = checkbox.checked,
                CheckboxAction::Color => settings.color = checkbox.checked,
                CheckboxAction::Shape => settings.shape = checkbox.checked,
                CheckboxAction::Sound => settings.sound = checkbox.checked,
            }
        }
    }
}
