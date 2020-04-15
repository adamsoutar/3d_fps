use sfml::system::*;

pub struct Sector {
    pub sides: Vec<Side>,
    pub ceil_height: f32,
    pub floor_height: f32,
    pub ceil_texture: String,
    pub floor_texture: String
}

pub struct Side {
    pub p1: Vector2f,
    pub p2: Vector2f,
    pub neighbour: i32,
    pub mid: String,
    pub lower: String,
    pub upper: String
}

// TODO: Do Thing as a trait
//       with inheriting structs like LocalPlayer, NetPlayer
pub struct Thing {
    pub pos: Vector2f, // Position
    pub zpos: f32,     // Gravity-affected vert. position
    pub falling: bool,  // Are we heading down?
    pub fall_velocity: f32,
    pub velocity: Vector2f,
    pub rot: f32,      // Rotation
    pub rsin: f32, // Cached trig values for the rotation
    pub rcos: f32, // For speed
    pub sector: usize,  // Sector in which the object resides
    pub yaw: f32 // Vertical look
}

impl Thing {
    pub fn set_rotation(&mut self, rot: f32) {
        self.rot = rot;
        self.rsin = rot.sin();
        self.rcos = rot.cos();
    }

    pub fn rotate(&mut self, delta: f32) {
        self.set_rotation(self.rot + delta)
    }
}