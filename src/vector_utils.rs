use sfml::system::*;
use crate::constants::*;

pub fn line_intersect (v1: Vector2f, v2: Vector2f, v3: Vector2f, v4: Vector2f) -> Vector2f {
    // From https://youtu.be/HQYsFshbkYw?t=188
    let mut v = Vector2f::new(cross_product(v1, v2), cross_product(v3, v4));
    let det = cross_product(v1 - v2, v3 - v4);
    v.x = cross_product(Vector2f::new(v.x, v1.x - v2.x), Vector2f::new(v.y, v3.x - v4.x)) / det;
    v.y = cross_product(Vector2f::new(v.x, v1.y - v2.y), Vector2::new(v.y, v3.y - v4.y)) / det;
    v
}

pub fn cross_product (v1: Vector2f, v2: Vector2f) -> f32 {
    v1.x * v2.y - v1.y * v2.x
}

pub fn sfml_vec (v: Vector2f) -> Vector2f {
    let center = Vector2f::new(WIDTH as f32 / 2., HEIGHT as f32 / 2.);
    center + Vector2f::new(v.x, -v.y)
}

pub fn rotate_vec (v: Vector2f, theta: f32) -> Vector2f {
    let t = -theta;
    let st = t.sin();
    let ct = t.cos();

    Vector2::new(
        v.x * ct - v.y * st,
        v.x * st + v.y * ct
    )
}

pub fn vector_magnitude (v: &Vector2f) -> f32 {
    (v.x * v.x + v.y * v.y).sqrt()
}