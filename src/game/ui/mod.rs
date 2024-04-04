use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::state::AppState;

use super::core::{cue::CueEngine, round::Round, score::Score, state::GameState};

pub mod button;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_ui.run_if(in_state(AppState::Game)));
    }
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
