use bevy::prelude::*;

pub const PADDLE_SPEED: f32 = 800.0;
pub const PADDLE_HEIGHT: f32 = 30.0;
pub const PADDLE_WIDTH: f32 = 120.0;

pub const BALL_RADIUS: f32 = 20.0;
pub const BALL_SPEED: f32 = 5.0;

//Blocks
pub const COLS: i32 = 2;
pub const ROWS: i32 = 4;
pub const GAP: f32 = 2.0;

//Colors
pub const PADDLE_COLOR: Color = Color::srgb(0.95, 0.25, 0.65);
pub const BALL_COLOR: Color = Color::srgb(0.95, 0.25, 0.65);
pub const BACKGROUND_COLOR: Color = Color::srgb(0.08, 0.08, 0.12);
pub const BLOCK_COLOR: Color = Color::srgb(0.18, 0.48, 0.19);
pub const BORDER_COLOR: Color = Color::srgb(0.05, 0.85, 0.95);

pub const SCREEN_COLOR: Color = Color::srgb(0.08, 0.08, 0.12);
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
