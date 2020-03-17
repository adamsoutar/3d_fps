use std::f32::consts::PI;

pub const PLAYER_SPEED: f32 = 300.;
pub const PLAYER_ROT_SPEED: f32 = PI;
pub const PLAYER_MAX_STEP_HEIGHT: f32 = 24.;
pub const GRAVITY: f32 = 1.;
// Same as DOOM
pub const PLAYER_EYE_HEIGHT: f32 = 41.;
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;
pub const XFOV: f32 = 0.75 * WIDTH as f32;
pub const YFOV: f32 = 0.9 * HEIGHT as f32;

pub const MAX_PORTAL_DRAWS: usize = 1000;