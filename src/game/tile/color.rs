use bevy::prelude::*;
use rand::{
    distr::{Distribution, StandardUniform},
    Rng, RngExt,
};

use crate::palette;

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

impl Distribution<TileColor> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileColor {
        match rng.random_range(0..=5) {
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
            TileColor::A => palette::RED_500,
            TileColor::B => palette::ORANGE_500,
            TileColor::C => palette::YELLOW_400,
            TileColor::D => palette::EMERALD_500,
            TileColor::E => palette::BLUE_500,
            TileColor::None => palette::LIME_500,
        }
    }
}
