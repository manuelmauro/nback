use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{asset::AudioAssets, config, state::AppState};

use self::{color::TileColor, position::TilePosition, shape::TileShape, sound::TileSound};

pub mod color;
pub mod position;
pub mod shape;
pub mod sound;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tile_position_system,
                tile_color_system,
                tile_shape_system,
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
#[require(TilePopAnimation, TilePosition, TileColor, TileShape, TileSound)]
pub struct Tile;

/// Pre-computed mesh handles for each tile shape.
#[derive(Resource)]
pub struct TileMeshes {
    pub circle: Handle<Mesh>,
    pub triangle: Handle<Mesh>,
    pub square: Handle<Mesh>,
    pub pentagon: Handle<Mesh>,
    pub hexagon: Handle<Mesh>,
}

impl TileMeshes {
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        let r = config::TILE_SIZE / 2.0;
        TileMeshes {
            circle: meshes.add(Circle::new(r)),
            triangle: meshes.add(RegularPolygon::new(r, 3)),
            square: meshes.add(RegularPolygon::new(r, 4)),
            pentagon: meshes.add(RegularPolygon::new(r, 5)),
            hexagon: meshes.add(RegularPolygon::new(r, 6)),
        }
    }

    pub fn get(&self, shape: &TileShape) -> Handle<Mesh> {
        match shape {
            TileShape::Circle => self.circle.clone(),
            TileShape::Triangle => self.triangle.clone(),
            TileShape::Square | TileShape::None => self.square.clone(),
            TileShape::Pentagon => self.pentagon.clone(),
            TileShape::Hexagon => self.hexagon.clone(),
        }
    }
}

/// Update tile state every time the position changes.
pub fn tile_position_system(
    mut tile: Single<(&mut Transform, &mut TilePopAnimation, &TilePosition), Changed<TilePosition>>,
) {
    let (transform, anim, position) = &mut *tile;
    info!(?position, "tile updated");
    transform.translation = (*position).into();
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

/// Update tile material color when the color cue changes.
pub fn tile_color_system(
    mut materials: ResMut<Assets<ColorMaterial>>,
    tile: Single<(&MeshMaterial2d<ColorMaterial>, &TileColor), Changed<TileColor>>,
) {
    let (mat_handle, color) = *tile;
    info!(?color, "tile color updated");
    if let Some(material) = materials.get_mut(mat_handle) {
        material.color = color.into();
    }
}

/// Swap the tile mesh when the shape cue changes.
pub fn tile_shape_system(
    tile_meshes: Res<TileMeshes>,
    mut tile: Single<(&mut Mesh2d, &mut TilePopAnimation, &TileShape), Changed<TileShape>>,
) {
    let (mesh, anim, shape) = &mut *tile;
    info!(?shape, "tile shape updated");
    mesh.0 = tile_meshes.get(shape);
    anim.timer.reset();
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
