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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(palette::SLATE_800),
                ))
                .with_children(|parent| {
                    game_title(parent, font.clone());
                });

            parent
                .spawn((
                    Node {
                        flex_grow: 0.5,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(palette::SLATE_800),
                ))
                .with_children(|parent| {
                    select_n(parent, &settings, font.clone());
                });

            parent
                .spawn((
                    Node {
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
                    BackgroundColor(palette::SLATE_800),
                ))
                .with_children(|parent| {
                    cue_selection(parent, &settings, font.clone());
                });

            parent
                .spawn((
                    Node {
                        flex_grow: 0.5,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(palette::SLATE_800),
                ))
                .with_children(|parent| {
                    play_button(parent, font.clone());
                });

            parent
                .spawn((
                    Node {
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
                    BackgroundColor(palette::SLATE_900),
                ))
                .with_children(|parent| {
                    score_history(parent, &scores, font.clone());
                });
        });
}

fn game_title(parent: &mut ChildSpawnerCommands, font: Handle<Font>) {
    parent.spawn((
        Text::new("Dual-N-Back"),
        TextFont {
            font,
            font_size: 86.0,
            ..default()
        },
        TextColor(palette::LIME_500),
    ));
}

fn select_n(parent: &mut ChildSpawnerCommands, settings: &Res<GameSettings>, font: Handle<Font>) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(button::BUTTON_BORDER_COLOR),
            BackgroundColor(button::NORMAL_BUTTON),
            DecreaseNButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("-"),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });

    parent
        .spawn(Node {
            margin: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new(settings.n.to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                NBackText,
            ));
        });

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(button::BUTTON_BORDER_COLOR),
            BackgroundColor(button::NORMAL_BUTTON),
            IncreaseNButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("+"),
                TextFont {
                    font,
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn cue_selection(parent: &mut ChildSpawnerCommands, settings: &Res<GameSettings>, font: Handle<Font>) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            border: UiRect::all(Val::Px(3.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(button::BUTTON_BORDER_COLOR),
        BackgroundColor(button::PRESSED_BUTTON),
        PositionCheckBox,
        Checkbox {
            checked: settings.position,
        },
    ));

    parent.spawn((
        Text::new("Position"),
        TextFont {
            font: font.clone(),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));

    parent.spawn((
        Button,
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            border: UiRect::all(Val::Px(3.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(button::BUTTON_BORDER_COLOR),
        BackgroundColor(button::PRESSED_BUTTON),
        SoundCheckbox,
        Checkbox {
            checked: settings.sound,
        },
    ));

    parent.spawn((
        Text::new("Sound"),
        TextFont {
            font: font.clone(),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));

    parent.spawn((
        Button,
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            border: UiRect::all(Val::Px(3.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(button::BUTTON_BORDER_COLOR),
        BackgroundColor(button::PRESSED_BUTTON),
        ColorCheckBox,
        Checkbox {
            checked: settings.color,
        },
    ));

    parent.spawn((
        Text::new("Color"),
        TextFont {
            font: font.clone(),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));
}

fn play_button(parent: &mut ChildSpawnerCommands, font: Handle<Font>) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(button::BUTTON_BORDER_COLOR),
            BackgroundColor(button::NORMAL_BUTTON),
            PlayButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PLAY"),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn score_history(
    parent: &mut ChildSpawnerCommands,
    scores: &ResMut<LatestGameScores>,
    font: Handle<Font>,
) {
    let header_style = TextFont {
        font: font.clone(),
        font_size: 32.0,
        ..default()
    };
    let header_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    parent.spawn((Text::new("N-Back"), header_style.clone(), header_color));
    parent.spawn((Text::new("Time"), header_style.clone(), header_color));
    parent.spawn((Text::new("Score"), header_style.clone(), header_color));

    let row_style = TextFont {
        font: font.clone(),
        font_size: 24.0,
        ..default()
    };
    let row_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    for score in scores.0.iter() {
        parent.spawn((
            Text::new(format!("{}", score.n)),
            row_style.clone(),
            row_color,
        ));
        parent.spawn((
            Text::new(format!(
                "{:.2}s",
                score.total_rounds as f32 * score.round_duration
            )),
            row_style.clone(),
            row_color,
        ));
        parent.spawn((
            Text::new(format!("{}%", score.f1_score_percent)),
            row_style.clone(),
            row_color,
        ));
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Node, &ChildOf, &ComputedNode)>,
    query_node: Query<&ComputedNode>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut node, child_of, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(child_of.parent()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            node.top = Val::Px(scrolling_list.position);
        }
    }
}
