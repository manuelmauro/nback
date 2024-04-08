use std::time::Duration;

use bevy::prelude::*;

use crate::{
    config,
    state::{despawn_screen, AppState, OnGameScreen},
};

use self::{
    core::{
        cue::{CueEngine, CueTimer},
        round::Round,
        score::Score,
        state::GameState,
        DualNBackBundle,
    },
    input::InputPlugin,
    score::{GameScore, LatestGameScores},
    settings::GameSettings,
    tile::{color::TileColor, position::TilePosition, sound::TileSound, TileBundle, TilePlugin},
    ui::button::GameButtonPlugin,
    ui::UiPlugin,
};

pub mod core;
pub mod input;
pub mod score;
pub mod settings;
pub mod tile;
pub mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin)
            .add_plugins(TilePlugin)
            .add_plugins(InputPlugin)
            .add_plugins(GameButtonPlugin)
            .add_event::<EndOfRoundEvent>()
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(
                PreUpdate,
                end_of_game_system.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (timer_system, end_of_round_system).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn setup(
    mut commands: Commands,
    settings: Res<GameSettings>,
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

    // start with a cue
    let mut timer = CueTimer::with_duration(settings.round_time);
    timer.tick(Duration::from_millis(
        ((settings.round_time * 1000.0) as u64) - 1,
    ));

    // game
    commands.spawn((
        TileBundle {
            name: tile,
            animation: player,
            ..default()
        },
        DualNBackBundle {
            engine: CueEngine::with(
                settings.n,
                settings.position,
                settings.color,
                settings.sound,
            ),
            round: Round::with_total(settings.rounds),
            timer,
            ..default()
        },
        OnGameScreen,
    ));
}

/// Tick all the `CueTimer` components on entities within the scene using bevy's
/// `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut query: Query<(&mut CueTimer, &GameState)>) {
    if let Ok((mut timer, state)) = query.get_single_mut() {
        if *state == GameState::Playing {
            timer.tick(time.delta());
            if timer.just_finished() {
                info!("tick!")
            }
        }
    }
}

#[derive(Event)]
pub struct EndOfRoundEvent {
    pub round: usize,
}

fn end_of_round_system(
    mut events: EventWriter<EndOfRoundEvent>,
    mut query: Query<(
        &mut CueEngine,
        &mut Round,
        &mut Score,
        &mut TilePosition,
        &mut TileColor,
        &mut TileSound,
        &CueTimer,
    )>,
) {
    if let Ok((mut engine, mut round, mut score, mut position, mut color, mut sound, timer)) =
        query.get_single_mut()
    {
        if timer.just_finished() {
            if let Some(positions) = &engine.positions {
                if round.answer.position {
                    if positions.is_match() {
                        score.record_tp();
                    } else {
                        score.record_fp();
                    }
                } else if positions.is_match() {
                    score.record_fn();
                } else {
                    score.record_tn();
                }
            }

            if let Some(colors) = &engine.colors {
                if round.answer.color {
                    if colors.is_match() {
                        score.record_tp();
                    } else {
                        score.record_fp();
                    }
                } else if colors.is_match() {
                    score.record_fn();
                } else {
                    score.record_tn();
                }
            }

            if let Some(sounds) = &engine.sounds {
                if round.answer.sound {
                    if sounds.is_match() {
                        score.record_tp();
                    } else {
                        score.record_fp();
                    }
                } else if sounds.is_match() {
                    score.record_fn();
                } else {
                    score.record_tn();
                }
            }

            round.answer.reset();

            let (new_position, new_color, new_sound) = engine.new_cue();
            if let Some(new_position) = new_position {
                *position = new_position;
            }
            if let Some(new_color) = new_color {
                *color = new_color;
            }
            if let Some(new_sound) = new_sound {
                *sound = new_sound;
            }

            events.send(EndOfRoundEvent {
                round: round.current,
            });

            round.current += 1;
        }
    }
}

fn end_of_game_system(
    mut settings: ResMut<GameSettings>,
    mut scores: ResMut<LatestGameScores>,
    mut app_state: ResMut<NextState<AppState>>,
    query: Query<(&CueEngine, &Round, &CueTimer, &mut Score)>,
) {
    if let Ok((engine, round, timer, score)) = query.get_single() {
        if round.is_last() {
            scores.0.push(GameScore {
                n: engine.n(),
                total_rounds: round.total,
                round_duration: timer.0.duration().as_secs_f32(),
                correct: score.correct(),
                wrong: score.wrong(),
                f1_score_percent: score.f1_score_percent(),
            });

            if score.f1_score_percent() >= 80 {
                settings.n += 1;
                settings.set_rounds_from_n();
            } else if score.f1_score_percent() <= 50 {
                settings.n = settings.n.max(1);
                settings.set_rounds_from_n();
            }

            app_state.set(AppState::Menu);
        }
    }
}
