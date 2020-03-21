use sfml::graphics::*;
use sfml::system::*;
use std::cmp::{min, max, PartialOrd};

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;

pub struct Cutoffs {
    top: i64,
    bottom: i64
}

struct RenderQueueItem {
    sector_id: usize,
    c_left: i64,
    c_right: i64
}

pub fn draw_3d_map (window: &mut RenderWindow, map: &Vec<Sector>, player: &Thing) {
    let w = WIDTH as i64 / 2;
    let h = HEIGHT as i64 / 2;

    // TODO: Keep this in main and don't redeclare it every frame
    let mut offs: Vec<Cutoffs> = vec![];
    for _ in 0..WIDTH {
        offs.push(Cutoffs {
            top: h,
            bottom: -h
        })
    }

    draw_screen(window, &mut offs, map, player);
}

fn draw_screen (window: &mut RenderWindow, cutoffs: &mut Vec<Cutoffs>, map: &Vec<Sector>, player: &Thing) {
    // Render queue is used for drawing portals
    let w = WIDTH as i64 / 2;
    let h = WIDTH as i64 / 2 - 1;
    let mut render_queue = vec![RenderQueueItem {
        sector_id: player.sector,
        c_left: -w,
        c_right: w - 1
    }];
    let mut drawn: Vec<usize> = vec![];

    let zero = Vector2f::new(0., 0.);
    let frust1 = Vector2f::new(-1., 0.1);
    let frust2 = Vector2f::new(1., 0.1);

    let ceil_colour = Color::rgb(34, 34, 34);
    let floor_colour = Color::rgb(0, 10, 170);
    let wall_colour = Color::rgb(170, 170, 170);
    let upper_lower_colour = Color::rgb(132, 24, 216);
    let edge_colour = Color::rgb(0, 0, 0);

    while render_queue.len() > 0 {
        let now = render_queue.pop().unwrap();

        // We've already done this sector
        if drawn.contains(&now.sector_id) {
            continue;
        }

        let sect = &map[now.sector_id];

        // For each wall
        for side in &sect.sides {
            let mut p1 = side.p1.clone();
            let mut p2 = side.p2.clone();

            // Rotate the map around the player
            p1 = rotate_vec(p1 - player.pos, -player.rot);
            p2 = rotate_vec(p2 - player.pos, -player.rot);

            // The wall is behind us
            if p1.y <= 0. && p2.y <= 0. { continue }

            if p1.y <= 0. || p2.y <= 0. {
                // View frustum clipping
                let i1 = line_intersect(p1, p2, zero, frust1);
                let i2 = line_intersect(p1, p2, zero, frust2);
                if p1.y <= 0. {
                    if i1.y > 0. { p1 = i1 } else { p1 = i2 }
                }
                if p2.y <= 0. {
                    if i1.y > 0. { p2 = i1 } else { p2 = i2 }
                }
            }

            let yceil = sect.ceil_height - player.zpos;
            let yfloor = sect.floor_height - player.zpos;

            // Perspective calculations
            let z1 = yfloor * YFOV;
            let z2 = yceil * YFOV;
            let x1 = (p1.x * XFOV / p1.y) as i64;
            let x2 = (p2.x * XFOV / p2.y) as i64;
            let y1a = (z2 / p1.y) as i64;
            let y1b = (z1 / p1.y) as i64;
            let y2a = (z2 / p2.y) as i64;
            let y2b = (z1 / p2.y) as i64;

            // The cutoff renders this invisible
            if x1 >= x2 || x2 < now.c_left || x1 > now.c_right { continue }

            // If this is a portal, get these for upper & lower calculations
            let mut nyceil = 0.;
            let mut nyfloor = 0.;
            if side.neighbour != -1 {
                let nsct = &map[side.neighbour as usize];
                nyceil = nsct.ceil_height - player.zpos;
                nyfloor = nsct.floor_height - player.zpos;
            }

            let nz1 = nyfloor * YFOV;
            let nz2 = nyceil * YFOV;
            let ny1a = (nz2 / p1.y) as i64;
            let ny1b = (nz1 / p1.y) as i64;
            let ny2a = (nz2 / p2.y) as i64;
            let ny2b = (nz1 / p2.y) as i64;

            let beginx = max(x1, now.c_left);
            let endx = min(x2, now.c_right);

            for x in beginx..=endx {
                let mut ctoff = &mut cutoffs[(x + WIDTH as i64 / 2) as usize];
                let mut col = wall_colour;
                if x == beginx || x == endx {
                    col = edge_colour
                }

                let ya = (x - x1) * (y2a - y1a) / (x2 - x1) + y1a;
                let yb = (x - x1) * (y2b - y1b) / (x2 - x1) + y1b;
                let cya = clamp(ya, ctoff.bottom, ctoff.top);
                let cyb = clamp(yb, ctoff.bottom, ctoff.top);

                // Render ceiling
                if DRAW_CEILINGS { vline(window, x, ctoff.top, cya - 1, ceil_colour) }
                // Render floor
                if DRAW_FLOORS { vline(window, x, cyb + 1, ctoff.bottom, floor_colour) }

                if side.neighbour != -1 {
                    // We potentially have uppers/lowers
                    let nya = (x - x1) * (ny2a - ny1a) / (x2 - x1) + ny1a;
                    let nyb = (x - x1) * (ny2b - ny1b) / (x2 - x1) + ny1b;
                    let cnya = clamp(nya, ctoff.bottom, ctoff.top);
                    let cnyb = clamp(nyb, ctoff.bottom, ctoff.top);

                    // Upper
                    vline(window, x, cya, cnya - 1, upper_lower_colour);
                    ctoff.top = min(ctoff.top, min(cya, cnya));

                    // Lower
                    vline(window, x, cnyb + 1, cyb, upper_lower_colour);
                    ctoff.bottom = clamp(min(cyb, cnyb), ctoff.bottom, 0);
                    ctoff.bottom = max(ctoff.bottom, max(cyb, cnyb));

                    // Don't draw the wall
                    continue;
                }

                // Render wall
                if DRAW_WALLS { vline(window, x, cya, cyb, col) }
            }

            if side.neighbour != -1 && endx >= beginx {
                render_queue.push(RenderQueueItem {
                    sector_id: side.neighbour as usize,
                    c_left: beginx,
                    c_right: endx
                })
            }
        }

        // So we don't draw self-refferential mirrors forever
        drawn.push(now.sector_id);
        if drawn.len() >= MAX_SECTOR_DRAWS {
            return;
        }
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

pub fn vline (window: &mut RenderWindow, x: i64, start_y: i64, end_y: i64, colour: Color) {
    // Lines must be drawn top to bottom
    if end_y > start_y { return }

    let mut va = VertexArray::default();
    va.set_primitive_type(PrimitiveType::Lines);
    va.append(&Vertex::with_pos_color(sfml_vec(Vector2f::new(x as f32, start_y as f32)), colour));
    va.append(&Vertex::with_pos_color(sfml_vec(Vector2f::new(x as f32, end_y as f32)), colour));
    window.draw(&va);
}

fn clamp<T:PartialOrd> (v: T, x: T, y: T) -> T {
    if v > y { return y }
    if v < x { return x }
    v
}