use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::nback::NBack;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, debug_ui);
    }
}

/// User interface.
pub fn debug_ui(mut egui_context: EguiContexts, mut game: ResMut<NBack>) {
    egui::Window::new("debug")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("n back: {}", game.positions.n_back()));
            ui.label(format!("correct: {}", game.score.correct()));
            ui.label(format!("wrong: {}", game.score.wrong()));
            ui.label(format!("F1 score: {}", game.score.f1_score()));
            ui.label(format!("{:?}", game.answer));

            if ui.button("Restart").clicked() {
                game.restart()
            }
        });
}
