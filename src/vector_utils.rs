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
pub fn dot_product (v1: Vector2f, v2: Vector2f) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

// Gives cos of the angle between two vectors
pub fn cos_theta (v1: Vector2f, v2: Vector2f) -> f32 {
    dot_product(v1, v2) / (mag(&v1) * mag(&v2))
}

pub fn unit_vector (v: Vector2f) -> Vector2f {
    1. / mag(&v) * v
}

pub fn vector_projection (v1: Vector2f, v2: Vector2f) -> Vector2f {
    mag(&v1) * cos_theta(v1, v2) * unit_vector(v2)
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

pub fn point_side (px1: &Vector2f, px2: &Vector2f, px: &Vector2f) -> f32 {
    let p1 = px1.clone();
    let p2 = px2.clone();
    let p = px.clone();
    cross_product(p2 - p1, p - p1)
}

pub fn mag (v: &Vector2f) -> f32 {
    (v.x * v.x + v.y * v.y).sqrt()
}

#[derive(PartialEq)]
pub enum SegmentIntersection {
    Collinear,
    Parallel,
    Intersection,
    None
}

// Determine the manner in which two line segments do or do not intersect
// https://stackoverflow.com/a/565282/7674702
pub fn segment_intersection (p1: &Vector2f, p2: &Vector2f, p3: &Vector2f, p4: &Vector2f) -> SegmentIntersection {
    let p = p1.clone();
    let r = p2.clone() - p;
    let q = p3.clone();
    let s = p4.clone() - q;

    let t = cross_product(q - p, s) / cross_product(r, s);
    let u = cross_product(p - q, r) / cross_product(s, r);

    let m1 = cross_product(r, s);
    let m2 = cross_product(q - p, r);

    // Not sure if 0 checks are 100% accurate for f32 and cross_product
    if m1 == 0. && m2 == 0. {
        return SegmentIntersection::Collinear;
    }

    if m1 == 0. {
        return SegmentIntersection::Parallel;
    }

    // This is mainly what we're after
    if t >= 0. && t <= 1. && u >= 0. && u <= 1. {
        return SegmentIntersection::Intersection;
    }

    SegmentIntersection::None
}