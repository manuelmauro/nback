use bevy::{prelude::*, window::WindowResolution};
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::AudioPlugin;
#[cfg(feature = "debug")]
use nback::debug::DebugPlugin;
use nback::{
    asset::AudioAssets, game::GamePlugin, menu::MenuPlugin, palette, splash::SplashPlugin,
    state::AppState,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(720, 1280).with_scale_factor_override(1.0),
                title: "Dual-N-Back".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(palette::SLATE_800))
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::Menu)
                .load_collection::<AudioAssets>(),
        )
        .add_plugins(AudioPlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.add_plugins(SplashPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, log_transitions)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn log_transitions(mut transitions: MessageReader<StateTransitionEvent<AppState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.exited, transition.entered
        );
    }
}
