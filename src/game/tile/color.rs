use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::config;

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub enum TileColor {
    A,
    B,
    C,
    D,
    E,
    #[default]
    None,
}

impl Distribution<TileColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileColor {
        match rng.gen_range(0..=5) {
            0 => TileColor::A,
            1 => TileColor::B,
            2 => TileColor::C,
            3 => TileColor::D,
            _ => TileColor::E,
        }
    }
}

impl From<&TileColor> for Color {
    fn from(c: &TileColor) -> Self {
        match c {
            TileColor::A => config::TILE_COLOR_A,
            TileColor::B => config::TILE_COLOR_B,
            TileColor::C => config::TILE_COLOR_C,
            TileColor::D => config::TILE_COLOR_D,
            TileColor::E => config::TILE_COLOR_E,
            TileColor::None => config::TILE_COLOR_C,
        }
    }
}
