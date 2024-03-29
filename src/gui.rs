use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::{nback::NBack, state::GameState};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, menu_ui.run_if(in_state(GameState::Menu)))
            .add_systems(Update, debug_ui.run_if(in_state(GameState::Game)));
    }
}

/// Game menu.
pub fn menu_ui(
    mut egui_context: EguiContexts,
    mut game: ResMut<NBack>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    egui::SidePanel::left("settings").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Dual-N-Back");
        ui.add(egui::Slider::new(&mut game.rounds, 1..=50).text("Rounds"));
        ui.add(egui::Slider::new(&mut game.round_time, 0.5..=4.0).text("Round Time"));
        ui.add(egui::Slider::new(&mut game.n, 1..=7).text("N-back"));

        if ui.button("Play").clicked() {
            game.restart();
            game_state.set(GameState::Game);
        }
    });

    egui::SidePanel::right("status").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("N-back: {}", game.n_back()));
        ui.label(format!("Correct: {}", game.score.correct()));
        ui.label(format!("Wrong: {}", game.score.wrong()));
        ui.label(format!(
            "Score: {}",
            (game.score.f1_score() * 100.0) as usize
        ));
    });
}

/// Debug interface.
pub fn debug_ui(mut egui_context: EguiContexts, game: ResMut<NBack>) {
    egui::Window::new("debug").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("N-back: {}", game.n_back()));
        ui.label(format!("Correct: {}", game.score.correct()));
        ui.label(format!("Wrong: {}", game.score.wrong()));
        ui.label(format!("Answer: {:?}", game.answer));
        ui.label(format!(
            "Score: {}",
            (game.score.f1_score() * 100.0) as usize
        ));
    });
}
