use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

// TEST

mod vector_utils;
use vector_utils::*;
mod drawing;
use drawing::*;
mod constants;
use constants::*;
mod map;
use map::*;
mod game_logic;
use game_logic::*;
mod resource_pool;

fn main() {
    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "3d_fps",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(false);
    window.set_mouse_cursor_visible(!(ENABLE_VERTICAL_MOUSELOOK || ENABLE_VERTICAL_MOUSELOOK));
    // window.set_framerate_limit(60);

    // Load "wad stuff"
    let resources = resource_pool::create_and_load();

    // TODO (SOON): Load maps from files
    let btex = String::from("bricks");
    let ctex = String::from("concrete");
    let stex = String::from("concrete-square");
    let map: Vec<Sector> = vec![
        // Spawn room
        Sector {
            sides: vec![
                Side {
                    p1: Vector2f::new(-256., 256.),
                    p2: Vector2f::new(78., 256.),
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(78., 256.),
                    p2: Vector2f::new(178., 256.),
                    // This line is the portal to the corridor.
                    neighbour: 1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(178., 256.),
                    p2: Vector2f::new(256., 256.),
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(256., 256.),
                    p2: Vector2f::new(256., -256.),
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(256., -256.),
                    p2: Vector2f::new(-256., -256.),
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(-256., -256.),
                    p2: Vector2f::new(-256., 256.),
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
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
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(78., 768.),
                    p2: Vector2f::new(178., 768.),
                    neighbour: 2,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(178., 768.),
                    p2: Vector2f::new(178., 256.),
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                },
                Side {
                    p1: Vector2f::new(178., 256.),
                    p2: Vector2f::new(78., 256.),
                    neighbour: 0,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
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
                    neighbour: -1,
                    mid: btex.clone(),
                    upper: stex.clone(),
                    lower: stex.clone()
                },
                Side {
                    p1: Vector2f::new(0., 768.),
                    p2: Vector2f::new(0., 1024.),
                    neighbour: -1,
                    mid: stex.clone(),
                    upper: stex.clone(),
                    lower: stex.clone()
                },
                Side {
                    p1: Vector2f::new(0., 1024.),
                    p2: Vector2f::new(512., 1024.),
                    neighbour: -1,
                    mid: stex.clone(),
                    upper: stex.clone(),
                    lower: stex.clone()
                },
                Side {
                    p1: Vector2f::new(512., 1024.),
                    p2: Vector2f::new(512., 768.),
                    neighbour: -1,
                    mid: stex.clone(),
                    upper: stex.clone(),
                    lower: stex.clone()
                },
                Side {
                    p1: Vector2f::new(512., 768.),
                    p2: Vector2f::new(178., 768.),
                    neighbour: -1,
                    mid: stex.clone(),
                    upper: stex.clone(),
                    lower: stex.clone()
                },
                Side {
                    p1: Vector2f::new(178., 768.),
                    p2: Vector2f::new(78., 768.),
                    neighbour: 1,
                    mid: btex.clone(),
                    upper: btex.clone(),
                    lower: btex.clone()
                }
            ],
            floor_height: 0.,
            ceil_height: 256.
        }
    ];

    let mut player = Thing {
        pos: Vector2f::new(0., 0.),
        zpos: PLAYER_EYE_HEIGHT,
        falling: false,
        fall_velocity: 0.,
        velocity: Vector2f::new(0.,0.),
        rot: 0.,
        sector: 0,
        yaw: 0.
    };

    let mut clock = Clock::start();
    let mut accum = 0.;

    // Prepare this each frame
    let h = HEIGHT as i64 / 2;
    let mut offs: Vec<Cutoffs> = vec![Cutoffs {
        top: h,
        bottom: -h
    }; WIDTH as usize];


    // Prepare render texture
    let mut pixels: Vec<u8> = vec![255; PIXEL_ARRAY_LENGTH];

    let mut render_texture = Texture::new(WIDTH, HEIGHT).unwrap();

    // Prepare for delta measurement
    let h32 = HEIGHT as i32 / 2;
    let w32 = WIDTH as i32 / 2;
    let center = Vector2i::new(w32, h32);
    center_mouse(&mut window);

    loop {
        let delta_time = clock.restart().as_seconds();
        // println!("{} FPS", 1. / delta_time);

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => {}
            }
        }

        // Mouselook
        let delta_mouse = window.mouse_position() - center;

        if ENABLE_VERTICAL_MOUSELOOK || ENABLE_HORIZONTAL_MOUSELOOK {
            mouselook(delta_mouse, &mut player);
            center_mouse(&mut window);
        }

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
        draw_3d_map(&mut window, &resources, &map, &player, &mut offs, &mut pixels);

        render_texture.update_from_pixels(&pixels, WIDTH, HEIGHT, 0, 0);
        let render_sprite = Sprite::with_texture(&render_texture);
        window.draw(&render_sprite);

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
        if lsi.kind == IntersectionKind::Intersection {
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
            // TODO: Run collision detection here so we can't clip on a corner
            //       It's a little more complicated than that, 'cause we need to
            //       not clip the line we came into the sector over
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
    let mut mx = v.x as f32 * X_MOUSE_SENSITIVITY;
    let mut my = v.y as f32 * Y_MOUSE_SENSITIVITY;
    if !ENABLE_HORIZONTAL_MOUSELOOK { mx = 0. }
    if !ENABLE_VERTICAL_MOUSELOOK { my = 0. }

    player.rot += mx;
    player.yaw += my;
    if player.yaw > MAX_YAW { player.yaw = MAX_YAW }
    if player.yaw < -MAX_YAW { player.yaw = -MAX_YAW }
}