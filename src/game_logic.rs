// For lock-step multiplayer
pub struct Step {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,

    pub rotation: f32
}