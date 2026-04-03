use bevy::prelude::*;

use crate::game::settings::GameSettings;

#[derive(Component)]
pub struct NBackText;

pub fn nback_text_system(
    settings: Res<GameSettings>,
    mut query: Query<&mut Text, With<NBackText>>,
) {
    for mut text in &mut query {
        text.0 = format!("{}-Back", settings.n);
    }
}
