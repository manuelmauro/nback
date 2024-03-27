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
    // Add walls
    let wall_color = Color::rgb(1.0, 1.0, 1.0);
    let wall_thickness = 4.0;
    let bounds = Vec2::new(260.0, 260.0);
    // left
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));
    // right
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));
    // bottom
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));
    // top
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..Default::default()
            },
            ..Default::default()
        },
        OnGameScreen,
    ));

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
