use bevy::prelude::*;

use crate::{
    config::{TILE_SIZE, TILE_SPACING},
    nback::NBack,
    state::{despawn_screen, GameState, OnGameScreen},
    tile::TilePosition,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                Update,
                (
                    timer_system,
                    answer_system,
                    cue_system.after(answer_system),
                    exit_game,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn setup(mut commands: Commands, game: Res<NBack>) {
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
                ..default()
            },
            ..default()
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
                ..default()
            },
            ..default()
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
                ..default()
            },
            ..default()
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
                ..default()
            },
            ..default()
        },
        OnGameScreen,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation((&TilePosition::None).into()),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.56, 0.0),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        TilePosition::None,
        CueTimer(Timer::from_seconds(game.round_time, TimerMode::Repeating)),
        OnGameScreen,
    ));
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
    mut query: Query<(&TilePosition, &mut Transform, &mut Sprite, &CueTimer)>,
) {
    if let Ok((_, mut transform, mut sprite, timer)) = query.get_single_mut() {
        if timer.just_finished() {
            if let Some((new_cell, new_pigment)) = game.next() {
                info!("cue: {:?}", new_cell);
                transform.translation = (&new_cell).into();
                sprite.color = (&new_pigment).into();
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

fn exit_game(game: ResMut<NBack>, mut game_state: ResMut<NextState<GameState>>) {
    if game.is_over() {
        game_state.set(GameState::Menu);
    }
}
