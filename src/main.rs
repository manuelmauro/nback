use bevy::{camera::ScalingMode, prelude::*, ui::UiScale, window::WindowResolution};
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::AudioPlugin;
#[cfg(feature = "debug")]
use nback::debug::DebugPlugin;
use nback::{
    asset::AudioAssets,
    config,
    game::GamePlugin,
    menu::MenuPlugin,
    palette,
    persistence::{Database, PersistencePlugin},
    splash::SplashPlugin,
    state::AppState,
};

fn main() {
    let mut app = App::new();

    // Open (or create) the database and load persisted state.
    let db = Database::open();
    let settings = db.load_settings();
    let scores = db.load_scores();

    app.add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(720, 1280),
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
        .insert_resource(settings)
        .insert_resource(scores)
        .insert_resource(db)
        .add_plugins((
            AudioPlugin,
            PersistencePlugin,
            SplashPlugin,
            MenuPlugin,
            GamePlugin,
        ));

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.add_systems(Startup, setup)
        .add_systems(Update, (fit_ui_scale, log_transitions))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: config::WORLD_WIDTH,
                min_height: config::WORLD_WIDTH,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// Keeps the UI scale in sync with the window size so that all `Val::Px` values
/// (authored for the 720×1280 reference resolution) scale uniformly.
fn fit_ui_scale(mut ui_scale: ResMut<UiScale>, window: Single<&Window>) {
    let scale = (window.width() / config::REF_WIDTH).min(window.height() / config::REF_HEIGHT);

    if ui_scale.0 != scale {
        ui_scale.0 = scale;
    }
}

fn log_transitions(mut transitions: MessageReader<StateTransitionEvent<AppState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.exited, transition.entered
        );
    }
}
