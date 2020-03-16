use sfml::system::*;
use sfml::graphics::*;

pub struct Sector {
    pub vertices: Vec<Vector2f>,
    pub ceil_height: f32,
    pub floor_height: f32
}
pub struct Thing {
    pub pos: Vector2f, // Position
    pub zpos: f32,     // Gravity-affected vert. position
    pub rot: f32       // Rotation
}