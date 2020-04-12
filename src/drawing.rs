use sfml::graphics::*;
use sfml::system::*;
use std::cmp::{min, max, PartialOrd};

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;
use crate::resource_pool::ResourcePool;
use image::RgbaImage;

#[derive(Clone)]
pub struct Cutoffs {
    pub top: i64,
    pub bottom: i64
}

struct RenderQueueItem {
    sector_id: usize,
    c_left: i64,
    c_right: i64
}

pub fn draw_3d_map (window: &mut RenderWindow, resources: &ResourcePool, map: &Vec<Sector>, player: &Thing, cutoffs: &mut Vec<Cutoffs>, pixels: &mut Vec<u8>) {
    draw_screen(window, resources, cutoffs, map, player, pixels);
}

fn draw_screen (window: &mut RenderWindow, resources: &ResourcePool, cutoffs: &mut Vec<Cutoffs>, map: &Vec<Sector>, player: &Thing, pixels: &mut Vec<u8>) {
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

    let yaw = |y, z| {
        y + z * player.yaw
    };

    while render_queue.len() > 0 {
        let now = render_queue.pop().unwrap();

        // We've already done this sector
        if drawn.contains(&now.sector_id) {
            continue;
        }

        let sect = &map[now.sector_id];

        // For each wall
        for side in &sect.sides {
            let mid_tex = &resources.textures[&side.mid];
            // TODO: Texture offsets
            let (us0, vs0) = (0, 0);
            let (us1, vs1) = mid_tex.dimensions();
            // let (us1, vs1) = (1, 1);
            let u0 = us0 as f32;
            let v0 = vs0 as f32;
            let u1 = us1 as f32 - 1.;
            let v1 = vs1 as f32 - 1.;

            // TODO
            let upper_tex = &resources.textures[&side.upper];
            let lower_tex = &resources.textures[&side.lower];

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
            let x1 = (p1.x * XFOV / p1.y) as i64;
            let x2 = (p2.x * XFOV / p2.y) as i64;

            // The cutoff renders this invisible
            if x1 >= x2 || x2 < now.c_left || x1 > now.c_right { continue }

            let yscale1 = YFOV / p1.y;
            let yscale2 = YFOV / p2.y;
            let y1a = (yaw(yceil, p1.y) * yscale1) as i64;
            let y1b = (yaw(yfloor, p1.y) * yscale1) as i64;
            let y2a = (yaw(yceil, p2.y) * yscale2) as i64;
            let y2b = (yaw(yfloor, p2.y) * yscale2) as i64;


            // If this is a portal, get these for upper & lower calculations
            let mut nyceil = 0.;
            let mut nyfloor = 0.;
            if side.neighbour != -1 {
                let nsct = &map[side.neighbour as usize];
                nyceil = nsct.ceil_height - player.zpos;
                nyfloor = nsct.floor_height - player.zpos;
            }

            let ny1a = (yaw(nyceil, p1.y) * yscale1) as i64;
            let ny1b = (yaw(nyfloor, p1.y) * yscale1) as i64;
            let ny2a = (yaw(nyceil, p2.y) * yscale2) as i64;
            let ny2b = (yaw(nyfloor, p2.y) * yscale2) as i64;

            let beginx = max(x1, now.c_left);
            let endx = min(x2, now.c_right);

            for x in beginx..=endx {
                let mut ctoff = &mut cutoffs[(x + WIDTH as i64 / 2) as usize];
                let mut col = wall_colour;
                if x == beginx || x == endx {
                    col = edge_colour
                }

                let z0 = p1.y;
                let z1 = p2.y;
                let alpha = ((x - x1) as f32 / (x2 - x1) as f32);

                let ya = (x - x1) * (y2a - y1a) / (x2 - x1) + y1a;
                let yb = (x - x1) * (y2b - y1b) / (x2 - x1) + y1b;
                let cya = clamp(ya, ctoff.bottom, ctoff.top);
                let cyb = clamp(yb, ctoff.bottom, ctoff.top);

                // Render ceiling
                if DRAW_CEILINGS { vline(x, ctoff.top, cya - 1, ceil_colour, pixels) }
                // Render floor
                if DRAW_FLOORS { vline(x, cyb + 1, ctoff.bottom, floor_colour, pixels) }

                if side.neighbour != -1 {
                    // We potentially have uppers/lowers
                    let nya = (x - x1) * (ny2a - ny1a) / (x2 - x1) + ny1a;
                    let nyb = (x - x1) * (ny2b - ny1b) / (x2 - x1) + ny1b;
                    let cnya = clamp(nya, ctoff.bottom, ctoff.top);
                    let cnyb = clamp(nyb, ctoff.bottom, ctoff.top);

                    // Upper
                    vline(x, cya, cnya - 1, upper_lower_colour, pixels);
                    ctoff.top = min(ctoff.top, min(cya, cnya));

                    // Lower
                    vline(x, cnyb + 1, cyb, upper_lower_colour, pixels);
                    ctoff.bottom = clamp(min(cyb, cnyb), ctoff.bottom, 0);
                    ctoff.bottom = max(ctoff.bottom, max(cyb, cnyb));

                    // Don't draw the wall
                    continue;
                }


                let ualpha = texmapping_calculation(alpha, u0, u1, z0, z1);

                // Render wall
                if DRAW_WALLS {
                    // vline(x, cya, cyb, col, pixels)
                    textured_line(mid_tex, x, cya, cyb, ya, yb, ualpha, v0, v1, pixels);
                }
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

fn texmapping_calculation (alpha: f32, u0: f32, u1: f32, z0: f32, z1: f32) -> f32 {
    let numerator = (1. - alpha) * u0 / z0 + alpha * u1 / z1;
    let denominator = (1. - alpha) * 1. / z0 + alpha * 1. / z1;
    numerator / denominator
}

// Perhaps useful for medpacks, players, other sprites etc.
fn world_to_screen_pos (v: Vector3f, player: &Thing) -> Vector2f {
    let p = rotate_vec(Vector2f::new(v.x, v.y) - player.pos, -player.rot);
    let x = p.x * XFOV / p.y;
    let y = (v.z - player.zpos) * YFOV / p.y;
    Vector2::new(x, y)
}

pub fn textured_line (texture: &RgbaImage, x: i64, start_y: i64, end_y: i64, real_sy: i64, real_ey: i64, ualpha: f32, v0: f32, v1: f32, pixels: &mut Vec<u8>) {
    let mut u = ualpha;
    let (umax, _) = texture.dimensions();
    let ufmax = umax as f32;

    // Horizontal texture wrapping
    while u > ufmax { u -= ufmax };

    let uw = WIDTH as usize;
    let scrnx = (x + WIDTH as i64 / 2) as usize;
    let scrnys = (-start_y + HEIGHT as i64 / 2) as usize;
    let scrnye = (-end_y + HEIGHT as i64 / 2) as usize;
    let rys = (-real_sy + HEIGHT as i64 / 2) as i64;
    let rye = (-real_ey + HEIGHT as i64 / 2) as i64;

    for y in scrnys..scrnye {
        let a = 1. - (y as i64 - rye) as f32 / (rys - rye) as f32;
        let v = v0 + (v1 - v0) * a;
        let c = texture.get_pixel(u as u32, v as u32).0;

        pixels[y * uw * 4 + scrnx * 4] = c[0];
        pixels[y * uw * 4 + scrnx * 4 + 1] = c[1];
        pixels[y * uw * 4 + scrnx * 4 + 2] = c[2];
        pixels[y * uw * 4 + scrnx * 4 + 3] = c[3];
    }
}

pub fn vline (x: i64, start_y: i64, end_y: i64, colour: Color, pixels: &mut Vec<u8>) {
    // Lines must be drawn top to bottom
    if end_y > start_y { return }

    let uw = WIDTH as usize;
    let scrnx = (x + WIDTH as i64 / 2) as usize;
    let scrnys = (-start_y + HEIGHT as i64 / 2) as usize;
    let scrnye = (-end_y + HEIGHT as i64 / 2) as usize;

    for y in scrnys..scrnye {
        pixels[y * uw * 4 + scrnx * 4] = colour.r;
        pixels[y * uw * 4 + scrnx * 4 + 1] = colour.g;
        pixels[y * uw * 4 + scrnx * 4 + 2] = colour.b;
        pixels[y * uw * 4 + scrnx * 4 + 3] = colour.a;
    }
}

fn clamp<T:PartialOrd> (v: T, x: T, y: T) -> T {
    if v > y { return y }
    if v < x { return x }
    v
}