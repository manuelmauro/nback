use bevy::prelude::*;

use crate::{
    game::{
        phase::GamePhase,
        score::{ScoreHistory, ScoreRecord},
        settings::GameSettings,
        tile::{color::TileColor, position::TilePosition, shape::TileShape, sound::TileSound},
    },
    state::AppState,
};

use self::{answer::Answer, cue::CueTimer, engine::CueEngine, round::Round, score::Score};

pub mod answer;
pub mod cue;
pub mod engine;
pub mod round;
pub mod score;

/// Marker for the game-session entity that holds all non-visual game state.
#[derive(Component)]
pub struct Session;

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EndOfRoundEvent>()
            .add_systems(
                PreUpdate,
                end_of_game_system.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (timer_system, end_of_round_system)
                    .chain()
                    .run_if(in_state(GamePhase::Playing)),
            );
    }
}

/// Tick the cue timer every frame.
fn timer_system(time: Res<Time>, mut session: Single<&mut CueTimer, With<Session>>) {
    session.tick(time.delta());
    if session.just_finished() {
        info!("tick!");
    }
}

#[derive(bevy::ecs::message::Message)]
pub struct EndOfRoundEvent {
    pub round: usize,
}

/// At the end of each round: evaluate the answer, generate new cues, advance.
fn end_of_round_system(
    mut events: MessageWriter<EndOfRoundEvent>,
    mut session: Single<
        (
            &mut CueEngine,
            &mut Round,
            &mut Score,
            &mut Answer,
            &CueTimer,
        ),
        With<Session>,
    >,
    mut tile: Single<(
        &mut TilePosition,
        &mut TileColor,
        &mut TileShape,
        &mut TileSound,
    )>,
) {
    let (engine, round, score, answer, timer) = &mut *session;
    let (position, color, shape, sound) = &mut *tile;

    if !timer.just_finished() {
        return;
    }

    // Evaluate each cue channel
    score.evaluate(&engine.positions, answer.position);
    score.evaluate(&engine.colors, answer.color);
    score.evaluate(&engine.shapes, answer.shape);
    score.evaluate(&engine.sounds, answer.sound);

    answer.reset();

    // Generate next cues
    let cue = engine.new_cue();
    if let Some(p) = cue.position {
        **position = p;
    }
    if let Some(c) = cue.color {
        **color = c;
    }
    if let Some(s) = cue.shape {
        **shape = s;
    }
    if let Some(s) = cue.sound {
        **sound = s;
    }

    events.write(EndOfRoundEvent {
        round: round.current,
    });

    round.current += 1;
}

/// When the last round is reached, record the score and return to menu.
fn end_of_game_system(
    mut settings: ResMut<GameSettings>,
    mut history: ResMut<ScoreHistory>,
    mut app_state: ResMut<NextState<AppState>>,
    session: Single<(&CueEngine, &Round, &CueTimer, &Score), With<Session>>,
) {
    let (engine, round, timer, score) = *session;

    if !round.is_last() {
        return;
    }

    history.0.push(ScoreRecord {
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
