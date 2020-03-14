use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;
use std::f32::consts::PI;

struct Wall {
    pub colour: Color,
    pub p1: Vector2f,
    pub p2: Vector2f
}
struct Thing {
    pub pos: Vector2f, // Position
    pub rot: f32       // Rotation
}

fn main() {
    let mut window = RenderWindow::new(
    (800, 600),
    "shooter",
    Style::CLOSE,
    &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let map: Vec<Wall> = vec![
        Wall {
            colour: Color::RED,
            p1: Vector2f::new(-100., 100.),
            p2: Vector2f::new(100., 100.)
        },
        Wall {
            colour: Color::GREEN,
            p1: Vector2f::new(100., 100.),
            p2: Vector2f::new(100., -100.)
        },
        Wall {
            colour: Color::YELLOW,
            p1: Vector2f::new(100., -100.),
            p2: Vector2f::new(-100., -100.)
        },
        Wall {
            colour: Color::CYAN,
            p1: Vector2f::new(-100., -100.),
            p2: Vector2f::new(-100., 100.)
        }
    ];
    let player = Thing {
        pos: Vector2f::new(0.0, 0.0),
        rot: 0.0
    };

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => {}
            }
        }

        window.clear(&Color::BLACK);
        draw_map(&mut window, &map, &player);
        window.display();
    }
}

// Angle in radians
fn draw_line_at_rotation (window: &mut RenderWindow, pos: Vector2f, length: f32, angle: f32, colour: Color) {
    let center = Vector2f::new(400., 300.);

    let mut rct = RectangleShape::new();
    rct.set_size(Vector2f::new(length, 3.));
    rct.set_rotation(angle * 180. / PI - 90.);
    rct.set_fill_color(&colour);
    rct.set_position(center + pos);

    window.draw(&rct);
}

fn draw_wall (wall: &Wall, window: &mut RenderWindow) {
    let xdiff = wall.p1.x - wall.p2.x;
    let ydiff = wall.p1.y - wall.p2.y;
    let angle = ydiff.atan2(xdiff) + PI;
    let yda = ydiff.abs();
    let xda = xdiff.abs();

    let py = xda * xda + yda * yda;
    let dist = py.sqrt();

    draw_line_at_rotation(window, wall.p1, dist, angle, wall.colour);
}

fn draw_thing (window: &mut RenderWindow, thing: &Thing) {
    let center = Vector2f::new(400., 300.);

    let mut rct = RectangleShape::new();
    rct.set_size(Vector2f::new(10., 10.));
    rct.set_fill_color(&Color::GREEN);
    rct.set_position(center + thing.pos - Vector2f::new(5., 5.));

    window.draw(&rct);

    draw_line_at_rotation(window, thing.pos, 30., thing.rot, Color::WHITE);
}

fn draw_map (window: &mut RenderWindow, map: &Vec<Wall>, player: &Thing) {
    for wall in map {
        draw_wall(&wall, window);
    }
    draw_thing(window, player);
}
