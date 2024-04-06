use bevy::prelude::*;

use crate::palette;

pub const NORMAL_BUTTON: Color = palette::SLATE_800;
pub const HOVERED_BUTTON: Color = palette::PURPLE_400;
pub const PRESSED_BUTTON: Color = palette::PURPLE_500;
pub const BUTTON_BORDER_COLOR: Color = palette::WHITE;
pub const PRESSED_BUTTON_BORDER_COLOR: Color = palette::WHITE;

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

#[allow(clippy::type_complexity)]
fn button_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = BUTTON_BORDER_COLOR;
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
