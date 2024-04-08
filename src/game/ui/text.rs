use bevy::prelude::*;

use crate::game::{settings::GameSettings, EndOfRoundEvent};

#[derive(Component)]
pub struct CurrentRoundText;

#[allow(clippy::type_complexity)]
pub fn round_system(
    settings: Res<GameSettings>,
    mut events: EventReader<EndOfRoundEvent>,
    mut query: Query<(&mut Text, &CurrentRoundText)>,
) {
    for (text, _) in &mut query.get_single_mut() {
        for e in events.read() {
            text.sections[0].value = format!("{}/{}", e.round, settings.rounds);
        }
    }
}
