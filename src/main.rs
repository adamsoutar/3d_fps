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
        "3d_fps",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(false);
    window.set_mouse_cursor_visible(false);
    // window.set_framerate_limit(60);

    // TODO (SOON): Load maps from files
    let map: Vec<Sector> = vec![
        // Spawn room
        Sector {
            sides: vec![
                Side {
                    p1: Vector2f::new(-256., 256.),
                    p2: Vector2f::new(78., 256.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(78., 256.),
                    p2: Vector2f::new(178., 256.),
                    // This line is the portal to the corridor.
                    neighbour: 1
                },
                Side {
                    p1: Vector2f::new(178., 256.),
                    p2: Vector2f::new(256., 256.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(256., 256.),
                    p2: Vector2f::new(256., -256.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(256., -256.),
                    p2: Vector2f::new(-256., -256.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(-256., -256.),
                    p2: Vector2f::new(-256., 256.),
                    neighbour: -1
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
                    p2: Vector2f::new(78., 768.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(78., 768.),
                    p2: Vector2f::new(178., 768.),
                    neighbour: 2
                },
                Side {
                    p1: Vector2f::new(178., 768.),
                    p2: Vector2f::new(178., 256.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(178., 256.),
                    p2: Vector2f::new(78., 256.),
                    neighbour: 0
                }
            ],
            floor_height: 16.,
            ceil_height: 100.
        },
        // The other room
        Sector {
            sides: vec![
                Side {
                    p1: Vector2f::new(78., 768.),
                    p2: Vector2f::new(0., 768.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(0., 768.),
                    p2: Vector2f::new(0., 1024.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(0., 1024.),
                    p2: Vector2f::new(512., 1024.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(512., 1024.),
                    p2: Vector2f::new(512., 768.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(512., 768.),
                    p2: Vector2f::new(178., 768.),
                    neighbour: -1
                },
                Side {
                    p1: Vector2f::new(178., 768.),
                    p2: Vector2f::new(78., 768.),
                    neighbour: 1
                }
            ],
            floor_height: -200.,
            ceil_height: 80.
        }
    ];

    let mut player = Thing {
        pos: Vector2f::new(0., 0.),
        zpos: PLAYER_EYE_HEIGHT,
        falling: false,
        fall_velocity: 0.,
        velocity: Vector2f::new(0.,0.),
        rot: 0.,
        sector: 0
    };

    let mut clock = Clock::start();
    let mut accum = 0.;

    // Prepare this each frame
    let mut offs: Vec<Cutoffs> = vec![];
    let h = HEIGHT as i64 / 2;
    for _ in 0..WIDTH {
        offs.push(Cutoffs {
            top: h,
            bottom: -h
        })
    }

    // Prepare for delta measurement
    let h32 = HEIGHT as i32 / 2;
    let w32 = WIDTH as i32 / 2;
    let center = Vector2i::new(w32, h32);
    center_mouse(&mut window);

    loop {
        let delta_time = clock.restart().as_seconds();
        println!("{} FPS", 1. / delta_time);

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => {}
            }
        }

        // Mouselook
        let delta_mouse = window.mouse_position() - center;
        mouselook(delta_mouse, &mut player);
        center_mouse(&mut window);

        accum += delta_time;
        if accum > PHYSICS_TIMESTEP {
            accum -= PHYSICS_TIMESTEP;
            // Do physics step
            process_movement(&mut player, &map);
        }

        // Prepare for render
        for i in 0..WIDTH as usize {
            offs[i].top = h;
            offs[i].bottom = -h;
        }

        window.clear(&Color::BLACK);
        draw_3d_map(&mut window, &map, &player, &mut offs);
        // draw_map(&mut window, &t_map, &player);
        window.display();
    }
}

/* Controls */
fn process_movement (player: &mut Thing, map: &Vec<Sector>) {
    // This part should run at 35 fps

    let mut acc = Vector2f::new(0., 0.);
    let mv = PLAYER_ACCELERATION;

    if Key::is_pressed(Key::W) {
        acc += Vector2f::new(0., mv);
    }
    if Key::is_pressed(Key::A) {
        acc += Vector2f::new(-mv, 0.);
    }
    if Key::is_pressed(Key::S) {
        acc += Vector2f::new(0., -mv);
    }
    if Key::is_pressed(Key::D) {
        acc += Vector2f::new(mv, 0.);
    }

    let rot_acc = rotate_vec(acc, player.rot);
    player.velocity += rot_acc;

    // Apply friction
    player.velocity *= FRICTION;

    if mag(&player.velocity) > PLAYER_SPEED_CAP {
        // Speed cap the player
        player.velocity = PLAYER_SPEED_CAP * unit_vector(player.velocity);
    }

    let sect = &map[player.sector];
    collision_detection(&sect, map, player);

    // Gravity
    if player.falling {
        player.fall_velocity -= GRAVITY;
    }

    // Apply velocity
    player.pos += player.velocity;
    player.zpos += player.fall_velocity;

    // Landing
    if player.zpos < sect.floor_height + PLAYER_EYE_HEIGHT {
        player.zpos = sect.floor_height + PLAYER_EYE_HEIGHT;
        player.falling = false;
    }

    // Rotation
    let rot = PLAYER_ROT_SPEED;
    let mut rt = 0.;
    if Key::is_pressed(Key::Left) {
        rt += -rot;
    }
    if Key::is_pressed(Key::Right) {
        rt += rot;
    }
    player.rot += rt;
}

fn collision_detection (sect: &Sector, map: &Vec<Sector>, player: &mut Thing) {
    let next_frame = player.pos + player.velocity;
    for s in 0..sect.sides.len() {
        let side = &sect.sides[s];

        // We'll cross the wall if we move
        let lsi = segment_intersection(&side.p1, &side.p2, &player.pos, &next_frame);
        if lsi == SegmentIntersection::Intersection {
            if side.neighbour == -1 {
                // Slide along the wall and rerun collision detection
                player.velocity = vector_projection(player.velocity, side.p2 - side.p1);
                collision_detection(sect, map, player);
                return;
            }

            // It's a portal, so we might be moving sectors
            let nu = side.neighbour as usize;
            let ns = &map[nu];
            let step = ns.floor_height - sect.floor_height;

            // We can't step that high
            if step > PLAYER_MAX_STEP_HEIGHT {
                player.velocity = vector_projection(player.velocity, side.p2 - side.p1);
                collision_detection(sect, map, player);
                return;
            }

            // We're moving to that sector!
            player.sector = nu;
            if step < 0. { player.falling = true }
        }
    }
}

fn center_mouse (window: &mut RenderWindow) {
    let h = HEIGHT as i32 / 2;
    let w = WIDTH as i32 / 2;
    window.set_mouse_position(&Vector2i::new(w, h));
}

fn mouselook (v: Vector2i, player: &mut Thing) {
    let mx = v.x as f32 * X_MOUSE_SENSITIVITY;
    player.rot += mx;
}