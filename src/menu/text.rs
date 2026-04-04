use bevy::prelude::*;

use crate::game::settings::GameSettings;

#[derive(Component)]
pub struct NBackText;

pub fn nback_text_system(
    settings: Res<GameSettings>,
    mut text: Single<&mut Text, With<NBackText>>,
) {
    text.0 = format!("{}-Back", settings.n);
}
