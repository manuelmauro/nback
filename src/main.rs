use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use nback::{game::GamePlugin, menu::MenuPlugin, splash::SplashPlugin, state::AppState};

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(SplashPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, log_transitions)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<AppState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}
