// TODO: THERE'S A BUG WITH VERTICAL MOUSELOOK THAT CRASHES THE GAME WHEN
//       YOU LOOK TOO HIGH. IT'S PROBABLY ALSO CAUSED BY A SECTOR THAT'S TOO HIGH UP

use sfml::graphics::*;
use sfml::system::*;
use std::cmp::{min, max, PartialOrd};

use crate::vector_utils::*;
use crate::constants::*;
use crate::map::*;
use crate::resource_pool::{ResourcePool, GameTexture};
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
            let (ums0, vms0) = (0, 0);
            let (ums1, vms1) = (mid_tex.width, mid_tex.height);
            let mut um0 = ums0 as f32;
            let vm0 = vms0 as f32;
            let mut um1 = ums1 as f32 - 1.;
            let vm1 = vms1 as f32 - 1.;

            let upper_tex = &resources.textures[&side.upper];
            let (uus0, vus0) = (0,0);
            let (uus1, vus1) = (upper_tex.width, upper_tex.height);
            let mut uu0 = uus0 as f32;
            let vu0 = vus0 as f32;
            let mut uu1 = uus1 as f32 - 1.;
            let vu1 = vus1 as f32 - 1.;

            let lower_tex = &resources.textures[&side.lower];
            let (uls0, vls0) = (0,0);
            let (uls1, vls1) = (lower_tex.width, lower_tex.height);
            let mut ul0 = uls0 as f32;
            let vl0 = vls0 as f32;
            let mut ul1 = uls1 as f32 - 1.;
            let vl1 = vls1 as f32 - 1.;

            let mut p1 = side.p1.clone();
            let mut p2 = side.p2.clone();
            // These pure versions won't be view frustum clipped
            let mut pp1 = side.p1.clone();
            let mut pp2 = side.p2.clone();

            // Rotate the map around the player
            p1 = rotate_vec(p1 - player.pos, -player.rot);
            p2 = rotate_vec(p2 - player.pos, -player.rot);
            pp1 = rotate_vec(pp1 - player.pos, -player.rot);
            pp2 = rotate_vec(pp2 - player.pos, -player.rot);

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

            // Frustum clip correction for texture mapping
            // From Bisqwit's code
            // https://bisqwit.iki.fi/jutut/kuvat/programming_examples/portalrendering.html
            if (p2.x - p1.x).abs() > (p2.y - p1.y).abs() {
                let c1 = (p1.x - pp1.x) / (pp2.x - pp1.x);
                let c2 = (p2.x - pp1.x) / (pp2.x - pp1.x);

                um0 = c1 * um1;
                um1 = c2 * um1;

                uu0 = c1 * uu1;
                uu1 = c2 * uu1;
            } else {
                let c1 = (p1.y - pp1.y) / (pp2.y - pp1.y);
                let c2 = (p2.y - pp1.y) / (pp2.y - pp1.y);

                um0 = c1 * um1;
                um1 = c2 * um1;

                uu0 = c1 * uu1;
                uu1 = c2 * uu1;
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
                    let uualpha = texmapping_calculation(alpha, uu0, uu1, z0, z1);
                    textured_line(upper_tex, x, cya, cnya - 1, ya, nya, uualpha, vu0, vu1, pixels);
                    // vline(x, cya, cnya - 1, upper_lower_colour, pixels);
                    ctoff.top = min(ctoff.top, min(cya, cnya));

                    // Lower
                    // vline(x, cnyb + 1, cyb, upper_lower_colour, pixels);
                    ctoff.bottom = clamp(min(cyb, cnyb), ctoff.bottom, 0);
                    ctoff.bottom = max(ctoff.bottom, max(cyb, cnyb));

                    // Don't draw the wall
                    continue;
                }

                // Render wall
                if DRAW_WALLS {
                    // vline(x, cya, cyb, col, pixels)
                    let umalpha = texmapping_calculation(alpha, um0, um1, z0, z1);
                    textured_line(mid_tex, x, cya, cyb, ya, yb, umalpha, vm0, vm1, pixels);
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

        // So we don't draw self-referential mirrors forever
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

pub fn textured_line (texture: &GameTexture, x: i64, start_y: i64, end_y: i64, real_sy: i64, real_ey: i64, ualpha: f32, v0: f32, v1: f32, pixels: &mut Vec<u8>) {
    let mut u = ualpha;
    let umax = texture.width;
    let ufmax = umax as f32;

    // Horizontal texture wrapping
    if u < 0. {
        return;
    }
    while u > ufmax { u -= ufmax };


    let uw = WIDTH as usize;
    let uh = HEIGHT as usize;

    let scrnx = (x + WIDTH as i64 / 2) as usize;

    let mut scrnys = (-start_y + HEIGHT as i64 / 2) as usize;
    let mut scrnye = (-end_y + HEIGHT as i64 / 2) as usize;
    scrnys = clamp(scrnys, 0, uh - 1);
    scrnye = clamp(scrnye, 0, uh - 1);

    let rys = (-real_sy + HEIGHT as i64 / 2) as i64;
    let rye = (-real_ey + HEIGHT as i64 / 2) as i64;

    if scrnx < 0 || scrnx >= uw || rye < 0 || rys >= HEIGHT as i64 {
        // It's offscreen
        return;
    }

    for y in scrnys..scrnye {
        let a = 1. - (y as i64 - rye) as f32 / (rys - rye) as f32;
        let v = v0 + (v1 - v0) * a;
        let uu = u as usize;
        let uv = v as usize;

        let i1 = y * uw * 4 + scrnx * 4;
        let i2 = uu * umax * 4 + uv * 4;

        pixels[i1] = texture.pixels[i2];
        pixels[i1 + 1] = texture.pixels[i2 + 1];
        pixels[i1 + 2] = texture.pixels[i2 + 2];
        pixels[i1 + 3] = texture.pixels[i2 + 3];
    }
}

pub fn vline (x: i64, start_y: i64, end_y: i64, colour: Color, pixels: &mut Vec<u8>) {
    // Unsupported to discourage me from fixing bugs in this function
    return;

    // Lines must be drawn top to bottom
    // if end_y > start_y { return }
    //
    // let uw = WIDTH as usize;
    // let uh = HEIGHT as usize;
    //
    // let mut scrnx = (x + WIDTH as i64 / 2) as usize;
    // let mut scrnys = (-start_y + HEIGHT as i64 / 2) as usize;
    // let mut scrnye = (-end_y + HEIGHT as i64 / 2) as usize;
    //
    // // TODO: Find out why we're getting draws above/below the screen
    // scrnye = clamp(scrnye, 0, uh);
    // scrnys = clamp(scrnys, 0, uh);
    // scrnx = clamp(scrnx, 0, uw);
    //
    // for y in scrnys..scrnye {
    //     pixels[y * uw * 4 + scrnx * 4] = colour.r;
    //     pixels[y * uw * 4 + scrnx * 4 + 1] = colour.g;
    //     pixels[y * uw * 4 + scrnx * 4 + 2] = colour.b;
    //     pixels[y * uw * 4 + scrnx * 4 + 3] = colour.a;
    // }
}

fn clamp<T:PartialOrd> (v: T, x: T, y: T) -> T {
    if v > y { return y }
    if v < x { return x }
    v
}