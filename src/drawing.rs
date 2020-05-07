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

pub fn draw_3d_map (window: &mut RenderWindow, resources: &ResourcePool, map: &Vec<Sector>, player: &Thing, cutoffs: &mut Vec<Cutoffs>, pixels: &mut Box<[u8]>) {
    draw_screen(window, resources, cutoffs, map, player, pixels);
}

fn draw_screen (window: &mut RenderWindow, resources: &ResourcePool, cutoffs: &mut Vec<Cutoffs>, map: &Vec<Sector>, player: &Thing, pixels: &mut Box<[u8]>) {
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
        let ceil_tex = &resources.textures[&sect.ceil_texture];
        let floor_tex = &resources.textures[&sect.floor_texture];

        // For each wall
        for side in &sect.sides {
            // TODO: Texture offsets
            let mid_tex = &resources.textures[&side.mid];
            let upper_tex = &resources.textures[&side.upper];
            let lower_tex = &resources.textures[&side.lower];

            let mut u0 = 0.;
            // Length of the side
            let mut u1 = mag(&(side.p1.clone() - side.p2.clone()));

            let mut p1 = side.p1.clone();
            let mut p2 = side.p2.clone();
            // These pure versions won't be view frustum clipped
            let mut pp1 = side.p1.clone();
            let mut pp2 = side.p2.clone();

            // Rotate the map around the player
            p1 = rotate_around_player(p1 - player.pos, player, false);
            p2 = rotate_around_player(p2 - player.pos, player, false);
            pp1 = rotate_around_player(pp1 - player.pos, player, false);
            pp2 = rotate_around_player(pp2 - player.pos, player, false);

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

                u0 = c1 * u1;
                u1 = c2 * u1;
            } else {
                let c1 = (p1.y - pp1.y) / (pp2.y - pp1.y);
                let c2 = (p2.y - pp1.y) / (pp2.y - pp1.y);

                u0 = c1 * u1;
                u1 = c2 * u1;
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
                let ualpha = texmapping_calculation(alpha, u0, u1, z0, z1);

                let ya = (x - x1) * (y2a - y1a) / (x2 - x1) + y1a;
                let yb = (x - x1) * (y2b - y1b) / (x2 - x1) + y1b;
                let cya = clamp(ya, ctoff.bottom, ctoff.top);
                let cyb = clamp(yb, ctoff.bottom, ctoff.top);

                // Render ceiling
                if DRAW_CEILINGS {
                    depth_textured_line(ceil_tex, x, ctoff.top, cya - 1, pixels, sect.ceil_height, player);
                    // vline(x, ctoff.top, cya - 1, ceil_colour, pixels)
                }
                // Render floor
                if DRAW_FLOORS {
                    depth_textured_line(floor_tex, x, cyb + 1, ctoff.bottom, pixels, sect.floor_height, player);
                    // vline(x, cyb + 1, ctoff.bottom, floor_colour, pixels)
                }


                if side.neighbour != -1 {
                    // We potentially have uppers/lowers
                    let nya = (x - x1) * (ny2a - ny1a) / (x2 - x1) + ny1a;
                    let nyb = (x - x1) * (ny2b - ny1b) / (x2 - x1) + ny1b;
                    let cnya = clamp(nya, ctoff.bottom, ctoff.top);
                    let cnyb = clamp(nyb, ctoff.bottom, ctoff.top);

                    // Upper
                    let upper_height = (sect.ceil_height - nyceil).abs();
                    textured_line(upper_tex, x, cya, cnya - 1, ya, nya, ualpha, pixels, upper_height);
                    ctoff.top = min(ctoff.top, min(cya, cnya));

                    // Lower
                    let lower_height = (sect.floor_height - nyfloor).abs();
                    textured_line(lower_tex, x, cnyb + 1, cyb, nyb + 1, yb, ualpha, pixels, lower_height);
                    ctoff.bottom = clamp(min(cyb, cnyb), ctoff.bottom, 0);
                    ctoff.bottom = max(ctoff.bottom, max(cyb, cnyb));

                    // Don't draw the wall
                    continue;
                }

                // Render wall
                if DRAW_WALLS {
                    let sec_height = sect.ceil_height - sect.floor_height;
                    textured_line(mid_tex, x, cya, cyb, ya, yb, ualpha, pixels, sec_height);
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

fn screen_to_world_pos (v: &Vector2f, z: f32, player: &Thing) -> Vector2f {
    let y = (YFOV * z - YFOV * player.zpos) / (v.y - YFOV * player.yaw);
    let x = (v.x * y) / XFOV;
    let v = rotate_around_player(Vector2f::new(x, y), player, true);
    v + player.pos
}

pub fn depth_textured_line (texture: &GameTexture, x: i64, start_y: i64, end_y: i64, pixels: &mut Box<[u8]>, hei: f32, player: &Thing) {
    let ih = HEIGHT as i64;
    let mut sys = -start_y + ih / 2;
    let mut sye = -end_y + ih / 2;
    sys = clamp(sys, 0, ih - 1);
    sye = clamp(sye, 0, ih - 1);

    let usx = (x + WIDTH as i64 / 2) as usize;
    let usys = sys as usize;
    let usye = sye as usize;
    let uw = WIDTH as usize;

    let h = HEIGHT as f32 / 2.;

    // Cache a bunch of stuff for performance
    let mut v = Vector2f::new(x as f32, -(usys as f32 - h));
    let th = texture.height;
    let tw = texture.width;

    for y in usys..usye {
        v.y -= 1.;
        let m = screen_to_world_pos(&v, hei, player);
        
        let mut ty = m.y as usize % th;
        let mut tx = m.x as usize % tw;

        let i1 = y * uw * 4 + usx * 4;
        let i2 = ty * texture.width * 4 + tx * 4;

        pixels[i1] = texture.pixels[i2];
        pixels[i1 + 1] = texture.pixels[i2 + 1];
        pixels[i1 + 2] = texture.pixels[i2 + 2];
        pixels[i1 + 3] = texture.pixels[i2 + 3];
    }
}

// Dude this function takes too many params
pub fn textured_line (texture: &GameTexture, x: i64, start_y: i64, end_y: i64, real_sy: i64, real_ey: i64, ualpha: f32, pixels: &mut Box<[u8]>, wall_height: f32) {
    let mut u = ualpha;
    let umax = texture.width;
    let ufmax = umax as f32;

    // Horizontal texture wrapping
    if u < 0. {
        return;
    }
    u %= ufmax;

    let uw = WIDTH as usize;
    let uh = HEIGHT as usize;

    let scrnx = (x + WIDTH as i64 / 2) as usize;

    let mut sys = -start_y + HEIGHT as i64 / 2;
    let mut sye = -end_y + HEIGHT as i64 / 2;
    sys = clamp(sys, 0, HEIGHT as i64 - 1);
    sye = clamp(sye, 0, HEIGHT as i64 - 1);

    let scrnys = sys as usize;
    let scrnye = sye as usize;

    let rys = (-real_sy + HEIGHT as i64 / 2) as i64;
    let rye = (-real_ey + HEIGHT as i64 / 2) as i64;

    if scrnx >= uw || rye < 0 || rys >= HEIGHT as i64 {
        // It's offscreen
        return;
    }

    for y in scrnys..scrnye {
        let a = 1. - (y as i64 - rye) as f32 / (rys - rye) as f32;
        let v = wall_height * a;

        if a > 1. {
            // This happens with portals above the player's view
            // it's not an issue.
            continue;
        }

        let uu = u as usize;
        let mut uv = v as usize;

        // Wrapping
        uv %= texture.height;

        let i1 = y * uw * 4 + scrnx * 4;
        let i2 = uv * umax * 4 + uu * 4;

        pixels[i1] = texture.pixels[i2];
        pixels[i1 + 1] = texture.pixels[i2 + 1];
        pixels[i1 + 2] = texture.pixels[i2 + 2];
        pixels[i1 + 3] = texture.pixels[i2 + 3];
    }
}

fn clamp<T:PartialOrd> (v: T, x: T, y: T) -> T {
    if v > y { return y }
    if v < x { return x }
    v
}