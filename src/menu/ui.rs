use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    game::{score::LatestGameScores, settings::GameSettings},
    palette,
    state::{AppState, OnMenuScreen},
};

use super::{
    button::{self, DecreaseNButton, IncreaseNButton, PlayButton},
    checkbox::{Checkbox, ColorCheckBox, PositionCheckBox, SoundCheckbox},
    text::NBackText,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Menu),
            menu_ui.run_if(in_state(AppState::Menu)),
        )
        .add_systems(Update, mouse_scroll);
    }
}

pub fn menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    scores: ResMut<LatestGameScores>,
) {
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: palette::SLATE_800.into(),
                    ..default()
                })
                .with_children(|parent| {
                    game_title(parent, font.clone());
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_grow: 0.5,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: palette::SLATE_800.into(),

                    ..default()
                })
                .with_children(|parent| {
                    select_n(parent, &settings, font.clone());
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        grid_template_columns: vec![
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                        ],
                        grid_template_rows: vec![
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                        ],
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        padding: UiRect::all(Val::Px(24.0)),
                        ..default()
                    },
                    background_color: palette::SLATE_800.into(),
                    ..default()
                })
                .with_children(|parent| {
                    cue_selection(parent, &settings, font.clone());
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_grow: 0.5,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: palette::SLATE_800.into(),
                    ..default()
                })
                .with_children(|parent| {
                    play_button(parent, font.clone());
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        flex_grow: 0.8,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        grid_template_columns: vec![
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                        ],
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    background_color: palette::SLATE_900.into(),
                    ..default()
                })
                .with_children(|parent| {
                    score_history(parent, &scores, font.clone());
                });
        });
}

fn game_title(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent.spawn(TextBundle::from_section(
        "Dual-N-Back",
        TextStyle {
            font,
            font_size: 86.0,
            color: palette::LIME_500,
        },
    ));
}

fn select_n(parent: &mut ChildBuilder, settings: &Res<GameSettings>, font: Handle<Font>) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(5.0)),
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
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(5.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    settings.n.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                NBackText,
            ));
        });

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(5.0)),
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
                    font,
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn cue_selection(parent: &mut ChildBuilder, settings: &Res<GameSettings>, font: Handle<Font>) {
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
            font: font.clone(),
            font_size: 32.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));

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
        SoundCheckbox,
        Checkbox {
            checked: settings.sound,
        },
    ));

    parent.spawn(TextBundle::from_section(
        "Sound",
        TextStyle {
            font: font.clone(),
            font_size: 32.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));

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
            font: font.clone(),
            font_size: 32.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));
}

fn play_button(parent: &mut ChildBuilder, font: Handle<Font>) {
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
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn score_history(parent: &mut ChildBuilder, scores: &ResMut<LatestGameScores>, font: Handle<Font>) {
    parent.spawn(TextBundle::from_section(
        "N-Back",
        TextStyle {
            font: font.clone(),
            font_size: 32.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));

    parent.spawn(TextBundle::from_section(
        "Time",
        TextStyle {
            font: font.clone(),
            font_size: 32.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));

    parent.spawn(TextBundle::from_section(
        "Score",
        TextStyle {
            font: font.clone(),
            font_size: 32.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));

    for score in scores.0.iter() {
        parent.spawn(TextBundle::from_section(
            format!("{}", score.n),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));

        parent.spawn(TextBundle::from_section(
            format!("{:.2}s", score.total_rounds as f32 * score.round_duration),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));

        parent.spawn(TextBundle::from_section(
            format!("{}%", score.f1_score_percent),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
