use bevy::prelude::*;

use crate::{
    game::{
        session::{EndOfRoundEvent, Session, cue::CueTimer},
        settings::GameSettings,
    },
    theme,
};

#[derive(Component)]
pub struct CurrentRoundText;

/// Marker for the timer progress bar fill node.
#[derive(Component)]
pub struct TimerBar;

pub fn round_system(
    settings: Res<GameSettings>,
    mut events: MessageReader<EndOfRoundEvent>,
    mut text: Single<&mut Text, With<CurrentRoundText>>,
) {
    for e in events.read() {
        text.0 = format!("{}/{}", e.round, settings.rounds);
    }
}

/// Update the timer bar width every frame.
pub fn timer_bar_system(
    session: Single<&CueTimer, With<Session>>,
    mut bar_node: Single<&mut Node, With<TimerBar>>,
    mut bar_bg: Single<&mut BackgroundColor, With<TimerBar>>,
) {
    let fraction = session.elapsed().as_secs_f32() / session.duration().as_secs_f32();
    bar_node.width = percent(fraction * 100.0);
    bar_bg.0 = theme::TIMER_FILL;
}
