extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod snake;
mod config;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, PressEvent, Button, Key};
use piston::window::WindowSettings;
use std::collections::LinkedList;
use config::*;
use snake::*;
use rand::thread_rng;
use rand::Rng;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Snake,
    food: Node
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        const SEPERATOR_LINE_RADIUS: f64 = 0.5;

        let snake = &self.snake;
        let food = &self.food;

        self.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen.
            clear(GREEN, gl);

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
                RED,
                [CELL_W * food.x, CELL_W * food.y, CELL_W, CELL_W],
                c.transform,
                gl,
            );

            // Draw the snake
            for node in snake.nodes.iter() {
                rectangle(
                    YELLOW,
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
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {

        match self.snake.direction {
            Direction::Up => self.snake.update_node_locations(0.0, -1.0),
            Direction::Down => self.snake.update_node_locations(0.0, 1.0),
            Direction::Right => self.snake.update_node_locations(1.0, 0.0),
            Direction::Left => self.snake.update_node_locations(-1.0, 0.0),
        };


        // Check if the snake has eaten the food
        let head = self.snake.nodes.front().unwrap();
        if head.x == self.food.x && head.y == self.food.y {
            // Just push back a new random node
            // it will be updated automatically
            self.snake.nodes.push_back(Node{x:-1.0, y:-1.0});
            self.place_random_food();
        }
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

    fn place_random_food(&mut self){

        let x_len = SCREEN_W / CELL_W;
        let y_len = SCREEN_H / CELL_W;

        let mut rng = thread_rng();
        loop {
            let rand_x: i32 = rng.gen_range(0, x_len as i32);
            let rand_y: i32 = rng.gen_range(0, y_len as i32);

            // Check if the snake is on those coordinates
            let mut flag = true;
            let mut snake_nodes_iter = self.snake.nodes.iter();

            while let Some(current) = snake_nodes_iter.next() {
                if current.x == rand_x as f64 && current.y == rand_y as f64{
                    flag = false;
                }
            }

            // Check if the position is not the same as previous
            if self.food.x == rand_x as f64 && self.food.y == rand_y as f64 {
                flag = false;
            }

            if flag {
                self.food.x = rand_x as f64;
                self.food.y = rand_y as f64;
                break;
            }
        }
    }
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
        snake,
        food
    };

    app.place_random_food();

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
            app.handle_input(&key)
        };
    }
}
