use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;
use std::f32::consts::PI;

// Clonable for transformed map
#[derive(Clone)]
struct Wall {
    pub colour: Color,
    pub p1: Vector2f,
    pub p2: Vector2f,
    pub height: f32
}
struct Thing {
    pub pos: Vector2f, // Position
    pub rot: f32       // Rotation
}

const PLAYER_SPEED: f32 = 1.;
const PLAYER_ROT_SPEED: f32 = PI;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

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
        draw_3d_map(&mut window, &mut t_map, &player);
        // draw_map(&mut window, &t_map, &player);
        window.display();
    }
}

/* -= CONTROLS STUFF =- */
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

/* -= VECTOR AND RENDER STUFF =- */
fn draw_quad (window: &mut RenderWindow, top_left: Vector2f, top_right: Vector2f, bottom_right: Vector2f, bottom_left: Vector2f, colour: Color) {
    let mut vertex_array = VertexArray::default();
    vertex_array.set_primitive_type(PrimitiveType::Triangles);

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

    window.draw(&vertex_array);
}

fn draw_3d_map (window: &mut RenderWindow, map: &mut Vec<Wall>, player: &Thing) {
    let to_top = HEIGHT as f32 / 2.;

    // "To perspective project a co-ordinate, you simply take its x and y
    //  co-ordinate and divide them by the z co-ordinate"
    // tz1 = wall.p1.y
    // tz2 = wall.p2.y
    // tx1 = wall.p1.x
    // tx2 = wall.p2.x
    for wall in map {
        if wall.p1.y > 0. || wall.p2.y  > 0. {
            // View frustum clipping
            let i1 = line_intersect(
                wall.p1,
                wall.p2,
                Vector2f::new(0., 0.),
                Vector2f::new(-1., 0.001));
            let i2 = line_intersect(
                wall.p1,
                wall.p2,
                Vector2f::new(0., 0.),
                Vector2f::new(1., 0.001));
            if wall.p1.y <= 0. {
                if i1.y > 0. {
                    wall.p1 = i1
                } else {
                    wall.p1 = i2
                }
            }
            if wall.p2.y <= 0. {
                if i1.y > 0. {
                    wall.p2 = i1;
                } else {
                    wall.p2 = i2;
                }
            }

            // Perspective projection
            let wz1 = wall.height / 2.;
            let wz2 = -wz1;
            let wx1 = wall.p1.x * 500. / wall.p1.y;
            let wx2 = wall.p2.x * 500. / wall.p2.y;
            let wy1a = wz2 / wall.p1.y;
            let wy1b = wz1 / wall.p1.y;
            let wy2a = wz2 / wall.p2.y;
            let wy2b = wz1 / wall.p2.y;

            let top_left = Vector2f::new(wx1, wy1a);
            let top_right = Vector2f::new(wx2, wy2a);
            let bottom_right = Vector2f::new(wx2, wy2b);
            let bottom_left = Vector2f::new(wx1, wy1b);


            let ceil_left = Vector2f::new(top_left.x, to_top);
            let ceil_right = Vector2f::new(top_right.x, to_top);
            let floor_left = Vector2f::new(bottom_left.x, -to_top);
            let floor_right = Vector2f::new(bottom_right.x, -to_top);

            // Ceiling
            draw_quad(window, ceil_left, ceil_right, top_right, top_left, Color::WHITE);
            // Floor
            draw_quad(window, bottom_left, bottom_right, floor_right, floor_left, Color::BLUE);
            // Wall
            draw_quad(window, top_left, top_right, bottom_right, bottom_left, wall.colour);
        }
    }
}

fn line_intersect (v1: Vector2f, v2: Vector2f, v3: Vector2f, v4: Vector2f) -> Vector2f {
    // From https://youtu.be/HQYsFshbkYw?t=188
    let mut v = Vector2f::new(cross_product(v1, v2), cross_product(v3, v4));
    let det = cross_product(v1 - v2, v3 - v4);
    v.x = cross_product(Vector2f::new(v.x, v1.x - v2.x), Vector2f::new(v.y, v3.x - v4.x)) / det;
    v.y = cross_product(Vector2f::new(v.x, v1.y - v2.y), Vector2::new(v.y, v3.y - v4.y)) / det;
    v
}

fn cross_product (v1: Vector2f, v2: Vector2f) -> f32 {
    v1.x * v2.y - v1.y * v2.x
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

fn sfml_vec (v: Vector2f) -> Vector2f {
    let center = Vector2f::new(WIDTH as f32 / 2., HEIGHT as f32 / 2.);
    center + Vector2f::new(v.x, -v.y)
}

fn rotate_vec (v: Vector2f, theta: f32) -> Vector2f {
    let t = -theta;
    let st = t.sin();
    let ct = t.cos();

    Vector2::new(
        v.x * ct - v.y * st,
        v.x * st + v.y * ct
    )
}

// Angle in radians
fn draw_line_at_rotation (window: &mut RenderWindow, pos: Vector2f, length: f32, angle: f32, colour: Color) {
    let mut rct = RectangleShape::new();
    rct.set_size(Vector2f::new(length, 3.));
    rct.set_rotation(angle * 180. / PI - 90.);
    rct.set_fill_color(&colour);
    rct.set_position(sfml_vec(pos));

    window.draw(&rct);
}

fn draw_line (window: &mut RenderWindow, p1: Vector2f, p2: Vector2f, colour: Color) {
    let xdiff = p2.x - p1.x;
    let ydiff = p2.y - p1.y;
    let angle = -ydiff.atan2(xdiff) + PI / 2.;

    let yda = ydiff.abs();
    let xda = xdiff.abs();

    let py = xda * xda + yda * yda;
    let dist = py.sqrt();

    draw_line_at_rotation(window, p1, dist, angle, colour);
}

fn draw_wall (wall: &Wall, window: &mut RenderWindow) {
    draw_line(window, wall.p1, wall.p2, wall.colour);
}

fn draw_thing (window: &mut RenderWindow, thing: &Thing, player: &Thing) {
    let mut rct = RectangleShape::new();
    rct.set_size(Vector2f::new(10., 10.));
    rct.set_fill_color(&Color::GREEN);
    rct.set_position(sfml_vec(thing.pos - player.pos) - Vector2f::new(5., 5.));

    window.draw(&rct);

    draw_line_at_rotation(window, thing.pos - player.pos, 30., thing.rot - player.rot, Color::WHITE);
}

fn draw_map (window: &mut RenderWindow, map: &Vec<Wall>, player: &Thing) {
    for wall in map {
        draw_wall(&wall, window);
    }
    // As a result of player being the thing and the anchor
    // he's always drawn facing straight up and centered.
    // That's fine because the world rotates around him.
    draw_thing(window, player, player);
}
