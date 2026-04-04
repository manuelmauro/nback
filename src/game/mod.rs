use std::time::Duration;

use bevy::prelude::*;

use crate::{config, state::AppState};

use self::{
    core::{
        cue::{CueEngine, CueTimer},
        round::{Answer, Round},
        score::Score,
        state::GameState,
    },
    input::InputPlugin,
    score::{GameScore, LatestGameScores},
    settings::GameSettings,
    tile::{Tile, TilePlugin, color::TileColor, position::TilePosition, sound::TileSound},
    ui::{UiPlugin, button::GameButtonPlugin},
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
        app.insert_resource(Answer::default())
            .add_plugins((UiPlugin, TilePlugin, InputPlugin, GameButtonPlugin))
            .add_message::<EndOfRoundEvent>()
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(
                PreUpdate,
                end_of_game_system.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (timer_system, end_of_round_system).run_if(in_state(AppState::Game)),
            );
    }
}

fn setup(mut commands: Commands, settings: Res<GameSettings>) {
    // Add walls
    let edge = (config::TILE_SIZE * 3.0) + (config::TILE_SPACING * 4.0);
    let bounds = Vec2::new(edge, edge);

    let wall_marker = DespawnOnExit(AppState::Game);

    // left
    commands.spawn((
        Sprite {
            color: config::WALL_COLOR,
            custom_size: Some(Vec2::new(
                config::WALL_THICKNESS,
                bounds.y + config::WALL_THICKNESS,
            )),
            ..default()
        },
        Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        wall_marker.clone(),
    ));
    // right
    commands.spawn((
        Sprite {
            color: config::WALL_COLOR,
            custom_size: Some(Vec2::new(
                config::WALL_THICKNESS,
                bounds.y + config::WALL_THICKNESS,
            )),
            ..default()
        },
        Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        wall_marker.clone(),
    ));
    // bottom
    commands.spawn((
        Sprite {
            color: config::WALL_COLOR,
            custom_size: Some(Vec2::new(
                bounds.x + config::WALL_THICKNESS,
                config::WALL_THICKNESS,
            )),
            ..default()
        },
        Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        wall_marker.clone(),
    ));
    // top
    commands.spawn((
        Sprite {
            color: config::WALL_COLOR,
            custom_size: Some(Vec2::new(
                bounds.x + config::WALL_THICKNESS,
                config::WALL_THICKNESS,
            )),
            ..default()
        },
        Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        wall_marker.clone(),
    ));

    // start with a cue
    let mut timer = CueTimer::with_duration(settings.round_time);
    timer.tick(Duration::from_millis(
        ((settings.round_time * 1000.0) as u64) - 1,
    ));

    // game tile + dual-n-back state
    let (tile, sprite, tile_transform) = Tile::bundle();
    commands.spawn((
        Name::new("tile"),
        tile,
        sprite,
        tile_transform,
        CueEngine::with(
            settings.n,
            settings.position,
            settings.color,
            settings.sound,
        ),
        GameState::default(),
        timer,
        Round::with_total(settings.rounds),
        Score::default(),
        wall_marker,
    ));
}

/// Tick all the `CueTimer` components on entities within the scene using bevy's
/// `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut tile: Single<(&mut CueTimer, &GameState)>) {
    let (timer, state) = &mut *tile;
    if **state == GameState::Playing {
        timer.tick(time.delta());
        if timer.just_finished() {
            info!("tick!")
        }
    }
}

#[derive(bevy::ecs::message::Message)]
pub struct EndOfRoundEvent {
    pub round: usize,
}

fn end_of_round_system(
    mut events: MessageWriter<EndOfRoundEvent>,
    mut answer: ResMut<Answer>,
    mut tile: Single<(
        &mut CueEngine,
        &mut Round,
        &mut Score,
        &mut TilePosition,
        &mut TileColor,
        &mut TileSound,
        &CueTimer,
    )>,
) {
    let (engine, round, score, position, color, sound, timer) = &mut *tile;

    if !timer.just_finished() {
        return;
    }

    if let Some(positions) = &engine.positions {
        if answer.position {
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
        if answer.color {
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
        if answer.sound {
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

    answer.reset();

    let (new_position, new_color, new_sound) = engine.new_cue();
    if let Some(new_position) = new_position {
        **position = new_position;
    }
    if let Some(new_color) = new_color {
        **color = new_color;
    }
    if let Some(new_sound) = new_sound {
        **sound = new_sound;
    }

    events.write(EndOfRoundEvent {
        round: round.current,
    });

    round.current += 1;
}

fn end_of_game_system(
    mut settings: ResMut<GameSettings>,
    mut scores: ResMut<LatestGameScores>,
    mut app_state: ResMut<NextState<AppState>>,
    tile: Single<(&CueEngine, &Round, &CueTimer, &Score)>,
) {
    let (engine, round, timer, score) = *tile;

    if !round.is_last() {
        return;
    }

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
