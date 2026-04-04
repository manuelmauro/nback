use bevy::prelude::*;
use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub enum TileShape {
    Circle,
    Triangle,
    Square,
    Pentagon,
    Hexagon,
    #[default]
    None,
}

impl Distribution<TileShape> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileShape {
        match rng.random_range(0..=4) {
            0 => TileShape::Circle,
            1 => TileShape::Triangle,
            2 => TileShape::Square,
            3 => TileShape::Pentagon,
            _ => TileShape::Hexagon,
        }
    }
}
