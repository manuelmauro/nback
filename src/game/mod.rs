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

pub mod score;
pub mod session;
pub mod settings;
pub mod tile;
pub mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SessionPlugin, TilePlugin, UiPlugin, GameButtonPlugin))
            .add_systems(OnEnter(AppState::Game), setup_game);
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
