//! Centralised design tokens — colors, spacing, and radii.
//!
//! Every UI element should reference these constants instead of
//! hard-coding `Color::srgb(…)` or magic `px(…)` values.

use bevy::prelude::*;

use crate::palette;

// ── colours ──────────────────────────────────────────────────────────

pub const BG: Color = palette::SLATE_900;
pub const SURFACE: Color = palette::SLATE_800;
pub const SURFACE_ALT: Color = palette::SLATE_700;
pub const BORDER: Color = palette::SLATE_600;

pub const TEXT: Color = Color::srgb(0.92, 0.93, 0.95);
pub const TEXT_MUTED: Color = palette::SLATE_400;

pub const ACCENT: Color = palette::LIME_500;
pub const ACCENT_HOVER: Color = palette::LIME_900;
pub const ACCENT_PRESS: Color = palette::LIME_600;

pub const GAME_BTN: Color = palette::TEAL_600;
pub const GAME_BTN_HOVER: Color = palette::TEAL_700;
pub const GAME_BTN_PRESS: Color = palette::TEAL_800;

pub const TIMER_TRACK: Color = palette::SLATE_700;
pub const TIMER_FILL: Color = palette::LIME_500;

// ── spacing ──────────────────────────────────────────────────────────

pub const SP_XS: Val = Val::Px(4.0);
pub const SP_SM: Val = Val::Px(8.0);
pub const SP_MD: Val = Val::Px(16.0);
pub const SP_LG: Val = Val::Px(24.0);
pub const SP_XL: Val = Val::Px(32.0);

// ── radii ────────────────────────────────────────────────────────────

pub const RADIUS_SM: Val = Val::Px(6.0);
pub const RADIUS_MD: Val = Val::Px(12.0);
pub const RADIUS_LG: Val = Val::Px(16.0);
pub const RADIUS_FULL: Val = Val::Px(9999.0);
