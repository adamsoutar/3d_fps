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

    let map: Vec<Wall> = vec![
        Wall {
            colour: Color::RED,
            p1: Vector2f::new(-1., 1.),
            p2: Vector2f::new(1., 1.),
            height: 128.
        },
        Wall {
            colour: Color::GREEN,
            p1: Vector2f::new(1., 1.),
            p2: Vector2f::new(1., -1.),
            height: 128.
        },
        Wall {
            colour: Color::YELLOW,
            p1: Vector2f::new(1., -1.),
            p2: Vector2f::new(-1., -1.),
            height: 128.
        },
        Wall {
            colour: Color::CYAN,
            p1: Vector2f::new(-1., -1.),
            p2: Vector2f::new(-1., 1.),
            height: 128.
        }
    ];
    // let map= vec![
    //     Wall {
    //         colour: Color::YELLOW,
    //         p1: Vector2f::new(-100., 100.),
    //         p2: Vector2f::new(100., 100.),
    //         height: 5000.
    //     }
    // ];

    let mut player = Thing {
        pos: Vector2f::new(0.0, 0.0),
        rot: 0.0
    };

    let mut clock = Clock::start();

    loop {
        let delta_time = clock.restart().as_seconds();
        //println!("{} FPS", 1. / delta_time);

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => {}
            }
        }

        process_movement(delta_time, &mut player);
        let mut t_map = get_transformed_map(&map, &player);

        window.clear(&Color::BLACK);
        draw_3d_map(&mut window, &mut t_map);
        // draw_map(&mut window, &t_map, &player);
        window.display();
    }
}

fn get_transformed_map (map: &Vec<Wall>, player: &Thing) -> Vec<Wall> {
    let mut t_map = vec![];

    for w in map {
        let mut wall = w.clone();
        wall.p1 = rotate_vec(wall.p1 - player.pos, -player.rot);
        wall.p2 = rotate_vec(wall.p2 - player.pos, -player.rot);
        t_map.push(wall);
    }

    t_map
}

/* Controls */
fn process_movement (delta_time: f32, player: &mut Thing) {
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

    player.pos += rotate_vec(movement, player.rot);

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

