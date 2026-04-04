use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::{
    game::{
        core::{
            cue::{CueEngine, CueTimer},
            round::{Answer, Round},
            score::Score,
            state::GameState,
        },
        settings::GameSettings,
    },
    state::AppState,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, debug_ui_system);
    }
}

fn debug_ui_system(
    mut contexts: EguiContexts,
    app_state: Res<State<AppState>>,
    settings: Option<Res<GameSettings>>,
    answer: Option<Res<Answer>>,
    query: Query<(&CueEngine, &GameState, &CueTimer, &Round, &Score)>,
) {
    let Some(ctx) = contexts.ctx_mut().ok() else {
        return;
    };
    egui::Window::new("🐛 Debug")
        .resizable(true)
        .vscroll(true)
        .show(ctx, |ui| {
        ui.label(format!("App State: {:?}", app_state.get()));
        ui.separator();

        if let Some(settings) = &settings {
            egui::CollapsingHeader::new("Settings")
                .default_open(true)
                .show(ui, |ui| {
                    egui::Grid::new("settings_grid").show(ui, |ui| {
                        ui.label("N-back level");
                        ui.label(format!("{}", settings.n));
                        ui.end_row();

                        ui.label("Rounds");
                        ui.label(format!("{}", settings.rounds));
                        ui.end_row();

                        ui.label("Round time");
                        ui.label(format!("{:.1}s", settings.round_time));
                        ui.end_row();

                        ui.label("Position cues");
                        ui.label(if settings.position { "✅" } else { "❌" });
                        ui.end_row();

                        ui.label("Color cues");
                        ui.label(if settings.color { "✅" } else { "❌" });
                        ui.end_row();

                        ui.label("Sound cues");
                        ui.label(if settings.sound { "✅" } else { "❌" });
                        ui.end_row();
                    });
                });
        }

        if let Ok((engine, game_state, timer, round, score)) = query.single() {
            ui.separator();

            egui::CollapsingHeader::new("Game")
                .default_open(true)
                .show(ui, |ui| {
                    egui::Grid::new("game_grid").show(ui, |ui| {
                        ui.label("State");
                        ui.label(format!("{:?}", game_state));
                        ui.end_row();

                        ui.label("Round");
                        ui.label(format!("{} / {}", round.current, round.total));
                        ui.end_row();

                        ui.label("Timer");
                        let remaining = timer.0.duration().as_secs_f32()
                            - timer.0.elapsed().as_secs_f32();
                        ui.add(
                            egui::ProgressBar::new(
                                timer.0.elapsed().as_secs_f32()
                                    / timer.0.duration().as_secs_f32(),
                            )
                            .text(format!("{:.1}s", remaining)),
                        );
                        ui.end_row();
                    });
                });

            ui.separator();

            egui::CollapsingHeader::new("Cues")
                .default_open(true)
                .show(ui, |ui| {
                    egui::Grid::new("cue_grid").show(ui, |ui| {
                        ui.label("N-back");
                        ui.label(format!("{}", engine.n()));
                        ui.end_row();

                        ui.label("Position match");
                        ui.label(match &engine.positions {
                            Some(p) => if p.is_match() { "🟢 YES" } else { "⚫ no" },
                            None => "—",
                        });
                        ui.end_row();

                        ui.label("Color match");
                        ui.label(match &engine.colors {
                            Some(c) => if c.is_match() { "🟢 YES" } else { "⚫ no" },
                            None => "—",
                        });
                        ui.end_row();

                        ui.label("Sound match");
                        ui.label(match &engine.sounds {
                            Some(s) => if s.is_match() { "🟢 YES" } else { "⚫ no" },
                            None => "—",
                        });
                        ui.end_row();
                    });
                });

            ui.separator();

            egui::CollapsingHeader::new("Score")
                .default_open(true)
                .show(ui, |ui| {
                    egui::Grid::new("score_grid").show(ui, |ui| {
                        ui.label("Correct");
                        ui.label(format!("{}", score.correct()));
                        ui.end_row();

                        ui.label("Wrong");
                        ui.label(format!("{}", score.wrong()));
                        ui.end_row();

                        ui.label("F1 Score");
                        ui.label(format!("{}%", score.f1_score_percent()));
                        ui.end_row();
                    });
                });

            if let Some(answer) = &answer {
                ui.separator();

                egui::CollapsingHeader::new("Current Answer")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("answer_grid").show(ui, |ui| {
                            ui.label("Position");
                            ui.label(if answer.position { "🟢" } else { "⚫" });
                            ui.end_row();

                            ui.label("Color");
                            ui.label(if answer.color { "🟢" } else { "⚫" });
                            ui.end_row();

                            ui.label("Sound");
                            ui.label(if answer.sound { "🟢" } else { "⚫" });
                            ui.end_row();
                        });
                    });
            }
        }
    });
}
