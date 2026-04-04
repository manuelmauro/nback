use bevy::prelude::*;

use crate::state::{AppState, OnGameScreen};

use self::{
    button::{GameButtonBundle, Shortcut},
    text::{CurrentRoundText, round_system},
};

use super::settings::GameSettings;

pub mod button;
pub mod text;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), game_ui)
            .add_systems(Update, round_system);
    }
}

pub fn game_ui(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Game info - N-Back label
                    parent.spawn((
                        Text::new(format!("{}-Back", settings.n)),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                    // Round counter
                    parent.spawn((
                        Text::new(""),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        CurrentRoundText,
                    ));
                });

            parent
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|parent| {
                    // Position button
                    parent
                        .spawn(GameButtonBundle {
                            button: Button,
                            node: Node {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: button::BUTTON_BORDER_COLOR.into(),
                            background_color: button::NORMAL_BUTTON.into(),
                            shortcut: Shortcut(KeyCode::KeyA),
                            action: button::ButtonAction::SamePosition,
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Position (A)"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });

                    // Sound button
                    parent
                        .spawn(GameButtonBundle {
                            button: Button,
                            node: Node {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: button::BUTTON_BORDER_COLOR.into(),
                            background_color: button::NORMAL_BUTTON.into(),
                            shortcut: Shortcut(KeyCode::KeyS),
                            action: button::ButtonAction::SameSound,
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Sound (S)"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });

                    // Color button
                    parent
                        .spawn(GameButtonBundle {
                            button: Button,
                            node: Node {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: button::BUTTON_BORDER_COLOR.into(),
                            background_color: button::NORMAL_BUTTON.into(),
                            shortcut: Shortcut(KeyCode::KeyD),
                            action: button::ButtonAction::SameColor,
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Color (D)"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });
                });
        });
}
