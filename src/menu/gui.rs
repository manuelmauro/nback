use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    game::{score::GameScore, settings::GameSettings},
    state::GameState,
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, menu_ui.run_if(in_state(GameState::Menu)));
    }
}

/// Game menu.
pub fn menu_ui(
    mut egui_context: EguiContexts,
    mut settings: ResMut<GameSettings>,
    score: ResMut<GameScore>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    egui::SidePanel::left("settings").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Dual-N-Back");
        ui.add(egui::Slider::new(&mut settings.rounds, 1..=50).text("Rounds"));
        ui.add(egui::Slider::new(&mut settings.round_time, 0.5..=4.0).text("Round Time"));
        ui.add(egui::Slider::new(&mut settings.n, 1..=7).text("N-back"));

        if ui.button("Play").clicked() {
            game_state.set(GameState::Game);
        }
    });

    egui::SidePanel::right("status").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("N-back: {}", score.n));
        ui.label(format!("Correct: {}", score.correct));
        ui.label(format!("Wrong: {}", score.wrong));
        ui.label(format!("Score: {}/100", (score.f1_score * 100.0) as usize));
    });
}
