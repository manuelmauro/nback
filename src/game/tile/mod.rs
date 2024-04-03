use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{asset::AudioAssets, config, state::AppState};

use self::{color::TileColor, position::TilePosition, sound::TileSound};

pub mod color;
pub mod position;
pub mod sound;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            // NOTE the run_if guard is a woraround for running the tile_sound_system
            // only when the audio resource handles are ready
            (tile_position_system, tile_color_system, tile_sound_system)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub sprite: SpriteBundle,
    pub name: Name,
    pub animation: AnimationPlayer,
    pub position: TilePosition,
    pub color: TileColor,
    pub sound: TileSound,
}

impl Default for TileBundle {
    fn default() -> Self {
        TileBundle {
            sprite: SpriteBundle {
                transform: Transform::from_translation((&TilePosition::None).into()),
                sprite: Sprite {
                    color: (&TileColor::None).into(),
                    custom_size: Some(Vec2::new(config::TILE_SIZE, config::TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            name: Name::default(),
            animation: AnimationPlayer::default(),
            position: TilePosition::None,
            color: TileColor::None,
            sound: TileSound::None,
        }
    }
}

/// Update tile state every time the position changes.
pub fn tile_position_system(
    mut query: Query<(&mut Transform, &mut AnimationPlayer, &TilePosition), Changed<TilePosition>>,
) {
    if let Ok((mut transform, mut animation, position)) = query.get_single_mut() {
        info!(?position, "tile updated");
        transform.translation = position.into();
        animation.replay();
    }
}

/// Update tile state every time the color changes.
pub fn tile_color_system(mut query: Query<(&mut Sprite, &TileColor), Changed<TileColor>>) {
    if let Ok((mut sprite, color)) = query.get_single_mut() {
        info!(?color, "tile updated");
        sprite.color = color.into();
    }
}

/// Update tile state every time the color changes.
pub fn tile_sound_system(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut query: Query<&TileSound, Changed<TileSound>>,
) {
    if let Ok(sound) = query.get_single_mut() {
        match sound {
            TileSound::C => {
                audio.play(audio_assets.c.clone());
            }
            TileSound::H => {
                audio.play(audio_assets.h.clone());
            }
            TileSound::K => {
                audio.play(audio_assets.k.clone());
            }
            TileSound::L => {
                audio.play(audio_assets.l.clone());
            }
            TileSound::Q => {
                audio.play(audio_assets.q.clone());
            }
            TileSound::R => {
                audio.play(audio_assets.r.clone());
            }
            TileSound::S => {
                audio.play(audio_assets.s.clone());
            }
            TileSound::T => {
                audio.play(audio_assets.t.clone());
            }
            TileSound::None => (),
        }
    }
}
