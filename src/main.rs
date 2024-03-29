use bevy::{prelude::*, winit::WinitWindows};
use nback::{game::GamePlugin, menu::MenuPlugin, splash::SplashPlugin, state::GameState};
use winit::window::Icon;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(SplashPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, set_window_icon)
        .add_systems(Update, log_transitions)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<GameState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}
