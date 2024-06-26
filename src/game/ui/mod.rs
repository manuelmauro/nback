use bevy::prelude::*;

use crate::state::{AppState, OnGameScreen};

use self::{
    button::{GameButtonBundle, Shortcut},
    text::{round_system, CurrentRoundText},
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
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    OnGameScreen,
                ))
                .with_children(|parent| game_info(parent, settings, font.clone()));

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::End,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                    OnGameScreen,
                ))
                .with_children(|parent| buttons(parent, font));
        });
}

fn game_info(parent: &mut ChildBuilder, settings: Res<GameSettings>, font: Handle<Font>) {
    parent.spawn(TextBundle::from_section(
        format!("{}-Back", settings.n),
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));
    parent.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ),
        CurrentRoundText,
    ));
}

fn buttons(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(GameButtonBundle {
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: button::BUTTON_BORDER_COLOR.into(),
                background_color: button::NORMAL_BUTTON.into(),
                ..default()
            },
            shortcut: Shortcut(KeyCode::KeyA),
            action: button::ButtonAction::SamePosition,
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Position (A)",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    parent
        .spawn(GameButtonBundle {
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: button::BUTTON_BORDER_COLOR.into(),
                background_color: button::NORMAL_BUTTON.into(),
                ..default()
            },
            shortcut: Shortcut(KeyCode::KeyS),
            action: button::ButtonAction::SameSound,
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Sound (S)",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    parent
        .spawn(GameButtonBundle {
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: button::BUTTON_BORDER_COLOR.into(),
                background_color: button::NORMAL_BUTTON.into(),
                ..default()
            },
            shortcut: Shortcut(KeyCode::KeyD),
            action: button::ButtonAction::SameColor,
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Color (D)",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
