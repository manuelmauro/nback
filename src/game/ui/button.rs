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

/// Returns a game button bundle as a tuple.
pub fn game_button(
    label: &str,
    font: Handle<Font>,
    shortcut: KeyCode,
    action: ButtonAction,
) -> impl Bundle + use<'_> {
    (
        Button,
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(3)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(BUTTON_BORDER_COLOR),
        BackgroundColor(NORMAL_BUTTON),
        Shortcut(shortcut),
        action,
        children![(
            Text::new(label),
            TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
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

type ButtonQuery<'w> = (
    &'w Interaction,
    &'w mut BackgroundColor,
    &'w mut BorderColor,
    &'w ButtonAction,
);

fn button_system(
    mut answer: ResMut<Answer>,
    mut query: Query<ButtonQuery, (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, mut border_color, action) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(BUTTON_BORDER_COLOR);
                match action {
                    ButtonAction::SamePosition => answer.position = true,
                    ButtonAction::SameSound => answer.sound = true,
                    ButtonAction::SameColor => answer.color = true,
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(BUTTON_BORDER_COLOR);
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(BUTTON_BORDER_COLOR);
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
            *border_color = BorderColor::all(PRESSED_BUTTON_BORDER_COLOR);
        }

        if keyboard_input.just_released(shortcut.0) {
            *color = NORMAL_BUTTON.into();
            *border_color = BorderColor::all(BUTTON_BORDER_COLOR);
        }
    }
}
