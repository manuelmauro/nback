use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{config::TILE_SIZE, despawn_screen, tile::TilePosition, GameState};

pub struct GamePlugin;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tiles = [
        TilePosition::TopLeft,
        TilePosition::TopCenter,
        TilePosition::TopRight,
        TilePosition::CenterLeft,
        TilePosition::Center,
        TilePosition::CenterRight,
        TilePosition::BottomLeft,
        TilePosition::BottomCenter,
        TilePosition::BottomRight,
    ];

    let num_tiles = tiles.len();

    for (i, tile) in tiles.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_tiles as f32, 0.95, 0.7);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                material: materials.add(color),
                transform: Transform::from_translation((&tile).into()),
                ..default()
            },
            OnGameScreen,
        ));
    }
}
