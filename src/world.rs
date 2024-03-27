use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    config::{TILE_SIZE, TILE_SPACING},
    gui::GuiPlugin,
    nback::NBack,
    state::{despawn_screen, GameState, OnGameScreen},
    tile::TilePosition,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GuiPlugin)
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                Update,
                (timer_system, answer_system, cue_system.after(answer_system))
                    .run_if(in_state(GameState::Game)),
            )
            .insert_resource(NBack::default())
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

    let edge = (TILE_SIZE * 3.0) + (TILE_SPACING * 4.0);
    let bounds = Vec2::new(edge, edge);
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

    let tile = TilePosition::None;
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                material: materials.add(Color::rgb(1.0, 0.56, 0.0)),
                transform: Transform::from_translation((&tile).into()),
                ..default()
            },
            OnGameScreen,
        ))
        .insert(tile)
        .insert(CueTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

#[derive(Component, Deref, DerefMut)]
struct CueTimer(Timer);

/// Tick all the `Timer` components on entities within the scene using bevy's
/// `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut query: Query<&mut CueTimer>) {
    for mut timer in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            info!("tick!")
        }
    }
}

/// Render cues.
fn cue_system(
    mut game: ResMut<NBack>,
    mut board_query: Query<(&TilePosition, &mut Transform, &mut Mesh2dHandle, &CueTimer)>,
) {
    if let Ok((_, mut transform, mut sprite, timer)) = board_query.get_single_mut() {
        if timer.just_finished() {
            if let Some((new_cell, new_pigment)) = game.next() {
                info!("cue: {:?}", new_cell);
                transform.translation = (&new_cell).into();

                // TODO sprite.material = (&new_pigment).into();
            }
        }
    }
}

/// Record answers.
fn answer_system(
    mut game: ResMut<NBack>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&CueTimer>,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        game.answer.w();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        game.answer.a();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        game.answer.s();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        game.answer.d();
    }

    if let Ok(timer) = query.get_single_mut() {
        if timer.just_finished() {
            game.check_answer();
            game.answer.reset();
            info!("reset answer");
        }
    }
}
