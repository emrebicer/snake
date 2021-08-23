extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, PressEvent, Button, Key};
use piston::window::WindowSettings;
use std::collections::LinkedList;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    snake: Snake,
    food: Node
}

struct Snake {
    nodes: LinkedList<Node>,
    direction: Direction,
}

#[derive(Clone)]
struct Node {
    x: f64,
    y: f64,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

const SCREEN_W: f64 = 500.0;
const SCREEN_H: f64 = 800.0;

const CELL_W: f64 = 25.0;

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        const SEPERATOR_LINE_RADIUS: f64 = 0.5;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let small_square = rectangle::square(0.0, 0.0, 25.0);
        let rotation = self.rotation;
        let snake = &self.snake;
        let food = &self.food;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);


            // Draw the seperator lines
            let num_of_cells_horizontal = (SCREEN_W / CELL_W) as i32;
            let num_of_cells_vertical = (SCREEN_H / CELL_W) as i32;

            for i in 1..num_of_cells_horizontal {
                line_from_to(
                    BLACK,
                    SEPERATOR_LINE_RADIUS,
                    [CELL_W * i as f64, 0.0],
                    [CELL_W * i as f64, SCREEN_H],
                    c.transform,
                    gl,
                );
            }

            for i in 1..num_of_cells_vertical {
                line_from_to(
                    BLACK,
                    SEPERATOR_LINE_RADIUS,
                    [0.0, CELL_W * i as f64],
                    [SCREEN_W, CELL_W * i as f64],
                    c.transform,
                    gl,
                );
            }

            // Draw the food
            rectangle(
                YELLOW,
                [CELL_W * food.x, CELL_W * food.y, CELL_W, CELL_W],
                c.transform,
                gl,
            );

            // Draw the snake
            for node in snake.nodes.iter() {
                rectangle(
                    RED,
                    [CELL_W * node.x, CELL_W * node.y, CELL_W, CELL_W],
                    c.transform,
                    gl,
                );
            }

            // Redraw the head in a different color
            rectangle(
                BLACK,
                [
                    CELL_W * snake.nodes.front().unwrap().x,
                    CELL_W * snake.nodes.front().unwrap().y,
                    CELL_W,
                    CELL_W,
                ],
                c.transform,
                gl,
            );

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
            rectangle(BLUE, small_square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;



        match self.snake.direction {
            Direction::Up => update_node_locations(&mut self.snake, 0.0, -1.0),
            Direction::Down => update_node_locations(&mut self.snake, 0.0, 1.0),
            Direction::Right => update_node_locations(&mut self.snake, 1.0, 0.0),
            Direction::Left => update_node_locations(&mut self.snake, -1.0, 0.0),
        };
    }

    fn handle_input(&mut self, key: &Key){

        if *key == Key::Up {
            match self.snake.direction {
                Direction::Down => {},
                _ => {self.snake.direction = Direction::Up}
            }
        } else if *key == Key::Down{
            // Down key
            match self.snake.direction {
                Direction::Up => {},
                _ => {self.snake.direction = Direction::Down}
            }
        } else if *key == Key::Right{
            // Right key
            match self.snake.direction {
                Direction::Left=> {},
                _ => {self.snake.direction = Direction::Right}
            }
        } else if *key == Key::Left{
            // Left key
            match self.snake.direction {
                Direction::Right=> {},
                _ => {self.snake.direction = Direction::Left}
            }
        };
    }
}


fn update_node_locations(snake: &mut Snake, x_change: f64, y_change: f64) {

    let mut prev = snake.nodes.front().unwrap().clone();

    let mut clone_nodes = snake.nodes.clone();

    let mut iter = clone_nodes.iter_mut();
    let node_count = snake.nodes.len();

    for index in 2..node_count - 1 {
        let current = iter.nth(index).unwrap();
        let temp = current.clone();
        current.x = prev.x;
        current.y = prev.y;
        prev = temp;
    }

    let head = iter.nth(0).unwrap();
    head.x += x_change;
    head.y += y_change;

    if head.x == -1.0 {
        head.x = SCREEN_W / CELL_W;
    } else if head.x == SCREEN_W / CELL_W {
        head.x = 0.0;
    }

    if head.y == -1.0 {
        head.y = SCREEN_H / CELL_W;
    } else if head.y == SCREEN_H / CELL_W {
        head.y = 0.0;
    }

    println!("{} --- {}", head.x, head.y);

    snake.nodes = clone_nodes;
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [SCREEN_W, SCREEN_H])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create the snake
    let mut nodes: LinkedList<Node> = LinkedList::new();
    nodes.push_back(Node { x: 10.0, y: 10.0 });
    nodes.push_back(Node { x: 11.0, y: 10.0 });
    nodes.push_back(Node { x: 12.0, y: 10.0 });

    let direction = Direction::Left;
    let food = Node{ x: 5.0, y: 5.0 };
    let snake = Snake { nodes, direction };

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        snake,
        food
    };

    let mut event_settings = EventSettings::new();
    event_settings.ups = 10;
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("{:?}", key);
            app.handle_input(&key)
        };
    }
}
