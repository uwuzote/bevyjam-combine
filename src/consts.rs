use bevy::prelude::{ClearColor, Color};

pub const LAUNCHER_TITLE: &'static str = "Koci4 combinations";

pub const PIXEL_SCALE: f32 = 3.0;

pub const TILE_SIZE: f32 = 24.0;
pub const DEMON_SIZE: f32 = 24.0;
pub const ITEM_SIZE: f32 = 16.0;

pub const STEP_SIZE: f32 = TILE_SIZE * PIXEL_SCALE;

pub const TILE_SCALE: f32 = STEP_SIZE / TILE_SIZE;
pub const DEMON_SCALE: f32 = STEP_SIZE / DEMON_SIZE;
pub const ITEM_SCALE: f32 = 5.0;

pub const TILE_Z_POS: f32 = 0.0;
pub const DEMON_Z_POS: f32 = 1.0;
pub const HIGHLIGHTER_Z_POS: f32 = DEMON_Z_POS - 0.1;

pub const CLEAR_COLOR: ClearColor = ClearColor(Color::rgb(0.4, 0.0, 0.0));
pub const HOTBAR_PADDING: f32 = 5.0;
