use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::state::AppState;

use super::core::{score::Score, state::GameState, DualNBack};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_ui.run_if(in_state(AppState::Game)));
    }
}

/// Debug interface.
pub fn debug_ui(
    mut egui_context: EguiContexts,
    mut query: Query<(&DualNBack, &Score, &mut GameState)>,
) {
    if let Ok((game, score, mut state)) = query.get_single_mut() {
        egui::Window::new("debug").show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("N-back: {}", game.n_back()));
            ui.label(format!("Correct: {}", score.correct()));
            ui.label(format!("Wrong: {}", score.wrong()));
            ui.label(format!("Answer: {:?}", game.answer));
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
