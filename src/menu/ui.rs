use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    game::{score::ScoreHistory, settings::GameSettings},
    state::AppState,
    theme,
};

use super::{
    button::{MenuButtonAction, RestingColor},
    checkbox::{CheckMark, Checkbox, CheckboxAction},
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
                align_items: AlignItems::Center,
                padding: UiRect::all(theme::SP_MD),
                row_gap: theme::SP_SM,
                ..default()
            },
            children![
                // Title
                (
                    Node {
                        justify_content: JustifyContent::Center,
                        padding: UiRect::vertical(theme::SP_LG),
                        ..default()
                    },
                    children![game_title(font.clone())],
                ),
                // N selector card
                card_node(children![n_selector_row(&settings, font.clone())]),
                // Cue toggles card
                card_node(children![cue_grid(&settings, font.clone())]),
                // Play / Quit buttons
                play_button(font.clone()),
                quit_button(font.clone()),
            ],
        ))
        .id();

    spawn_score_history(&mut commands, root, &scores, font);
}

// ── widgets ──────────────────────────────────────────────────────────

fn card_node(children: impl Bundle) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            width: px(360),
            padding: UiRect::all(theme::SP_LG),
            border_radius: BorderRadius::all(theme::RADIUS_LG),
            ..default()
        },
        BackgroundColor(theme::SURFACE),
        children,
    )
}

fn game_title(font: Handle<Font>) -> impl Bundle {
    (
        Text::new("Dual-N-Back"),
        TextFont {
            font,
            font_size: 72.0,
            ..default()
        },
        TextColor(theme::ACCENT),
    )
}

fn n_selector_row(settings: &GameSettings, font: Handle<Font>) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: theme::SP_LG,
            ..default()
        },
        children![
            round_button("-", MenuButtonAction::DecreaseN, font.clone()),
            nback_label(settings, font.clone()),
            round_button("+", MenuButtonAction::IncreaseN, font),
        ],
    )
}

fn round_button(
    label: &str,
    action: MenuButtonAction,
    font: Handle<Font>,
) -> impl Bundle + use<'_> {
    (
        Button,
        action,
        Node {
            width: px(52),
            height: px(52),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(theme::RADIUS_FULL),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme::SURFACE),
        RestingColor(theme::SURFACE),
        BorderColor::all(theme::BORDER),
        children![(
            Text::new(label),
            TextFont {
                font,
                font_size: 36.0,
                ..default()
            },
            TextColor(theme::TEXT),
        )],
    )
}

fn nback_label(settings: &GameSettings, font: Handle<Font>) -> impl Bundle {
    (
        Node {
            min_width: px(120),
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new(format!("{}-Back", settings.n)),
            TextFont {
                font,
                font_size: 48.0,
                ..default()
            },
            TextColor(theme::TEXT),
            NBackText,
        )],
    )
}

fn cue_grid(settings: &GameSettings, font: Handle<Font>) -> impl Bundle {
    (
        Node {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::min_content(), GridTrack::fr(1.0)],
            row_gap: theme::SP_MD,
            column_gap: theme::SP_LG,
            ..default()
        },
        children![
            checkbox(settings.position, CheckboxAction::Position),
            cue_label("Position", font.clone()),
            checkbox(settings.color, CheckboxAction::Color),
            cue_label("Color", font.clone()),
            checkbox(settings.shape, CheckboxAction::Shape),
            cue_label("Shape", font.clone()),
            checkbox(settings.sound, CheckboxAction::Sound),
            cue_label("Sound", font),
        ],
    )
}

fn checkbox(checked: bool, action: CheckboxAction) -> impl Bundle {
    let (bg, mark_color) = if checked {
        (theme::ACCENT, theme::BG)
    } else {
        (theme::SURFACE, Color::NONE)
    };

    (
        Button,
        action,
        Checkbox { checked },
        Node {
            width: px(36),
            height: px(36),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(theme::RADIUS_SM),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(theme::BORDER),
        BackgroundColor(bg),
        children![(
            Node {
                width: px(16),
                height: px(16),
                border_radius: BorderRadius::all(theme::RADIUS_SM),
                ..default()
            },
            BackgroundColor(mark_color),
            CheckMark,
        )],
    )
}

fn cue_label(label: &str, font: Handle<Font>) -> impl Bundle + use<'_> {
    (
        Text::new(label),
        TextFont {
            font,
            font_size: 28.0,
            ..default()
        },
        TextColor(theme::TEXT),
        Node {
            align_self: AlignSelf::Center,
            ..default()
        },
    )
}

fn play_button(font: Handle<Font>) -> impl Bundle {
    (
        Button,
        MenuButtonAction::Play,
        Node {
            width: px(360),
            height: px(64),
            border_radius: BorderRadius::all(theme::RADIUS_MD),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme::ACCENT),
        RestingColor(theme::ACCENT),
        children![(
            Text::new("PLAY"),
            TextFont {
                font,
                font_size: 32.0,
                ..default()
            },
            TextColor(theme::BG),
        )],
    )
}

fn quit_button(font: Handle<Font>) -> impl Bundle {
    (
        Button,
        MenuButtonAction::Quit,
        Node {
            width: px(360),
            height: px(48),
            border_radius: BorderRadius::all(theme::RADIUS_MD),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme::SURFACE_ALT),
        RestingColor(theme::SURFACE_ALT),
        children![(
            Text::new("Quit"),
            TextFont {
                font,
                font_size: 24.0,
                ..default()
            },
            TextColor(theme::TEXT_MUTED),
        )],
    )
}

// ── score history ────────────────────────────────────────────────────

fn spawn_score_history(
    commands: &mut Commands,
    parent: Entity,
    scores: &ScoreHistory,
    font: Handle<Font>,
) {
    let header_font = TextFont {
        font: font.clone(),
        font_size: 22.0,
        ..default()
    };
    let row_font = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..default()
    };

    let score_section = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                width: px(360),
                padding: UiRect::all(theme::SP_SM),
                border_radius: BorderRadius::all(theme::RADIUS_LG),
                ..default()
            },
            BackgroundColor(theme::SURFACE),
        ))
        .with_children(|col| {
            // Header row
            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|row| {
                for label in ["N", "Time", "Score", "Date"] {
                    row.spawn((
                        Text::new(label),
                        header_font.clone(),
                        TextColor(theme::TEXT_MUTED),
                        Node {
                            flex_basis: percent(25),
                            padding: UiRect::all(theme::SP_SM),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                    ));
                }
            });

            // Data rows
            for (i, score) in scores.0.iter().enumerate() {
                let bg = if i % 2 == 0 {
                    Color::NONE
                } else {
                    theme::SURFACE_ALT
                };
                let date_short = score
                    .played_at
                    .get(..10)
                    .unwrap_or(&score.played_at)
                    .to_string();

                col.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(theme::RADIUS_SM),
                        ..default()
                    },
                    BackgroundColor(bg),
                ))
                .with_children(|row| {
                    for text in [
                        format!("{}", score.n),
                        format!("{:.0}s", score.total_rounds as f32 * score.round_duration),
                        format!("{}%", score.f1_score_percent),
                        date_short.clone(),
                    ] {
                        row.spawn((
                            Text::new(text),
                            row_font.clone(),
                            TextColor(theme::TEXT),
                            Node {
                                flex_basis: percent(25),
                                padding: UiRect::all(theme::SP_SM),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                        ));
                    }
                });
            }
        })
        .id();

    commands.entity(parent).add_child(score_section);
}

// ── scroll ───────────────────────────────────────────────────────────

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
