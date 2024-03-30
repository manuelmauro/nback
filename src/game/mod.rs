use bevy::prelude::*;

use crate::{
    config,
    state::{despawn_screen, GameState, OnGameScreen},
};

use self::{
    button::{GameButtonBundle, GameButtonPlugin, Shortcut},
    core::DualNBack,
    gui::GuiPlugin,
    score::GameScore,
    settings::GameSettings,
    tile::{tile_system, TileBundle},
};

pub mod button;
pub mod core;
pub mod gui;
pub mod score;
pub mod settings;
pub mod tile;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GuiPlugin)
            .add_plugins(GameButtonPlugin)
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                Update,
                (
                    timer_system,
                    answer_system,
                    tile_system.after(answer_system),
                    endgame_system,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn setup(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    // Add walls
    let edge = (config::TILE_SIZE * 3.0) + (config::TILE_SPACING * 4.0);
    let bounds = Vec2::new(edge, edge);
    // left
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite {
                color: config::WALL_COLOR,
                custom_size: Some(Vec2::new(
                    config::WALL_THICKNESS,
                    bounds.y + config::WALL_THICKNESS,
                )),
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
                color: config::WALL_COLOR,
                custom_size: Some(Vec2::new(
                    config::WALL_THICKNESS,
                    bounds.y + config::WALL_THICKNESS,
                )),
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
                color: config::WALL_COLOR,
                custom_size: Some(Vec2::new(
                    bounds.x + config::WALL_THICKNESS,
                    config::WALL_THICKNESS,
                )),
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
                color: config::WALL_COLOR,
                custom_size: Some(Vec2::new(
                    bounds.x + config::WALL_THICKNESS,
                    config::WALL_THICKNESS,
                )),
                ..default()
            },
            ..default()
        },
        OnGameScreen,
    ));

    let tile = Name::new("tile");
    let mut animation = AnimationClip::default();

    animation.add_curve_to_path(
        EntityPath {
            parts: vec![tile.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.25],
            keyframes: Keyframes::Scale(vec![Vec3::splat(0.8), Vec3::splat(1.0)]),
            interpolation: Interpolation::Linear,
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(animations.add(animation));

    // game
    commands.spawn((
        TileBundle {
            name: tile,
            animation: player,
            timer: CueTimer(Timer::from_seconds(
                settings.round_time,
                TimerMode::Repeating,
            )),
            ..default()
        },
        DualNBack::default(),
        OnGameScreen,
    ));

    // buttons
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(GameButtonBundle {
                    button: ButtonBundle {
                        style: Style {
                            left: Val::Px(-100.0),
                            top: Val::Px(230.0),
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(3.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: config::BUTTON_BORDER_COLOR.into(),
                        background_color: config::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    shortcut: Shortcut(KeyCode::KeyA),
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Position (A)",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn(GameButtonBundle {
                    button: ButtonBundle {
                        style: Style {
                            left: Val::Px(100.0),
                            top: Val::Px(230.0),
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(3.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: config::BUTTON_BORDER_COLOR.into(),
                        background_color: config::NORMAL_BUTTON.into(),
                        ..default()
                    },
                    shortcut: Shortcut(KeyCode::KeyD),
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Color (D)",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

#[derive(Component, Deref, DerefMut)]
pub struct CueTimer(Timer);

/// Tick all the `CueTimer` components on entities within the scene using bevy's
/// `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut query: Query<(&mut CueTimer, &DualNBack)>) {
    if let Ok((mut timer, game)) = query.get_single_mut() {
        if !game.paused {
            if timer.tick(time.delta()).just_finished() {
                info!("tick!")
            }
        }
    }
}

/// Record answers.
fn answer_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut DualNBack, &CueTimer)>,
) {
    if let Ok((mut game, timer)) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            game.answer.same_position();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            game.answer.same_color();
        }

        if timer.just_finished() {
            game.check_answer();
            game.answer.reset();
            info!("reset answer");
        }
    }
}

fn endgame_system(
    mut score: ResMut<GameScore>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&DualNBack>,
) {
    if let Ok(game) = query.get_single() {
        if game.is_over() {
            score.n = game.n;
            score.correct = game.score.correct();
            score.wrong = game.score.wrong();
            score.f1_score = game.score.f1_score();
            game_state.set(GameState::Menu);
        }
    }
}
