use bevy::prelude::*;

use crate::game::{settings::GameSettings, EndOfRoundEvent};

#[derive(Component)]
pub struct CurrentRoundText;

#[allow(clippy::type_complexity)]
pub fn round_system(
    settings: Res<GameSettings>,
    mut events: MessageReader<EndOfRoundEvent>,
    mut query: Query<&mut Text, With<CurrentRoundText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        for e in events.read() {
            text.0 = format!("{}/{}", e.round, settings.rounds);
        }
    }
}
