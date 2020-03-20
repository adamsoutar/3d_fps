use sfml::graphics::*;
use sfml::system::*;
use std::cmp::{min, max, PartialOrd};

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;
use std::io::Lines;

pub struct Cutoffs {
    top: f32,
    bottom: f32
}

pub fn draw_3d_map (window: &mut RenderWindow, map: &Vec<Sector>, player: &Thing) {
    let w = WIDTH as f32 / 2.;
    let h = HEIGHT as f32 / 2.;

    // TODO: Keep this in main and don't redeclare it every frame
    let mut offs: Vec<Cutoffs> = vec![];
    for _ in 0..WIDTH {
        offs.push(Cutoffs {
            top: h,
            bottom: -h
        })
    }

    draw_sector(window, map, player.sector, player, -w, w, &mut offs, 0);
}

fn draw_sector (window: &mut RenderWindow, map: &Vec<Sector>, sect_id: usize, player: &Thing, clip_left: f32, clip_right: f32, cutoffs: &mut Vec<Cutoffs>, recursion: usize) {
    if recursion >= MAX_PORTAL_DRAWS {
        return;
    }

    let sect = &map[sect_id];
    for s in 0..sect.sides.len() {
        let side = &sect.sides[s];
        let mut p1 = side.p1.clone();
        let mut p2 = side.p2.clone();

        // Transform map
        p1 = rotate_vec(p1 - player.pos, -player.rot);
        p2 = rotate_vec(p2 - player.pos, -player.rot);

        draw_wall(window, &p1, &p2, map, sect_id, side, player, clip_left, clip_right, cutoffs, recursion);
    }
}

fn draw_wall (
    window: &mut RenderWindow,
    px1: &Vector2f, px2: &Vector2f,
    map: &Vec<Sector>, sect_id: usize,
    side: &Side, player: &Thing,
    clip_left: f32, clip_right: f32,
    cutoffs: &mut Vec<Cutoffs>,
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
        draw_quad(window, ceil_left, ceil_right, top_right, top_left, Color::rgb(34, 34, 34), clip_left, clip_right, cutoffs);
        // Floor
        draw_quad(window, bottom_left, bottom_right, floor_right, floor_left, Color::rgb(0, 10, 170), clip_left, clip_right, cutoffs);

        // Don't draw walls over portals
        if side.neighbour_sect != -1 {
            // Uppers and lowers
            let n = &map[side.neighbour_sect as usize];
            let f_diff = n.floor_height - sect.floor_height;
            let c_diff = sect.ceil_height - n.ceil_height;
            let mut p_tl = top_left.clone();
            let mut p_tr = top_right.clone();
            let mut p_bl = bottom_left.clone();
            let mut p_br = bottom_right.clone();

            if c_diff > 0. {
                // We should draw an upper
                let t_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.ceil_height), player);
                let t_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.ceil_height), player);
                let b_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.ceil_height - c_diff), player);
                let b_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.ceil_height - c_diff), player);
                draw_quad(window, t_l, t_r, b_r, b_l, Color::rgb(132, 24, 216), clip_left, clip_right, cutoffs);

                p_tl = b_l;
                p_tr = b_r;
            }

            if f_diff > 0. {
                // We should draw a lower
                let t_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.floor_height + f_diff), player);
                let t_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.floor_height + f_diff), player);
                let b_r = raw_screen_pos(Vector3f::new(p2.x, p2.y, sect.floor_height), player);
                let b_l = raw_screen_pos(Vector3f::new(p1.x, p1.y, sect.floor_height), player);
                draw_quad(window, t_l, t_r, b_r, b_l, Color::rgb(132, 24, 216), clip_left, clip_right, cutoffs);

                p_bl = t_l;
                p_br = t_r;
            }

            fill_quad_cutoffs(p_tl, p_tr, p_br, p_bl, cutoffs);

            // Draw portal in the mid
            // TODO: Can have a texture over a portal hole?
            draw_sector(window, map, side.neighbour_sect as usize, player, top_left.x, top_right.x, cutoffs, recursion + 1);
            return;
        }

        // Wall
        draw_quad(window, top_left, top_right, bottom_right, bottom_left, Color::rgb(170, 170, 170), clip_left, clip_right, cutoffs);
    }
}

fn fill_quad_cutoffs (top_left: Vector2f, top_right: Vector2f, bottom_right: Vector2f, bottom_left: Vector2f, cutoffs: &mut Vec<Cutoffs>) {
    let w = WIDTH as i64 / 2;

    let x1 = top_left.x;
    let x2 = top_right.x;
    let y1a = top_left.y;
    let y1b = bottom_left.y;
    let y2a = top_right.y;
    let y2b = bottom_right.y;

    let mut sx = x1 as i64 + w;
    let mut ex = x2 as i64 + w;
    sx = clamp(sx, 0, WIDTH as i64 - 1);
    ex = clamp(ex, 0, WIDTH as i64 - 1);

    for i in sx..=ex {
        let fi = i as f32;
        let ya = (fi - x1) * (y2a - y1a) / (x2 - x1) + y1a;
        let yb = (fi - x1) * (y2b - y1b) / (x2 - x1) + y1b;

        cutoffs[i as usize].top = ya;
        cutoffs[i as usize].bottom = yb;
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
    clip_left: f32, clip_right: f32,
    cutoffs: &mut Vec<Cutoffs>
) {
    let startx = clip_left as i64;
    let endx = clip_right as i64;

    let x1 = top_left.x;
    let x2 = top_right.x;
    let y1a = top_left.y;
    let y1b = bottom_left.y;
    let y2a = top_right.y;
    let y2b = bottom_right.y;

    let w = WIDTH as i64 / 2;

    let mut begin = max(x1 as i64, startx);
    let mut end = min(x2 as i64, endx);
    begin = clamp(begin, -w, w - 1);
    end = clamp(end, -w, w - 1);

    for x in begin..=end {
        let fx = x as f32;
        let ya = (fx - x1) * (y2a - y1a) / (x2 - x1) + y1a;
        let yb = (fx - x1) * (y2b - y1b) / (x2 - x1) + y1b;

        let starty = cutoffs[(x + w) as usize].top;
        let endy = cutoffs[(x + w) as usize].bottom;

        let cya = clamp(ya, endy as f32, starty as f32);
        let cyb = clamp(yb, endy as f32, starty as f32);

        vline(window, fx, cya, cyb, colour);
    }
}

fn clamp<T:PartialOrd> (v: T, x: T, y: T) -> T {
    if v > y { return y }
    if v < x { return x }
    v
}