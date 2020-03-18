use std::f32::consts::PI;

// Do not change. Player movement is calibrated for this framerate.
pub const PHYSICS_TIMESTEP: f32 = 1. / 35.;

pub const PLAYER_ROT_SPEED: f32 = PI / 35.;
pub const PLAYER_MAX_STEP_HEIGHT: f32 = 24.;
pub const GRAVITY: f32 = 1.;
pub const FRICTION: f32 = 0.90625;
pub const PLAYER_ACCELERATION: f32 = 1.5625;
pub const PLAYER_SPEED_CAP: f32 = 30.;
// Same as DOOM
pub const PLAYER_EYE_HEIGHT: f32 = 41.;
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;
pub const XFOV: f32 = 0.75 * WIDTH as f32;
pub const YFOV: f32 = 0.9 * HEIGHT as f32;

pub const MAX_PORTAL_DRAWS: usize = 2;