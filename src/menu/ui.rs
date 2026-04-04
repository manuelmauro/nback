use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    game::{score::ScoreHistory, settings::GameSettings},
    palette,
    state::AppState,
};

use super::{
    button::{self, MenuButtonAction},
    checkbox::{Checkbox, CheckboxAction},
    text::NBackText,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), menu_ui)
            .add_systems(Update, mouse_scroll.run_if(in_state(AppState::Menu)));
    }
}

pub fn menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    scores: ResMut<ScoreHistory>,
) {
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");

    let root = commands
        .spawn((
            DespawnOnExit(AppState::Menu),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(px(5)),
                ..default()
            },
            children![
                // Title
                (
                    Node {
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(px(5)),
                        ..default()
                    },
                    BackgroundColor(palette::SLATE_800),
                    children![game_title(font.clone())],
                ),
                // N selector
                (
                    Node {
                        flex_grow: 0.5,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(px(5)),
                        ..default()
                    },
                    BackgroundColor(palette::SLATE_800),
                    children![
                        decrease_n_button(font.clone()),
                        nback_label(&settings, font.clone()),
                        increase_n_button(font.clone()),
                    ],
                ),
                // Cue selection
                (
                    Node {
                        display: Display::Grid,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(px(5)),
                        grid_template_columns: vec![
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                        ],
                        grid_template_rows: vec![
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                        ],
                        row_gap: px(12),
                        column_gap: px(12),
                        padding: UiRect::all(px(24)),
                        ..default()
                    },
                    BackgroundColor(palette::SLATE_800),
                    children![
                        checkbox(settings.position, CheckboxAction::Position),
                        cue_label("Position", font.clone()),
                        checkbox(settings.color, CheckboxAction::Color),
                        cue_label("Color", font.clone()),
                        checkbox(settings.shape, CheckboxAction::Shape),
                        cue_label("Shape", font.clone()),
                        checkbox(settings.sound, CheckboxAction::Sound),
                        cue_label("Sound", font.clone()),
                    ],
                ),
                // Play button
                (
                    Node {
                        flex_grow: 0.5,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(px(5)),
                        ..default()
                    },
                    BackgroundColor(palette::SLATE_800),
                    children![play_button(font.clone())],
                ),
            ],
        ))
        .id();

    // Score history — spawned separately because children are dynamic
    spawn_score_history(&mut commands, root, &scores, font);
}

fn game_title(font: Handle<Font>) -> impl Bundle {
    (
        Text::new("Dual-N-Back"),
        TextFont {
            font,
            font_size: 86.0,
            ..default()
        },
        TextColor(palette::LIME_500),
    )
}

fn decrease_n_button(font: Handle<Font>) -> impl Bundle {
    (
        Button,
        MenuButtonAction::DecreaseN,
        Node {
            width: px(40),
            height: px(40),
            border: UiRect::all(px(3)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(px(5)),
            ..default()
        },
        BorderColor::all(button::BUTTON_BORDER_COLOR),
        BackgroundColor(button::NORMAL_BUTTON),
        children![(
            Text::new("-"),
            TextFont {
                font,
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}

fn nback_label(settings: &GameSettings, font: Handle<Font>) -> impl Bundle {
    (
        Node {
            margin: UiRect::all(px(5)),
            ..default()
        },
        children![(
            Text::new(format!("{}-Back", settings.n)),
            TextFont {
                font,
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            NBackText,
        )],
    )
}

fn increase_n_button(font: Handle<Font>) -> impl Bundle {
    (
        Button,
        MenuButtonAction::IncreaseN,
        Node {
            width: px(40),
            height: px(40),
            border: UiRect::all(px(3)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(px(5)),
            ..default()
        },
        BorderColor::all(button::BUTTON_BORDER_COLOR),
        BackgroundColor(button::NORMAL_BUTTON),
        children![(
            Text::new("+"),
            TextFont {
                font,
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}

fn checkbox(checked: bool, action: CheckboxAction) -> impl Bundle {
    let bg = if checked {
        super::checkbox::PRESSED_BUTTON
    } else {
        super::checkbox::NORMAL_BUTTON
    };

    (
        Button,
        Node {
            width: px(32),
            height: px(32),
            border: UiRect::all(px(3)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(super::checkbox::BUTTON_BORDER_COLOR),
        BackgroundColor(bg),
        action,
        Checkbox { checked },
    )
}

fn cue_label(label: &str, font: Handle<Font>) -> impl Bundle + use<'_> {
    (
        Text::new(label),
        TextFont {
            font,
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    )
}

fn play_button(font: Handle<Font>) -> impl Bundle {
    (
        Button,
        MenuButtonAction::Play,
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(3)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(button::BUTTON_BORDER_COLOR),
        BackgroundColor(button::NORMAL_BUTTON),
        children![(
            Text::new("PLAY"),
            TextFont {
                font,
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}

fn spawn_score_history(
    commands: &mut Commands,
    parent: Entity,
    scores: &ScoreHistory,
    font: Handle<Font>,
) {
    let header_style = TextFont {
        font: font.clone(),
        font_size: 32.0,
        ..default()
    };
    let header_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    let row_style = TextFont {
        font: font.clone(),
        font_size: 24.0,
        ..default()
    };
    let row_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    let score_section = commands
        .spawn((
            Node {
                display: Display::Grid,
                flex_grow: 0.8,
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                margin: UiRect::all(px(5)),
                grid_template_columns: vec![
                    GridTrack::auto(),
                    GridTrack::auto(),
                    GridTrack::auto(),
                ],
                row_gap: px(12),
                column_gap: px(12),
                padding: UiRect::all(px(12)),
                ..default()
            },
            BackgroundColor(palette::SLATE_900),
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("N-Back"), header_style.clone(), header_color));
            parent.spawn((Text::new("Time"), header_style.clone(), header_color));
            parent.spawn((Text::new("Score"), header_style.clone(), header_color));

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
        })
        .id();

    commands.entity(parent).add_child(score_section);
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
            node.top = px(scrolling_list.position);
        }
    }
}
