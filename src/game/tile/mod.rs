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
            (
                tile_position_system,
                tile_color_system,
                tile_sound_system,
                tile_pop_animation_system,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}

/// Simple pop animation component.
#[derive(Component)]
pub struct TilePopAnimation {
    pub timer: Timer,
}

impl Default for TilePopAnimation {
    fn default() -> Self {
        TilePopAnimation {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
        }
    }
}

/// Marker component for the game tile. Required components are auto-inserted.
#[derive(Component, Default)]
#[require(TilePopAnimation, TilePosition, TileColor, TileSound)]
pub struct Tile;

impl Tile {
    /// Returns the default sprite + transform for the tile.
    pub fn bundle() -> (Tile, Sprite, Transform) {
        (
            Tile,
            Sprite {
                color: (&TileColor::None).into(),
                custom_size: Some(Vec2::new(config::TILE_SIZE, config::TILE_SIZE)),
                ..default()
            },
            Transform::from_translation((&TilePosition::None).into()),
        )
    }
}

/// Update tile state every time the position changes.
pub fn tile_position_system(
    mut tile: Single<(&mut Transform, &mut TilePopAnimation, &TilePosition), Changed<TilePosition>>,
) {
    let (transform, anim, position) = &mut *tile;
    info!(?position, "tile updated");
    transform.translation = (*position).into();
    // Reset and start the pop animation
    anim.timer.reset();
}

/// Animate tile scale (pop effect: 0.8 → 1.0).
pub fn tile_pop_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TilePopAnimation)>,
) {
    for (mut transform, mut anim) in &mut query {
        if !anim.timer.is_finished() {
            anim.timer.tick(time.delta());
            let t = anim.timer.fraction();
            let scale = 0.8 + 0.2 * t;
            transform.scale = Vec3::splat(scale);
        }
    }
}

/// Update tile state every time the color changes.
pub fn tile_color_system(mut tile: Single<(&mut Sprite, &TileColor), Changed<TileColor>>) {
    let (sprite, color) = &mut *tile;
    info!(?color, "tile updated");
    sprite.color = (*color).into();
}

/// Update tile state every time the sound changes.
pub fn tile_sound_system(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    sound: Single<&TileSound, Changed<TileSound>>,
) {
    match *sound {
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
