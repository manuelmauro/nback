use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    config,
    game::{nback::NBack, tile::TilePosition},
    state::{despawn_screen, GameState, OnMenuScreen},
};

use self::gui::GuiPlugin;

pub mod gui;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GuiPlugin)
            .add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>)
            .insert_resource(NBack::default());
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

    let edge = (config::TILE_SIZE * 3.0) + (config::TILE_SPACING * 4.0);
    let bounds = Vec2::new(edge, edge);
    // left
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..default()
            },
            ..default()
        },
        OnMenuScreen,
    ));
    // right
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..default()
            },
            ..default()
        },
        OnMenuScreen,
    ));
    // bottom
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..default()
            },
            ..default()
        },
        OnMenuScreen,
    ));
    // top
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..default()
            },
            ..default()
        },
        OnMenuScreen,
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
                mesh: Mesh2dHandle(
                    meshes.add(Rectangle::new(config::TILE_SIZE, config::TILE_SIZE)),
                ),
                material: materials.add(color),
                transform: Transform::from_translation((&tile).into()),
                ..default()
            },
            OnMenuScreen,
        ));
    }
}
