use bevy::prelude::*;

use crate::state::AppState;

use self::{
    button::{ButtonAction, game_button},
    text::{CurrentRoundText, round_system},
};

use super::settings::GameSettings;

pub mod button;
pub mod text;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), game_ui)
            .add_systems(Update, round_system.run_if(in_state(AppState::Game)));
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
            padding: UiRect::all(px(20)),
            ..default()
        },
        children![
            // Top bar: N-Back label + Round counter
            (
                Node {
                    flex_grow: 1.0,
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
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ),
                    (
                        Text::new(""),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        CurrentRoundText,
                    ),
                ],
            ),
            // Bottom bar: action buttons
            (
                Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                children![
                    game_button(
                        "Position (A)",
                        font.clone(),
                        KeyCode::KeyA,
                        ButtonAction::SamePosition
                    ),
                    game_button(
                        "Color (S)",
                        font.clone(),
                        KeyCode::KeyS,
                        ButtonAction::SameColor
                    ),
                    game_button(
                        "Shape (D)",
                        font.clone(),
                        KeyCode::KeyD,
                        ButtonAction::SameShape
                    ),
                    game_button(
                        "Sound (F)",
                        font.clone(),
                        KeyCode::KeyF,
                        ButtonAction::SameSound
                    ),
                ],
            ),
        ],
    ));
}
