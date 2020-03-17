use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

mod vector_utils;
use vector_utils::*;
mod drawing;
use drawing::*;
mod constants;
use constants::*;
mod map;
use map::*;

fn main() {
    let mut window = RenderWindow::new(
    (WIDTH, HEIGHT),
    "3d-fps",
    Style::CLOSE,
    &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    // TODO (SOON): Load maps from files
    let map: Vec<Sector> = vec![
        // Spawn room
        Sector {
            sides: vec![
                Side {
                    p1: Vector2f::new(-256., 256.),
                    p2: Vector2f::new(78., 256.),
                    neighbour_sect: -1,
                    neighbour_side: -1
                },
                Side {
                    p1: Vector2f::new(78., 256.),
                    p2: Vector2f::new(178., 256.),
                    // This line is the portal to the corridor.
                    neighbour_sect: 1,
                    neighbour_side: 0
                },
                Side {
                    p1: Vector2f::new(178., 256.),
                    p2: Vector2f::new(256., 256.),
                    neighbour_sect: -1,
                    neighbour_side: -1
                },
                Side {
                    p1: Vector2f::new(256., 256.),
                    p2: Vector2f::new(256., -256.),
                    neighbour_sect: -1,
                    neighbour_side: -1
                },
                Side {
                    p1: Vector2f::new(256., -256.),
                    p2: Vector2f::new(-256., -256.),
                    neighbour_sect: -1,
                    neighbour_side: -1
                },
                Side {
                    p1: Vector2f::new(-256., -256.),
                    p2: Vector2f::new(-256., 256.),
                    neighbour_sect: -1,
                    neighbour_side: -1
                }
            ],
            ceil_height: 128.,
            floor_height: 0.
        },
        // The corridor
        Sector {
            sides: vec![
                Side {
                    p1: Vector2f::new(78., 256.),
                    p2: Vector2f::new(178., 256.),
                    neighbour_sect: 0,
                    neighbour_side: 1
                },
                Side {
                    p1: Vector2f::new(178., 256.),
                    p2: Vector2f::new(178., 768.),
                    neighbour_side: -1,
                    neighbour_sect: -1
                },
                Side {
                    p1: Vector2f::new(178., 768.),
                    p2: Vector2f::new(78., 768.),
                    neighbour_sect: -1,
                    neighbour_side: -1
                },
                Side {
                    p1: Vector2f::new(78., 768.),
                    p2: Vector2f::new(78., 256.),
                    neighbour_side: -1,
                    neighbour_sect: -1
                }
            ],
            floor_height: 16.,
            ceil_height: 100.
        }
    ];

    let mut player = Thing {
        pos: Vector2f::new(0., 0.),
        zpos: PLAYER_EYE_HEIGHT,
        falling: false,
        velocity: Vector3f::new(0.,0.,0.),
        rot: 0.,
        sector: 0
    };

    let mut clock = Clock::start();

    loop {
        let delta_time = clock.restart().as_seconds();
        // println!("{} FPS", 1. / delta_time);

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => {}
            }
        }

        process_movement(delta_time, &mut player, &map);

        window.clear(&Color::BLACK);
        draw_3d_map(&mut window, &map, &player);
        // draw_map(&mut window, &t_map, &player);
        window.display();
    }
}

/* Controls */
fn process_movement (delta_time: f32, player: &mut Thing, map: &Vec<Sector>) {
    // Forward, Backward, Strafe
    let mut movement = Vector2f::new(0., 0.);
    let mv = PLAYER_SPEED * delta_time;

    if Key::is_pressed(Key::W) {
        movement += Vector2f::new(0., mv);
    }
    if Key::is_pressed(Key::A) {
        movement += Vector2f::new(-mv, 0.);
    }
    if Key::is_pressed(Key::S) {
        movement += Vector2f::new(0., -mv);
    }
    if Key::is_pressed(Key::D) {
        movement += Vector2f::new(mv, 0.);
    }

    let mut rot_mov = rotate_vec(movement, player.rot);

    /* COLLISION DETECTION */
    // TODO: Could probably be a function on its own
    let sect = &map[player.sector];
    for s in 0..sect.sides.len() {
        let side = &sect.sides[s];

        // We'll cross the wall if we move
        let lsi = segment_intersection(&side.p1, &side.p2, &player.pos, &(player.pos + rot_mov));
        if lsi == SegmentIntersection::Intersection {
            if side.neighbour_sect == -1 {
                // TODO: Vector projection
                rot_mov = Vector2f::new(0., 0.);
                continue;
            }

            // It's a portal, so we might be moving sectors
            let nu = side.neighbour_sect as usize;
            let ns = &map[nu];
            let step = ns.floor_height - sect.floor_height;

            // We can't step that high
            if step > PLAYER_MAX_STEP_HEIGHT {
                // TODO: Vector projection
                rot_mov = Vector2f::new(0., 0.);
                continue;
            }

            // We're moving to that sector!
            player.sector = nu;
            if step < 0. { player.falling = true }
        }
    }
    /* END COLLISION DETECTION */

    player.pos += rot_mov;

    // Gravity
    if player.falling {
        player.velocity.z -= GRAVITY;
    }

    // Apply velocity
    let vel2d = Vector2f::new(player.velocity.x, player.velocity.y);
    player.pos += vel2d;
    player.zpos += player.velocity.z;

    // Landing
    if player.zpos < sect.floor_height + PLAYER_EYE_HEIGHT {
        player.zpos = sect.floor_height + PLAYER_EYE_HEIGHT;
        player.falling = false;
    }

    // Rotation
    let rot = PLAYER_ROT_SPEED * delta_time;
    let mut rt = 0.;
    if Key::is_pressed(Key::Left) {
        rt += -rot;
    }
    if Key::is_pressed(Key::Right) {
        rt += rot;
    }
    player.rot += rt;
}

