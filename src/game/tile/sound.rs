use bevy::prelude::*;
use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

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

impl Distribution<TileSound> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileSound {
        match rng.random_range(0..=7) {
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
