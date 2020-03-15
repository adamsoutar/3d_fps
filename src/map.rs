use sfml::system::*;
use sfml::graphics::*;

// Clonable for transformed map
#[derive(Clone)]
pub struct Wall {
    pub colour: Color,
    pub p1: Vector2f,
    pub p2: Vector2f,
    pub height: f32
}
pub struct Thing {
    pub pos: Vector2f, // Position
    pub rot: f32       // Rotation
}