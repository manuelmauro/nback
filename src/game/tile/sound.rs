use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::config;

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub enum TileSound {
    C,
    H,
    K,
    L,
    Q,
    R,
    S,
    T,
    #[default]
    None,
}

impl Distribution<TileSound> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileSound {
        match rng.gen_range(0..=5) {
            0 => TileSound::C,
            1 => TileSound::H,
            2 => TileSound::K,
            3 => TileSound::L,
            4 => TileSound::Q,
            5 => TileSound::R,
            6 => TileSound::S,
            _ => TileSound::T,
        }
    }
}

impl From<&TileSound> for Option<&str> {
    fn from(c: &TileSound) -> Self {
        match c {
            TileSound::C => Some(config::TILE_SOUND_C),
            TileSound::H => Some(config::TILE_SOUND_H),
            TileSound::K => Some(config::TILE_SOUND_K),
            TileSound::L => Some(config::TILE_SOUND_L),
            TileSound::Q => Some(config::TILE_SOUND_Q),
            TileSound::R => Some(config::TILE_SOUND_R),
            TileSound::S => Some(config::TILE_SOUND_S),
            TileSound::T => Some(config::TILE_SOUND_T),
            TileSound::None => None,
        }
    }
}
