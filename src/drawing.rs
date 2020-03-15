use sfml::graphics::*;
use sfml::system::*;
use std::f32::consts::PI;

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;

pub fn draw_3d_map (window: &mut RenderWindow, map: &mut Vec<Wall>) {
    let to_top = HEIGHT as f32 / 2.;

    // "To perspective project a co-ordinate, you simply take its x and y
    //  co-ordinate and divide them by the z co-ordinate"
    // tz1 = wall.p1.y
    // tz2 = wall.p2.y
    // tx1 = wall.p1.x
    // tx2 = wall.p2.x
    for wall in map {
        if wall.p1.y > 0. || wall.p2.y  > 0. {
            // View frustum clipping
            let i1 = line_intersect(
                wall.p1,
                wall.p2,
                Vector2f::new(0., 0.),
                Vector2f::new(-1., 0.001));
            let i2 = line_intersect(
                wall.p1,
                wall.p2,
                Vector2f::new(0., 0.),
                Vector2f::new(1., 0.001));
            if wall.p1.y <= 0. {
                if i1.y > 0. {
                    wall.p1 = i1
                } else {
                    wall.p1 = i2
                }
            }
            if wall.p2.y <= 0. {
                if i1.y > 0. {
                    wall.p2 = i1;
                } else {
                    wall.p2 = i2;
                }
            }

            // Perspective projection
            let wz1 = wall.height / 2.;
            let wz2 = -wz1;
            let wx1 = wall.p1.x * 500. / wall.p1.y;
            let wx2 = wall.p2.x * 500. / wall.p2.y;
            let wy1a = wz2 / wall.p1.y;
            let wy1b = wz1 / wall.p1.y;
            let wy2a = wz2 / wall.p2.y;
            let wy2b = wz1 / wall.p2.y;

            let top_left = Vector2f::new(wx1, wy1a);
            let top_right = Vector2f::new(wx2, wy2a);
            let bottom_right = Vector2f::new(wx2, wy2b);
            let bottom_left = Vector2f::new(wx1, wy1b);


            let ceil_left = Vector2f::new(top_left.x, to_top);
            let ceil_right = Vector2f::new(top_right.x, to_top);
            let floor_left = Vector2f::new(bottom_left.x, -to_top);
            let floor_right = Vector2f::new(bottom_right.x, -to_top);

            // Ceiling
            draw_quad(window, ceil_left, ceil_right, top_right, top_left, Color::WHITE);
            // Floor
            draw_quad(window, bottom_left, bottom_right, floor_right, floor_left, Color::BLUE);
            // Wall
            draw_quad(window, top_left, top_right, bottom_right, bottom_left, wall.colour);
        }
    }
}

pub fn draw_quad (window: &mut RenderWindow, top_left: Vector2f, top_right: Vector2f, bottom_right: Vector2f, bottom_left: Vector2f, colour: Color) {
    let mut vertex_array = VertexArray::default();
    vertex_array.set_primitive_type(PrimitiveType::Triangles);

    let tl = sfml_vec(top_left);
    let tr = sfml_vec(top_right);
    let br = sfml_vec(bottom_right);
    let bl = sfml_vec(bottom_left);
    vertex_array.append(&Vertex::with_pos_color(tl, colour));
    vertex_array.append(&Vertex::with_pos_color(tr, colour));
    vertex_array.append(&Vertex::with_pos_color(bl, colour));
    vertex_array.append(&Vertex::with_pos_color(bl, colour));
    vertex_array.append(&Vertex::with_pos_color(tr, colour));
    vertex_array.append(&Vertex::with_pos_color(br, colour));

    window.draw(&vertex_array);
}

// Angle in radians
pub fn draw_line_at_rotation (window: &mut RenderWindow, pos: Vector2f, length: f32, angle: f32, colour: Color) {
    let mut rct = RectangleShape::new();
    rct.set_size(Vector2f::new(length, 3.));
    rct.set_rotation(angle * 180. / PI - 90.);
    rct.set_fill_color(&colour);
    rct.set_position(sfml_vec(pos));

    window.draw(&rct);
}

pub fn draw_line (window: &mut RenderWindow, p1: Vector2f, p2: Vector2f, colour: Color) {
    let xdiff = p2.x - p1.x;
    let ydiff = p2.y - p1.y;
    let angle = -ydiff.atan2(xdiff) + PI / 2.;

    let yda = ydiff.abs();
    let xda = xdiff.abs();

    let py = xda * xda + yda * yda;
    let dist = py.sqrt();

    draw_line_at_rotation(window, p1, dist, angle, colour);
}

pub fn draw_wall (wall: &Wall, window: &mut RenderWindow) {
    draw_line(window, wall.p1, wall.p2, wall.colour);
}

pub fn draw_thing (window: &mut RenderWindow, thing: &Thing, player: &Thing) {
    let mut rct = RectangleShape::new();
    rct.set_size(Vector2f::new(10., 10.));
    rct.set_fill_color(&Color::GREEN);
    rct.set_position(sfml_vec(thing.pos - player.pos) - Vector2f::new(5., 5.));

    window.draw(&rct);

    draw_line_at_rotation(window, thing.pos - player.pos, 30., thing.rot - player.rot, Color::WHITE);
}

pub fn draw_map (window: &mut RenderWindow, map: &Vec<Wall>, player: &Thing) {
    for wall in map {
        draw_wall(&wall, window);
    }
    // As a result of player being the thing and the anchor
    // he's always drawn facing straight up and centered.
    // That's fine because the world rotates around him.
    draw_thing(window, player, player);
}