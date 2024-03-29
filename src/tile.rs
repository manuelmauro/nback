use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::config;

#[derive(Clone, Component, Debug, Default, PartialEq)]
pub enum TilePosition {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    #[default]
    None,
}

impl TilePosition {
    fn row(&self) -> f32 {
        match self {
            TilePosition::TopLeft | TilePosition::TopCenter | TilePosition::TopRight => 1.0,
            TilePosition::CenterLeft | TilePosition::Center | TilePosition::CenterRight => 0.0,
            TilePosition::BottomLeft | TilePosition::BottomCenter | TilePosition::BottomRight => {
                -1.0
            }
            TilePosition::None => 0.0,
        }
    }

    fn column(&self) -> f32 {
        match self {
            TilePosition::TopLeft | TilePosition::CenterLeft | TilePosition::BottomLeft => -1.0,
            TilePosition::TopCenter | TilePosition::Center | TilePosition::BottomCenter => 0.0,
            TilePosition::TopRight | TilePosition::CenterRight | TilePosition::BottomRight => 1.0,
            TilePosition::None => 0.0,
        }
    }
}

impl Distribution<TilePosition> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TilePosition {
        match rng.gen_range(0..=9) {
            0 => TilePosition::TopLeft,
            1 => TilePosition::TopCenter,
            2 => TilePosition::TopRight,
            3 => TilePosition::CenterLeft,
            4 => TilePosition::Center,
            5 => TilePosition::CenterRight,
            6 => TilePosition::BottomLeft,
            7 => TilePosition::BottomCenter,
            _ => TilePosition::BottomRight,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
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

impl From<&TilePosition> for Vec3 {
    fn from(tile: &TilePosition) -> Self {
        Vec3::new(
            tile.column() * (config::TILE_SIZE + config::TILE_SPACING),
            tile.row() * (config::TILE_SIZE + config::TILE_SPACING),
            0.0,
        )
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
            TileColor::None => Color::rgb(0.0, 0.0, 0.0),
        }
    }
}
