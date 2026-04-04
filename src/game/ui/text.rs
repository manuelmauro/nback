use bevy::prelude::*;

use crate::game::{session::EndOfRoundEvent, settings::GameSettings};

#[derive(Component)]
pub struct CurrentRoundText;

pub fn round_system(
    settings: Res<GameSettings>,
    mut events: MessageReader<EndOfRoundEvent>,
    mut text: Single<&mut Text, With<CurrentRoundText>>,
) {
    for e in events.read() {
        text.0 = format!("{}/{}", e.round, settings.rounds);
    }
}
