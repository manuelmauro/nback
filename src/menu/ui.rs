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

const SCORES_PER_PAGE: usize = 5;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScorePage(0))
            .add_systems(OnEnter(AppState::Menu), menu_ui)
            .add_systems(
                Update,
                (
                    mouse_scroll,
                    page_button_system,
                    rebuild_score_table.run_if(resource_changed::<ScorePage>),
                )
                    .run_if(in_state(AppState::Menu)),
            );
    }
}

/// Current page index for the score history table.
#[derive(Resource)]
pub struct ScorePage(pub usize);

/// Marker for the score table container so we can rebuild it.
#[derive(Component)]
struct ScoreTableContainer;

#[derive(Component)]
enum PageAction {
    Prev,
    Next,
}

/// Marker for the page info text (e.g. "1 / 3").
#[derive(Component)]
struct PageInfoText;

pub fn menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    scores: ResMut<ScoreHistory>,
    mut page: ResMut<ScorePage>,
) {
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");

    // Reset to first page when entering the menu (scores may have changed).
    page.0 = 0;

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
                // N selector section
                card_node(children![n_selector_row(&settings, font.clone())]),
                // Cue toggles section
                card_node(children![cue_grid(&settings, font.clone())]),
                // Primary menu actions
                play_button(font.clone()),
                quit_button(font.clone()),
            ],
        ))
        .id();

    spawn_score_section(&mut commands, root, &scores, font);
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

fn spawn_score_section(
    commands: &mut Commands,
    parent: Entity,
    scores: &ScoreHistory,
    font: Handle<Font>,
) {
    let total_pages = total_pages(scores.0.len());

    let section = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            width: px(360),
            row_gap: theme::SP_SM,
            ..default()
        },))
        .with_children(|col| {
            // Table
            col.spawn((
                ScoreTableContainer,
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(theme::SP_SM),
                    border_radius: BorderRadius::all(theme::RADIUS_LG),
                    ..default()
                },
                BackgroundColor(theme::SURFACE),
            ))
            .with_children(|table| {
                build_score_rows(table, scores, 0, font.clone());
            });

            // Pagination controls
            if total_pages > 1 {
                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: theme::SP_MD,
                    ..default()
                })
                .with_children(|row| {
                    // Prev
                    row.spawn((
                        Button,
                        PageAction::Prev,
                        Node {
                            width: px(40),
                            height: px(36),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(theme::RADIUS_SM),
                            ..default()
                        },
                        BackgroundColor(theme::SURFACE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("‹"),
                            TextFont {
                                font: font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(theme::TEXT),
                        ));
                    });

                    // Page info
                    row.spawn((
                        Text::new(format!("1 / {}", total_pages)),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(theme::TEXT_MUTED),
                        PageInfoText,
                    ));

                    // Next
                    row.spawn((
                        Button,
                        PageAction::Next,
                        Node {
                            width: px(40),
                            height: px(36),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(theme::RADIUS_SM),
                            ..default()
                        },
                        BackgroundColor(theme::SURFACE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("›"),
                            TextFont {
                                font: font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(theme::TEXT),
                        ));
                    });
                });
            }
        })
        .id();

    commands.entity(parent).add_child(section);
}

fn build_score_rows(
    table: &mut ChildSpawnerCommands,
    scores: &ScoreHistory,
    page: usize,
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

    // Header
    table
        .spawn(Node {
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

    // Data rows for this page
    let start = page * SCORES_PER_PAGE;
    let page_scores = scores.0.iter().skip(start).take(SCORES_PER_PAGE);

    for (i, score) in page_scores.enumerate() {
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

        table
            .spawn((
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
}

fn total_pages(count: usize) -> usize {
    if count == 0 {
        1
    } else {
        count.div_ceil(SCORES_PER_PAGE)
    }
}

type PageQuery<'w> = (&'w Interaction, &'w PageAction);

/// Handle prev/next page button presses.
fn page_button_system(
    scores: Res<ScoreHistory>,
    mut page: ResMut<ScorePage>,
    query: Query<PageQuery, (Changed<Interaction>, With<Button>)>,
) {
    let max_page = total_pages(scores.0.len()).saturating_sub(1);
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                PageAction::Prev => {
                    page.0 = page.0.saturating_sub(1);
                }
                PageAction::Next => {
                    page.0 = (page.0 + 1).min(max_page);
                }
            }
        }
    }
}

/// Rebuild the score table rows when the page changes.
fn rebuild_score_table(
    mut commands: Commands,
    page: Res<ScorePage>,
    scores: Res<ScoreHistory>,
    asset_server: Res<AssetServer>,
    table_query: Query<(Entity, &Children), With<ScoreTableContainer>>,
    mut page_text: Query<&mut Text, With<PageInfoText>>,
) {
    let Ok((table_entity, children)) = table_query.single() else {
        return;
    };

    // Despawn old rows
    for child in children.iter() {
        commands.entity(child).despawn();
    }

    // Rebuild
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");
    commands.entity(table_entity).with_children(|table| {
        build_score_rows(table, &scores, page.0, font);
    });

    // Update page info text
    let pages = total_pages(scores.0.len());
    if let Ok(mut text) = page_text.single_mut() {
        text.0 = format!("{} / {}", page.0 + 1, pages);
    }
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
