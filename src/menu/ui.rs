use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    game::{score::LatestGameScores, settings::GameSettings, ui::button},
    palette,
    state::{AppState, OnMenuScreen},
};

use super::{
    button::{DecreaseNButton, IncreaseNButton, PlayButton},
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
                    font_size: 64.0,
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
                                    width: Val::Px(64.0),
                                    height: Val::Px(64.0),
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
                                    font_size: 20.0,
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
                                    width: Val::Px(64.0),
                                    height: Val::Px(64.0),
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
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
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
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}

/// Game menu.
pub fn egui_ui(
    mut egui_context: EguiContexts,
    mut settings: ResMut<GameSettings>,
    scores: ResMut<LatestGameScores>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    egui::SidePanel::left("settings")
        .resizable(false)
        .exact_width(250.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Dual-N-Back");
            ui.add(egui::Slider::new(&mut settings.rounds, 1..=50).text("Rounds"));
            ui.add(egui::Slider::new(&mut settings.round_time, 0.5..=4.0).text("Round Time"));
            ui.add(egui::Slider::new(&mut settings.n, 1..=7).text("N-back"));
            ui.add(egui::Checkbox::new(&mut settings.position, "Position"));
            ui.add(egui::Checkbox::new(&mut settings.color, "Color"));
            ui.add(egui::Checkbox::new(&mut settings.sound, "Sound"));

            if ui.button("Play").clicked() {
                app_state.set(AppState::Game);
            }
        });

    egui::SidePanel::right("status")
        .resizable(false)
        .exact_width(250.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.columns(4, |cols| {
                cols[0].label("N-Back");
                cols[1].label("Rounds");
                cols[2].label("Time");
                cols[3].label("Score");
            });

            for score in scores.0.iter() {
                ui.columns(4, |cols| {
                    cols[0].label(format!("{}", score.n));
                    cols[1].label(format!("{}", score.total_rounds));
                    cols[2].label(format!(
                        "{:.2}s",
                        score.total_rounds as f32 * score.round_duration
                    ));
                    cols[3].label(format!("{}%", score.f1_score_percent));
                });
            }
        });
}
