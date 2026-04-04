use bevy::prelude::*;

use crate::{state::AppState, theme};

use self::{
    button::{ButtonAction, game_button},
    text::{CurrentRoundText, TimerBar, round_system, timer_bar_system},
};

use super::settings::GameSettings;

pub mod button;
pub mod text;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), game_ui)
            .add_systems(
                Update,
                (round_system, timer_bar_system).run_if(in_state(AppState::Game)),
            );
    }
}

pub fn game_ui(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");

    commands.spawn((
        DespawnOnExit(AppState::Game),
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(theme::SP_MD),
            ..default()
        },
        children![
            // Top bar: N-Back + Round counter
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: percent(100),
                    ..default()
                },
                children![
                    (
                        Text::new(format!("{}-Back", settings.n)),
                        TextFont {
                            font: font.clone(),
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(theme::TEXT),
                    ),
                    (
                        Text::new(""),
                        TextFont {
                            font: font.clone(),
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(theme::TEXT_MUTED),
                        CurrentRoundText,
                    ),
                ],
            ),
            // Timer progress bar
            (
                Node {
                    width: percent(100),
                    height: px(6),
                    margin: UiRect::vertical(theme::SP_SM),
                    border_radius: BorderRadius::all(theme::RADIUS_FULL),
                    ..default()
                },
                BackgroundColor(theme::TIMER_TRACK),
                children![(
                    Node {
                        width: percent(0),
                        height: percent(100),
                        border_radius: BorderRadius::all(theme::RADIUS_FULL),
                        ..default()
                    },
                    BackgroundColor(theme::TIMER_FILL),
                    TimerBar,
                )],
            ),
            // Spacer
            (Node {
                flex_grow: 1.0,
                ..default()
            },),
            // Action buttons
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: theme::SP_SM,
                    width: percent(100),
                    ..default()
                },
                children![
                    game_button(
                        "Pos (A)",
                        font.clone(),
                        KeyCode::KeyA,
                        ButtonAction::SamePosition
                    ),
                    game_button(
                        "Col (S)",
                        font.clone(),
                        KeyCode::KeyS,
                        ButtonAction::SameColor
                    ),
                    game_button(
                        "Shp (D)",
                        font.clone(),
                        KeyCode::KeyD,
                        ButtonAction::SameShape
                    ),
                    game_button(
                        "Snd (F)",
                        font.clone(),
                        KeyCode::KeyF,
                        ButtonAction::SameSound
                    ),
                ],
            ),
        ],
    ));
}
