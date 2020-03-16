use sfml::system::*;

pub struct Sector {
    pub sides: Vec<Side>,
    pub ceil_height: f32,
    pub floor_height: f32
}

pub struct Side {
    pub p1: Vector2f,
    pub p2: Vector2f,
    pub neighbour_sect: i32,
    pub neighbour_side: i32
}

pub struct Thing {
    pub pos: Vector2f, // Position
    pub zpos: f32,     // Gravity-affected vert. position
    pub rot: f32       // Rotation
}