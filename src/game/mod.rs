use bevy::prelude::*;

use crate::{config, state::AppState};

use self::{
    session::{
        Session, SessionPlugin, answer::Answer, cue::CueTimer, engine::CueEngine, round::Round,
        score::Score,
    },
    settings::GameSettings,
    tile::{Tile, TilePlugin},
    ui::{UiPlugin, button::GameButtonPlugin},
};

pub mod phase;
pub mod score;
pub mod session;
pub mod settings;
pub mod tile;
pub mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<phase::GamePhase>()
            .add_plugins((SessionPlugin, TilePlugin, UiPlugin, GameButtonPlugin))
            .add_systems(OnEnter(AppState::Game), setup_game)
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(phase::GamePhase::Paused), spawn_pause_overlay)
            .add_systems(OnEnter(phase::GamePhase::Playing), despawn_pause_overlay);
    }
}

/// Spawn the arena, the tile with its first cue, and the session entity.
fn setup_game(mut commands: Commands, settings: Res<GameSettings>) {
    let edge = (config::TILE_SIZE * 3.0) + (config::TILE_SPACING * 4.0);
    let bounds = Vec2::new(edge, edge);
    let marker = DespawnOnExit(AppState::Game);

    // Walls: left, right, bottom, top
    for (x, y, w, h) in [
        (
            -bounds.x / 2.0,
            0.0,
            config::WALL_THICKNESS,
            bounds.y + config::WALL_THICKNESS,
        ),
        (
            bounds.x / 2.0,
            0.0,
            config::WALL_THICKNESS,
            bounds.y + config::WALL_THICKNESS,
        ),
        (
            0.0,
            -bounds.y / 2.0,
            bounds.x + config::WALL_THICKNESS,
            config::WALL_THICKNESS,
        ),
        (
            0.0,
            bounds.y / 2.0,
            bounds.x + config::WALL_THICKNESS,
            config::WALL_THICKNESS,
        ),
    ] {
        commands.spawn((
            Sprite {
                color: config::WALL_COLOR,
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            marker.clone(),
        ));
    }

    // Create engine and generate the first cue up-front so the player
    // sees a real cue from the start (no phantom round).
    let mut engine = CueEngine::new(
        settings.n,
        settings.position,
        settings.color,
        settings.sound,
    );
    let (first_pos, first_color, first_sound) = engine.new_cue();

    let tile_pos = first_pos.unwrap_or_default();
    let tile_color = first_color.unwrap_or_default();
    let tile_sound = first_sound.unwrap_or_default();

    // Spawn tile with the first cue already applied.
    // Change-detection will fire on the first frame, playing the sound
    // and triggering the pop animation.
    commands.spawn((
        Name::new("tile"),
        Tile,
        Sprite {
            color: (&tile_color).into(),
            custom_size: Some(Vec2::new(config::TILE_SIZE, config::TILE_SIZE)),
            ..default()
        },
        Transform::from_translation((&tile_pos).into()),
        tile_pos,
        tile_color,
        tile_sound,
        marker.clone(),
    ));

    // Spawn session. The engine already consumed the first cue, and
    // current starts at 1 (round 0 is the cue we just displayed).
    // The timer starts fresh — the player gets the full duration.
    commands.spawn((
        Name::new("session"),
        Session,
        engine,
        CueTimer::with_duration(settings.round_time),
        Round {
            current: 1,
            total: settings.rounds,
        },
        Score::default(),
        Answer::default(),
        marker,
    ));
}

/// Toggle between Playing and Paused on Escape.
fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current: Res<State<phase::GamePhase>>,
    mut next: ResMut<NextState<phase::GamePhase>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next.set(match current.get() {
            phase::GamePhase::Playing => phase::GamePhase::Paused,
            phase::GamePhase::Paused => phase::GamePhase::Playing,
        });
    }
}

#[derive(Component)]
struct PauseOverlay;

fn spawn_pause_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");

    commands.spawn((
        PauseOverlay,
        DespawnOnExit(AppState::Game),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        // Render on top of the game UI
        GlobalZIndex(10),
        children![(
            Text::new("PAUSED"),
            TextFont {
                font,
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    ));
}

fn despawn_pause_overlay(mut commands: Commands, query: Query<Entity, With<PauseOverlay>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
