use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::config::{SIZE, SPACING};

#[derive(Clone, Component, Debug, Default, PartialEq)]
pub enum Tile {
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

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        match rng.gen_range(0..=9) {
            0 => Tile::TopLeft,
            1 => Tile::TopCenter,
            2 => Tile::TopRight,
            3 => Tile::CenterLeft,
            4 => Tile::Center,
            5 => Tile::CenterRight,
            6 => Tile::BottomLeft,
            7 => Tile::BottomCenter,
            _ => Tile::BottomRight,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Pigment {
    A,
    B,
    C,
    D,
    E,
    #[default]
    None,
}

impl Distribution<Pigment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pigment {
        match rng.gen_range(0..=5) {
            0 => Pigment::A,
            1 => Pigment::B,
            2 => Pigment::C,
            3 => Pigment::D,
            _ => Pigment::E,
        }
    }
}

impl From<&Tile> for Vec3 {
    fn from(tile: &Tile) -> Self {
        Vec3::new(
            column(tile) * (SIZE + SPACING),
            row(tile) * (SIZE + SPACING),
            0.0,
        )
    }
}

fn row(tile: &Tile) -> f32 {
    match &tile {
        Tile::TopLeft => 1.0,
        Tile::TopCenter => 1.0,
        Tile::TopRight => 1.0,
        Tile::CenterLeft => 0.0,
        Tile::Center => 0.0,
        Tile::CenterRight => 0.0,
        Tile::BottomLeft => -1.0,
        Tile::BottomCenter => -1.0,
        Tile::BottomRight => -1.0,
        Tile::None => 0.0,
    }
}

fn column(tile: &Tile) -> f32 {
    match &tile {
        Tile::TopLeft => -1.0,
        Tile::TopCenter => 0.0,
        Tile::TopRight => 1.0,
        Tile::CenterLeft => -1.0,
        Tile::Center => 0.0,
        Tile::CenterRight => 1.0,
        Tile::BottomLeft => -1.0,
        Tile::BottomCenter => 0.0,
        Tile::BottomRight => 1.0,
        Tile::None => 0.0,
    }
}

impl From<&Pigment> for Color {
    fn from(pigment: &Pigment) -> Self {
        match pigment {
            Pigment::A => Color::rgb(1.0, 0.56, 0.0),
            Pigment::B => Color::rgb(0.60, 0.05, 1.0),
            Pigment::C => Color::rgb(1.0, 0.0, 0.65),
            Pigment::D => Color::rgb(0.12, 1.0, 0.14),
            Pigment::E => Color::rgb(0.12, 0.80, 1.0),
            Pigment::None => Color::rgb(0.0, 0.0, 0.0),
        }
    }
}
