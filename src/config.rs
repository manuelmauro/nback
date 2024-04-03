use bevy::prelude::*;

// tile
pub const TILE_SIZE: f32 = 100.0;
pub const TILE_SPACING: f32 = 10.0;
pub const TILE_COLOR_A: Color = Color::rgb(1.0, 0.56, 0.0);
pub const TILE_COLOR_B: Color = Color::rgb(0.60, 0.05, 1.0);
pub const TILE_COLOR_C: Color = Color::rgb(1.0, 0.0, 0.65);
pub const TILE_COLOR_D: Color = Color::rgb(0.12, 1.0, 0.14);
pub const TILE_COLOR_E: Color = Color::rgb(0.12, 0.80, 1.0);
pub const TILE_SOUND_C: &str = "sounds/letters/c.ogg";
pub const TILE_SOUND_H: &str = "sounds/letters/h.ogg";
pub const TILE_SOUND_K: &str = "sounds/letters/k.ogg";
pub const TILE_SOUND_L: &str = "sounds/letters/l.ogg";
pub const TILE_SOUND_Q: &str = "sounds/letters/q.ogg";
pub const TILE_SOUND_R: &str = "sounds/letters/r.ogg";
pub const TILE_SOUND_S: &str = "sounds/letters/s.ogg";
pub const TILE_SOUND_T: &str = "sounds/letters/t.ogg";

// splash screen
pub const SPLASH_SCREEN_DURATION: f32 = 1.0;

// game/menu screen
pub const WALL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const WALL_THICKNESS: f32 = 4.0;

// buttons
pub const NORMAL_BUTTON: Color = Color::rgb(0.16, 0.17, 0.18);
pub const HOVERED_BUTTON: Color = Color::rgb(0.20, 0.21, 0.22);
pub const PRESSED_BUTTON: Color = Color::rgb(1.0, 0.56, 0.0);
pub const BUTTON_BORDER_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const PRESSED_BUTTON_BORDER_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
