use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    config,
    state::{AppState, OnGameScreen},
};

use self::button::{GameButtonBundle, Shortcut};

use super::{
    core::{cue::CueEngine, round::Round, score::Score, state::GameState},
    settings::GameSettings,
};

pub mod button;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_systems(Update, debug_ui.run_if(in_state(AppState::Game)));
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<GameSettings>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            if settings.position {
                parent
                    .spawn(GameButtonBundle {
                        button: ButtonBundle {
                            style: Style {
                                left: Val::Px(-100.0),
                                top: Val::Px(230.0),
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: config::BUTTON_BORDER_COLOR.into(),
                            background_color: config::NORMAL_BUTTON.into(),
                            ..default()
                        },
                        shortcut: Shortcut(KeyCode::KeyA),
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Position (A)",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }

            if settings.sound {
                parent
                    .spawn(GameButtonBundle {
                        button: ButtonBundle {
                            style: Style {
                                left: Val::Px(0.0),
                                top: Val::Px(230.0),
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: config::BUTTON_BORDER_COLOR.into(),
                            background_color: config::NORMAL_BUTTON.into(),
                            ..default()
                        },
                        shortcut: Shortcut(KeyCode::KeyS),
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Sound (S)",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }

            if settings.color {
                parent
                    .spawn(GameButtonBundle {
                        button: ButtonBundle {
                            style: Style {
                                left: Val::Px(100.0),
                                top: Val::Px(230.0),
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(3.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: config::BUTTON_BORDER_COLOR.into(),
                            background_color: config::NORMAL_BUTTON.into(),
                            ..default()
                        },
                        shortcut: Shortcut(KeyCode::KeyD),
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Color (D)",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }
        });
}

/// Debug interface.
pub fn debug_ui(
    mut egui_context: EguiContexts,
    mut query: Query<(&CueEngine, &Round, &Score, &mut GameState)>,
) {
    if let Ok((engine, round, score, mut state)) = query.get_single_mut() {
        egui::Window::new("debug").show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("N-back: {}", engine.n()));
            ui.label(format!("Correct: {}", score.correct()));
            ui.label(format!("Wrong: {}", score.wrong()));
            ui.label(format!("Answer: {:?}", round.answer));
            ui.label(format!("Score: {}", score.f1_score_percent()));

            if ui.button("Play").clicked() {
                *state = GameState::Playing;
            }

            if ui.button("Pause").clicked() {
                *state = GameState::Paused;
            }
        });
    }
}
