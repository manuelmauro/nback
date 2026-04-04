use bevy::prelude::*;

use crate::palette;

/// Reference design resolution. All `Val::Px` values in UI are authored for this size.
/// At runtime the window's scale factor is adjusted so the UI scales uniformly.
pub const REF_WIDTH: f32 = 720.0;
pub const REF_HEIGHT: f32 = 1280.0;

/// The world-space width the game was designed for.
/// The camera projection is scaled so this many world units always fit the viewport.
pub const WORLD_WIDTH: f32 = 720.0;

// tile
pub const TILE_SIZE: f32 = 100.0;
pub const TILE_SPACING: f32 = 10.0;

// splash screen
pub const SPLASH_SCREEN_DURATION: f32 = 1.0;

// game/menu screen
pub const WALL_COLOR: Color = palette::WHITE;
pub const WALL_THICKNESS: f32 = 4.0;
