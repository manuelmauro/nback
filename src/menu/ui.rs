use bevy::prelude::*;

use crate::{
    game::{score::LatestGameScores, settings::GameSettings},
    palette,
    state::{AppState, OnMenuScreen},
};

use super::{
    button::{self, DecreaseNButton, IncreaseNButton, PlayButton},
    checkbox::{Checkbox, ColorCheckBox, PositionCheckBox, SoundCheckBox},
    text::NBackText,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Menu),
            menu_ui.run_if(in_state(AppState::Menu)),
        );
    }
}

pub fn menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    scores: ResMut<LatestGameScores>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Dual-N-Back",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 96.0,
                    color: palette::LIME_500,
                },
            ),));

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(75.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    OnMenuScreen,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    border: UiRect::all(Val::Px(3.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: button::BUTTON_BORDER_COLOR.into(),
                                background_color: button::NORMAL_BUTTON.into(),
                                ..default()
                            },
                            DecreaseNButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "-",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    parent.spawn((
                        TextBundle::from_section(
                            settings.n.to_string(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        NBackText,
                    ));

                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    border: UiRect::all(Val::Px(3.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: button::BUTTON_BORDER_COLOR.into(),
                                background_color: button::NORMAL_BUTTON.into(),
                                ..default()
                            },
                            IncreaseNButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "+",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(75.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    OnMenuScreen,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: button::BUTTON_BORDER_COLOR.into(),
                            background_color: button::PRESSED_BUTTON.into(),
                            ..default()
                        },
                        PositionCheckBox,
                        Checkbox {
                            checked: settings.position,
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        "Position",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(75.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    OnMenuScreen,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: button::BUTTON_BORDER_COLOR.into(),
                            background_color: button::PRESSED_BUTTON.into(),
                            ..default()
                        },
                        SoundCheckBox,
                        Checkbox {
                            checked: settings.sound,
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        "Sound",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(75.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    OnMenuScreen,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: button::BUTTON_BORDER_COLOR.into(),
                            background_color: button::PRESSED_BUTTON.into(),
                            ..default()
                        },
                        ColorCheckBox,
                        Checkbox {
                            checked: settings.color,
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        "Color",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    OnMenuScreen,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
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
                            PlayButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "PLAY",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(75.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    OnMenuScreen,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "N-Back",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        "Time",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        "Score",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            for score in scores.0.iter() {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        OnMenuScreen,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{}", score.n),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));

                        parent.spawn(TextBundle::from_section(
                            format!("{:.2}s", score.total_rounds as f32 * score.round_duration),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));

                        parent.spawn(TextBundle::from_section(
                            format!("{}%", score.f1_score_percent),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }
        });
}
