use bevy::prelude::*;
use nback::{splash::SplashPlugin, state::GameState, world::WorldPlugin};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(SplashPlugin)
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            bevy::window::close_on_esc.run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, log_transitions)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<GameState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}
