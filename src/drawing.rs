use sfml::graphics::*;
use sfml::system::*;
use std::f32::consts::PI;

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;

pub fn draw_3d_map (window: &mut RenderWindow, map: &Vec<Sector>, player: &Thing) {
    // Traverse sectors, then draw them backwards "on top of each other"
    // to create see-through-able portals
    let mut portal_stack: Vec<&Sector> = vec![];
    portal_stack.push(&map[0]);

    for _ in 0..portal_stack.len() {
        draw_sector(window, portal_stack.pop().unwrap(), player);
    }
}

fn draw_sector (window: &mut RenderWindow, sect: &Sector, player: &Thing) {
    for side in &sect.sides {
        let mut p1 = side.p1.clone();
        let mut p2 = side.p2.clone();

        // Transform map
        p1 = rotate_vec(p1 - player.pos, -player.rot);
        p2 = rotate_vec(p2 - player.pos, -player.rot);

        draw_wall(window, &p1, &p2, &sect, player);
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
            Vector2f::new(-1., 0.00001));
        let i2 = line_intersect(
            p1,
            p2,
            Vector2f::new(0., 0.),
            Vector2f::new(1., 0.00001));
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

        let wz1 = yfloor * YFOV;
        let wz2 = yceil * YFOV;
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

        let to_top = HEIGHT as f32;
        let ceil_left = Vector2f::new(top_left.x, top_left.y + to_top);
        let ceil_right = Vector2f::new(top_right.x, top_right.y + to_top);
        let floor_left = Vector2f::new(bottom_left.x, bottom_left.y - to_top);
        let floor_right = Vector2f::new(bottom_right.x, bottom_right.y - to_top);

        // Wall
        draw_quad(window, top_left, top_right, bottom_right, bottom_left, Color::YELLOW);
        // Ceiling
        draw_quad(window, ceil_left, ceil_right, top_right, top_left, Color::WHITE);
        // Floor
        draw_quad(window, bottom_left, bottom_right, floor_right, floor_left, Color::BLUE);
    }
}

pub fn draw_quad (window: &mut RenderWindow, top_left: Vector2f, top_right: Vector2f, bottom_right: Vector2f, bottom_left: Vector2f, colour: Color) {
    let mut vertex_array = VertexArray::default();
    vertex_array.set_primitive_type(PrimitiveType::TriangleStrip);

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