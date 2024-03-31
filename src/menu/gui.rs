use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    game::{
        score::{GameScore, LatestGameScores},
        settings::GameSettings,
    },
    state::AppState,
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, menu_ui.run_if(in_state(AppState::Menu)));
    }
}

/// Game menu.
pub fn menu_ui(
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
                    cols[3].label(format!("{}%", (score.f1_score * 100.0) as usize));
                });
            }
        });
}
