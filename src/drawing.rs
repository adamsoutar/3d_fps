use sfml::graphics::*;
use sfml::system::*;
use std::f32::consts::PI;

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;

pub fn draw_3d_map (window: &mut RenderWindow, map: &Vec<Sector>, player: &Thing) {
    // "To perspective project a co-ordinate, you simply take its x and y
    //  co-ordinate and divide them by the z co-ordinate"
    // tz1 = p1.y
    // tz2 = p2.y
    // tx1 = p1.x
    // tx2 = p2.x
    for sect in map {
        for v in 0..sect.vertices.len() {
            let mut p1 = sect.vertices[v];
            // Sectors loop around to their original vertex
            let mut p2 = if v != sect.vertices.len() - 1 {
                sect.vertices[v + 1]
            } else { sect.vertices[0] };

            // Transform map
            p1 = rotate_vec(p1 - player.pos, -player.rot);
            p2 = rotate_vec(p2 - player.pos, -player.rot);

            draw_wall(window, &p1, &p2, &sect, player);
        }
    }
}

fn draw_wall (window: &mut RenderWindow, px1: &Vector2f, px2: &Vector2f, sect: &Sector, player: &Thing) {
    let mut p1 = px1.clone();
    let mut p2 = px2.clone();

    if p1.y > 0. || p2.y  > 0. {
        // View frustum clipping
        let i1 = line_intersect(
            p1,
            p2,
            Vector2f::new(0., 0.),
            Vector2f::new(-1., 0.001));
        let i2 = line_intersect(
            p1,
            p2,
            Vector2f::new(0., 0.),
            Vector2f::new(1., 0.001));
        if p1.y <= 0. {
            if i1.y > 0. {
                p1 = i1
            } else {
                p1 = i2
            }
        }
        if p2.y <= 0. {
            if i1.y > 0. {
                p2 = i1;
            } else {
                p2 = i2;
            }
        }

        // Perspective projection
        let yceil = sect.ceil_height - player.zpos;
        let yfloor = sect.floor_height - player.zpos;

        let wz1 = yfloor * 1000.;
        let wz2 = yceil * 1000.;
        let wx1 = p1.x * XFOV / p1.y;
        let wx2 = p2.x * XFOV / p2.y;
        let wy1a = wz2 / p1.y;
        let wy1b = wz1 / p1.y;
        let wy2a = wz2 / p2.y;
        let wy2b = wz1 / p2.y;

        let top_left = Vector2f::new(wx1, wy1a);
        let top_right = Vector2f::new(wx2, wy2a);
        let bottom_right = Vector2f::new(wx2, wy2b);
        let bottom_left = Vector2f::new(wx1, wy1b);

        let to_top = HEIGHT as f32 / 2.;
        let ceil_left = Vector2f::new(top_left.x, to_top);
        let ceil_right = Vector2f::new(top_right.x, to_top);
        let floor_left = Vector2f::new(bottom_left.x, -to_top);
        let floor_right = Vector2f::new(bottom_right.x, -to_top);

        // Ceiling
        draw_quad(window, ceil_left, ceil_right, top_right, top_left, Color::WHITE);
        // Floor
        draw_quad(window, bottom_left, bottom_right, floor_right, floor_left, Color::BLUE);
        // Wall
        draw_quad(window, top_left, top_right, bottom_right, bottom_left, Color::YELLOW);
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