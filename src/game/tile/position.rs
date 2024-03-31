use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::config;

#[derive(Component, Clone, Debug, Default, PartialEq)]
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

impl From<&TilePosition> for Vec3 {
    fn from(tile: &TilePosition) -> Self {
        Vec3::new(
            tile.column() * (config::TILE_SIZE + config::TILE_SPACING),
            tile.row() * (config::TILE_SIZE + config::TILE_SPACING),
            0.0,
        )
    }
}
