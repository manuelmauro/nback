use bevy::prelude::*;

use crate::{config, state::AppState};

use self::{
    session::SessionPlugin,
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
            .add_systems(OnEnter(AppState::Game), spawn_arena);
    }
}

/// Spawn the visual arena walls.
fn spawn_arena(mut commands: Commands) {
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

    // Game tile (visual only — session entity is spawned by SessionPlugin)
    let (tile, sprite, transform) = Tile::bundle();
    commands.spawn((Name::new("tile"), tile, sprite, transform, marker));
}
