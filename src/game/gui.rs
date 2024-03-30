use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::state::GameState;

use super::core::DualNBack;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_ui.run_if(in_state(GameState::Game)));
    }
}

/// Debug interface.
pub fn debug_ui(mut egui_context: EguiContexts, mut query: Query<&mut DualNBack>) {
    if let Ok(mut game) = query.get_single_mut() {
        egui::Window::new("debug").show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("N-back: {}", game.n_back()));
            ui.label(format!("Correct: {}", game.score.correct()));
            ui.label(format!("Wrong: {}", game.score.wrong()));
            ui.label(format!("Answer: {:?}", game.answer));
            ui.label(format!(
                "Score: {}",
                (game.score.f1_score() * 100.0) as usize
            ));

            if ui.button("Pause").clicked() {
                game.paused = !game.paused;
            }
        });
    }
}
