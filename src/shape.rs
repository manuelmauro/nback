use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{config::SIZE, despawn_screen, tile::Tile, GameState};

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
    let cells = [
        Tile::TopLeft,
        Tile::TopCenter,
        Tile::TopRight,
        Tile::CenterLeft,
        Tile::Center,
        Tile::CenterRight,
        Tile::BottomLeft,
        Tile::BottomCenter,
        Tile::BottomRight,
    ];

    let num_shapes = cells.len();

    for (i, tile) in cells.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(SIZE, SIZE))),
                material: materials.add(color),
                transform: Transform::from_translation((&tile).into()),
                ..default()
            },
            OnGameScreen,
        ));
    }
}
