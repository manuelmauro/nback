use bevy::prelude::*;

use crate::config;

#[derive(Component)]
pub struct Shortcut(pub KeyCode);

#[derive(Bundle)]
pub struct GameButtonBundle {
    pub button: ButtonBundle,
    pub shortcut: Shortcut,
}

impl Default for GameButtonBundle {
    fn default() -> Self {
        Self {
            button: ButtonBundle::default(),
            shortcut: Shortcut(KeyCode::Space),
        }
    }
}

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system)
            .add_systems(Update, button_shortcut_system);
    }
}

fn button_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = config::PRESSED_BUTTON.into();
                border_color.0 = config::BUTTON_BORDER_COLOR.into();
            }
            Interaction::Hovered => {
                *color = config::HOVERED_BUTTON.into();
                border_color.0 = config::BUTTON_BORDER_COLOR.into();
            }
            Interaction::None => {
                *color = config::NORMAL_BUTTON.into();
                border_color.0 = config::BUTTON_BORDER_COLOR.into();
            }
        }
    }
}

fn button_shortcut_system(
    mut interaction_query: Query<(&mut BackgroundColor, &mut BorderColor, &Shortcut), With<Button>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut color, mut border_color, shortcut) in &mut interaction_query {
        if keyboard_input.pressed(shortcut.0) {
            *color = config::PRESSED_BUTTON.into();
            border_color.0 = config::PRESSED_BUTTON_BORDER_COLOR.into();
        }

        if keyboard_input.just_released(shortcut.0) {
            *color = config::NORMAL_BUTTON.into();
            border_color.0 = config::BUTTON_BORDER_COLOR.into();
        }
    }
}
