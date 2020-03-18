use sfml::graphics::*;
use sfml::system::*;
use std::cmp::{min, max};

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;
use std::io::Lines;

pub fn draw_3d_map (window: &mut RenderWindow, map: &Vec<Sector>, player: &Thing) {
    let w = WIDTH as i64 / 2;
    let h = HEIGHT as i64 / 2;
    draw_sector(window, map, player.sector, player, -w, w, -h, h, 0);
}

fn draw_sector (window: &mut RenderWindow, map: &Vec<Sector>, sect_id: usize, player: &Thing, sx: i64, ex: i64, sy: i64, ey: i64, recursion: usize) {
    if recursion >= MAX_PORTAL_DRAWS {
        return;
    }

    let sect = &map[sect_id];
    for side in &sect.sides {
        let mut p1 = side.p1.clone();
        let mut p2 = side.p2.clone();

        // Transform map
        p1 = rotate_vec(p1 - player.pos, -player.rot);
        p2 = rotate_vec(p2 - player.pos, -player.rot);

        draw_wall(window, &p1, &p2, map, sect_id, side, player, sx, ex, sy, ey, recursion);
    }
}

fn draw_wall (
    window: &mut RenderWindow,
    px1: &Vector2f, px2: &Vector2f,
    map: &Vec<Sector>, sect_id: usize,
    side: &Side, player: &Thing,
    startx: i64, endx: i64,
    starty: i64, endy: i64,
    recursion: usize
) {
    let mut p1 = px1.clone();
    let mut p2 = px2.clone();
    let sect = &map[sect_id];

    if p1.y > 0. || p2.y  > 0. {
        // View frustum clipping
        let i1 = line_intersect(
            p1,
            p2,
            Vector2f::new(0., 0.),
            Vector2f::new(-1., 0.1));
        let i2 = line_intersect(
            p1,
            p2,
            Vector2f::new(0., 0.),
            Vector2f::new(1., 0.1));
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

        // Ceiling
        draw_quad(window, ceil_left, ceil_right, top_right, top_left, Color::rgb(34, 34, 34), startx, endx, starty, endy);
        // Floor
        draw_quad(window, bottom_left, bottom_right, floor_right, floor_left, Color::rgb(0, 10, 170), startx, endx, starty, endy);

        // Don't draw walls over portals
        if side.neighbour_sect != -1 {
            // Uppers and lowers
            let n = &map[side.neighbour_sect as usize];
            let f_diff = n.floor_height - sect.floor_height;
            let c_diff = sect.ceil_height - n.ceil_height;

            if c_diff > 0. {
                // We should draw an upper
                let t_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.ceil_height), player);
                let t_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.ceil_height), player);
                let b_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.ceil_height - c_diff), player);
                let b_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.ceil_height - c_diff), player);
                draw_quad(window, t_l, t_r, b_r, b_l, Color::rgb(132, 24, 216), startx, endx, starty, endy);
            }

            if f_diff > 0. {
                // We should draw a lower
                let t_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.floor_height + f_diff), player);
                let t_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.floor_height + f_diff), player);
                let b_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.floor_height), player);
                let b_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.floor_height), player);
                draw_quad(window, t_l, t_r, b_r, b_l, Color::rgb(132, 24, 216), startx, endx, starty, endy);
            }

            // Draw portal in the mid
            // TODO: Can have a texture over a portal hole?

            draw_sector(window, map, side.neighbour_sect as usize, player, startx, endx, starty, endy, recursion + 1);
            return;
        }

        // Wall
        draw_quad(window, top_left, top_right, bottom_right, bottom_left, Color::rgb(170, 170, 170), startx, endx, starty, endy);
    }
}

fn raw_screen_pos (v: Vector3f, player: &Thing) -> Vector2f {
    let p = Vector2f::new(v.x, v.y);
    let x = p.x * XFOV / p.y;
    let y = (v.z - player.zpos) * YFOV / p.y;
    Vector2::new(x, y)
}

fn world_to_screen_pos (v: Vector3f, player: &Thing) -> Vector2f {
    let p = rotate_vec(Vector2f::new(v.x, v.y) - player.pos, -player.rot);
    let x = p.x * XFOV / p.y;
    let y = (v.z - player.zpos) * YFOV / p.y;
    Vector2::new(x, y)
}

pub fn vline (window: &mut RenderWindow, x: f32, startY: f32, endY: f32, colour: Color) {
    let mut va = VertexArray::default();
    va.set_primitive_type(PrimitiveType::Lines);
    va.append(&Vertex::with_pos_color(sfml_vec(Vector2f::new(x, startY)), colour));
    va.append(&Vertex::with_pos_color(sfml_vec(Vector2f::new(x, endY)), colour));
    window.draw(&va);
}

pub fn draw_quad (
    window: &mut RenderWindow,
    top_left: Vector2f, top_right: Vector2f,
    bottom_right: Vector2f, bottom_left: Vector2f,
    colour: Color,
    startx: i64, endx: i64,
    starty: i64, endy: i64
) {
    let x1 = top_left.x;
    let x2 = top_right.x;
    let y1a = top_left.y;
    let y1b = bottom_left.y;
    let y2a = top_right.y;
    let y2b = bottom_right.y;
    let begin = max(x1 as i64, startx);
    let end = min(x2 as i64, endx);
    for x in begin..=end {
        let fx = x as f32;
        let ya = (fx - x1) * (y2a - y1a) / (x2 - x1) + y1a;
        let yb = (fx - x1) * (y2b - y1b) / (x2 - x1) + y1b;
        let cya = clamp(ya, starty as f32, endy as f32);
        let cyb = clamp(yb, starty as f32, endy as f32);
        vline(window, fx, cya, cyb, colour);
    }

    /*let mut vertex_array = VertexArray::default();
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

    window.draw(&vertex_array);*/
}

fn clamp (v: f32, x: f32, y: f32) -> f32 {
    if v > y { return y }
    if v < x { return x }
    v
}