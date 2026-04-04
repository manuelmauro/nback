use bevy::prelude::*;

use crate::{
    game::{
        phase::GamePhase,
        session::{Session, answer::Answer},
    },
    theme,
};

#[derive(Component)]
pub struct Shortcut(pub KeyCode);

#[derive(Component)]
pub enum ButtonAction {
    SamePosition,
    SameColor,
    SameShape,
    SameSound,
}

/// Returns a compact pill-shaped game button.
pub fn game_button(
    label: &str,
    font: Handle<Font>,
    shortcut: KeyCode,
    action: ButtonAction,
) -> impl Bundle + use<'_> {
    (
        Button,
        Node {
            flex_grow: 1.0,
            height: px(56),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(theme::RADIUS_MD),
            ..default()
        },
        BackgroundColor(theme::GAME_BTN),
        Shortcut(shortcut),
        action,
        children![(
            Text::new(label),
            TextFont {
                font,
                font_size: 18.0,
                ..default()
            },
            TextColor(theme::TEXT),
        )],
    )
}

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (button_system, button_shortcut_system).run_if(in_state(GamePhase::Playing)),
        );
    }
}

type ButtonQuery<'w> = (&'w Interaction, &'w mut BackgroundColor, &'w ButtonAction);

fn button_system(
    mut answer: Single<&mut Answer, With<Session>>,
    mut query: Query<ButtonQuery, (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, action) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = theme::GAME_BTN_PRESS.into();
                match action {
                    ButtonAction::SamePosition => answer.position = true,
                    ButtonAction::SameColor => answer.color = true,
                    ButtonAction::SameShape => answer.shape = true,
                    ButtonAction::SameSound => answer.sound = true,
                }
            }
            Interaction::Hovered => {
                *color = theme::GAME_BTN_HOVER.into();
            }
            Interaction::None => {
                *color = theme::GAME_BTN.into();
            }
        }
    }
}

fn button_shortcut_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut answer: Single<&mut Answer, With<Session>>,
    mut query: Query<(&mut BackgroundColor, &Shortcut, &ButtonAction), With<Button>>,
) {
    for (mut color, shortcut, action) in &mut query {
        if keyboard_input.pressed(shortcut.0) {
            *color = theme::GAME_BTN_PRESS.into();
        }
        if keyboard_input.just_pressed(shortcut.0) {
            match action {
                ButtonAction::SamePosition => answer.position = true,
                ButtonAction::SameColor => answer.color = true,
                ButtonAction::SameShape => answer.shape = true,
                ButtonAction::SameSound => answer.sound = true,
            }
        }
        if keyboard_input.just_released(shortcut.0) {
            *color = theme::GAME_BTN.into();
        }
    }
}
