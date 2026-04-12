//! Design tokens and interaction palettes shared across the UI.

use bevy::prelude::*;

use crate::palette;

const NEON_RED: Color = Color::srgba(1.0, 98.0 / 255.0, 81.0 / 255.0, 1.0);
const NEON_RED_DIM: Color = Color::srgba(172.0 / 255.0, 64.0 / 255.0, 63.0 / 255.0, 1.0);
const NEON_YELLOW: Color = Color::srgba(252.0 / 255.0, 226.0 / 255.0, 8.0 / 255.0, 1.0);
const NEON_CYAN: Color = Color::srgba(8.0 / 255.0, 226.0 / 255.0, 252.0 / 255.0, 1.0);

pub const BG: Color = Color::srgba(8.0 / 255.0, 10.0 / 255.0, 14.0 / 255.0, 1.0);
pub const SURFACE: Color = Color::srgba(19.0 / 255.0, 24.0 / 255.0, 33.0 / 255.0, 0.96);
pub const SURFACE_ALT: Color = Color::srgba(25.0 / 255.0, 32.0 / 255.0, 43.0 / 255.0, 0.96);

pub const BORDER: Color = Color::srgba(172.0 / 255.0, 64.0 / 255.0, 63.0 / 255.0, 0.6);
pub const BORDER_HOVER: Color = Color::srgba(252.0 / 255.0, 226.0 / 255.0, 8.0 / 255.0, 0.95);
pub const BORDER_PRESS: Color = Color::srgba(8.0 / 255.0, 226.0 / 255.0, 252.0 / 255.0, 1.0);

pub const TEXT: Color = palette::SLATE_100;
pub const TEXT_MUTED: Color = palette::SLATE_300;
pub const TEXT_ACCENT: Color = NEON_YELLOW;
pub const TEXT_ON_ACCENT: Color = BG;

pub const ACCENT: Color = NEON_YELLOW;
pub const ACCENT_HOVER: Color = NEON_RED;
pub const ACCENT_PRESS: Color = NEON_CYAN;

pub const GAME_BTN: Color = SURFACE_ALT;
pub const GAME_BTN_HOVER: Color = NEON_RED_DIM;
pub const GAME_BTN_PRESS: Color = NEON_RED;

pub const TIMER_TRACK: Color = SURFACE_ALT;
pub const TIMER_FILL: Color = NEON_RED;
pub const TIMER_DANGER: Color = NEON_YELLOW;

// ── interaction palettes ───────────────────────────────────────────

#[derive(Component, Clone, Copy)]
pub struct ButtonPalette {
    pub idle_bg: Color,
    pub hover_bg: Color,
    pub pressed_bg: Color,
    pub idle_border: Color,
    pub hover_border: Color,
    pub pressed_border: Color,
}

impl ButtonPalette {
    pub const fn new(
        idle_bg: Color,
        hover_bg: Color,
        pressed_bg: Color,
        idle_border: Color,
        hover_border: Color,
        pressed_border: Color,
    ) -> Self {
        Self {
            idle_bg,
            hover_bg,
            pressed_bg,
            idle_border,
            hover_border,
            pressed_border,
        }
    }
}

pub const BUTTON_PRIMARY: ButtonPalette = ButtonPalette::new(
    ACCENT,
    ACCENT_HOVER,
    ACCENT_PRESS,
    BORDER_HOVER,
    BORDER_HOVER,
    BORDER_PRESS,
);

pub const BUTTON_SECONDARY: ButtonPalette = ButtonPalette::new(
    SURFACE,
    SURFACE_ALT,
    ACCENT_PRESS,
    BORDER,
    BORDER_HOVER,
    BORDER_PRESS,
);

pub const BUTTON_GAME: ButtonPalette = ButtonPalette::new(
    GAME_BTN,
    GAME_BTN_HOVER,
    GAME_BTN_PRESS,
    BORDER,
    BORDER_HOVER,
    BORDER_PRESS,
);

/// Apply a button palette based on interaction state.
pub fn apply_button_palette(
    interaction: &Interaction,
    palette: &ButtonPalette,
    bg: &mut BackgroundColor,
    border: &mut BorderColor,
) {
    let (fill, stroke) = match *interaction {
        Interaction::Pressed => (palette.pressed_bg, palette.pressed_border),
        Interaction::Hovered => (palette.hover_bg, palette.hover_border),
        Interaction::None => (palette.idle_bg, palette.idle_border),
    };

    bg.0 = fill;
    *border = BorderColor::all(stroke);
}

// ── spacing ─────────────────────────────────────────────────────────

pub const SP_XS: Val = Val::Px(4.0);
pub const SP_SM: Val = Val::Px(8.0);
pub const SP_MD: Val = Val::Px(16.0);
pub const SP_LG: Val = Val::Px(24.0);
pub const SP_XL: Val = Val::Px(32.0);

pub const STROKE_SM: Val = Val::Px(1.0);
pub const STROKE_MD: Val = Val::Px(2.0);

// ── radii ───────────────────────────────────────────────────────────

// Radius tokens are zeroed for now to keep this change focused on shared theming.
pub const RADIUS_SM: Val = Val::Px(0.0);
pub const RADIUS_MD: Val = Val::Px(0.0);
pub const RADIUS_LG: Val = Val::Px(0.0);
pub const RADIUS_FULL: Val = Val::Px(0.0);
