mod config;
mod snake;

use config::*;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston_window::*;
use rand::thread_rng;
use rand::Rng;
use snake::*;
use std::collections::LinkedList;

pub struct Game {
    window: PistonWindow,
    snake: Snake,
    food: Node,
    high_score: u16,
}

impl Game {
    fn render(&mut self, _args: &RenderArgs, event: &Event, glyphs: &mut Glyphs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        const SEPERATOR_LINE_RADIUS: f64 = 0.5;

        let snake = &self.snake;
        let food = &self.food;
        let font_size: u32 = 32;
        let text_padding: f64 = 10.0;

        self.window.draw_2d(event, |c, gl, device| {
            //Clear the screen
            clear([0.42, 0.0, 0.5, 1.0], gl);
            let num_of_cells_horizontal = (SCREEN_W / CELL_W) as i32;
            let num_of_cells_vertical = (SCREEN_H / CELL_W) as i32;

            // Draw the food
            rectangle(
                YELLOW,
                [CELL_W * food.x, CELL_W * food.y, CELL_W, CELL_W],
                c.transform,
                gl,
            );

            // Draw the snake
            let color_multiplier = 1.0 / snake.nodes.len() as f32;
            let mut node_index = 0.0;
            for node in snake.nodes.iter().rev() {
                rectangle(
                    [
                        color_multiplier * node_index,
                        1.0 - color_multiplier * node_index,
                        1.0 - color_multiplier * node_index,
                        1.0,
                    ],
                    [CELL_W * node.x, CELL_W * node.y, CELL_W, CELL_W],
                    c.transform,
                    gl,
                );

                node_index += 1.0;
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

            // Draw the seperator lines
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

            // Render current score
            text(
                [0.0, 1.0, 0.0, 0.5],
                font_size,
                snake.nodes.len().to_string().as_str(),
                &mut *glyphs,
                c.transform
                    .trans(text_padding, font_size as f64 + text_padding),
                gl,
            )
            .unwrap();

            glyphs.factory.encoder.flush(device);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Update the snakes location
        match self.snake.direction {
            Direction::Up => self.snake.update_node_locations(0.0, -1.0),
            Direction::Down => self.snake.update_node_locations(0.0, 1.0),
            Direction::Right => self.snake.update_node_locations(1.0, 0.0),
            Direction::Left => self.snake.update_node_locations(-1.0, 0.0),
        };

        // Check if the snake did bite itself
        let mut snake_nodes_iter = self.snake.nodes.iter();
        let head = snake_nodes_iter.next().unwrap().clone();

        while let Some(node) = snake_nodes_iter.next() {
            if head.x == node.x && head.y == node.y {
                // Bit itself...
                println!("Game over!");
            }
        }

        // Check if the snake has eaten the food
        if head.x == self.food.x && head.y == self.food.y {
            // Just push back a new random node
            // it will be updated automatically
            self.snake.nodes.push_back(Node { x: -1.0, y: -1.0 });
            self.place_random_food();
        }
    }

    fn handle_input(&mut self, key: &Key) {
        let mut nodes_iter = self.snake.nodes.iter();
        let head = nodes_iter.next().unwrap();
        let second = nodes_iter.next().unwrap();

        if *key == Key::Up || *key == Key::W {
            match self.snake.direction {
                Direction::Down => {}
                _ => {
                    if head.y != second.y + 1 as f64 {
                        self.snake.direction = Direction::Up
                    }
                }
            }
        } else if *key == Key::Down || *key == Key::S {
            match self.snake.direction {
                Direction::Up => {}
                _ => {
                    if head.y != second.y - 1 as f64 {
                        self.snake.direction = Direction::Down
                    }
                }
            }
        } else if *key == Key::Right || *key == Key::D {
            match self.snake.direction {
                Direction::Left => {}
                _ => {
                    if head.x != second.x - 1 as f64 {
                        self.snake.direction = Direction::Right
                    }
                }
            }
        } else if *key == Key::Left || *key == Key::A {
            match self.snake.direction {
                Direction::Right => {}
                _ => {
                    if head.x != second.x + 1 as f64 {
                        self.snake.direction = Direction::Left
                    }
                }
            }
        };
    }

    fn place_random_food(&mut self) {
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
                if current.x == rand_x as f64 && current.y == rand_y as f64 {
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
    let mut window: PistonWindow = WindowSettings::new("Snake", [SCREEN_W, SCREEN_H])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Load the glyph
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("PlaymegamesReguler-2OOee.ttf"))
        .unwrap();

    // Create the snake
    let mut nodes: LinkedList<Node> = LinkedList::new();
    nodes.push_back(Node { x: 10.0, y: 10.0 });
    nodes.push_back(Node { x: 11.0, y: 10.0 });
    nodes.push_back(Node { x: 12.0, y: 10.0 });

    let direction = Direction::Left;
    let food = Node { x: 5.0, y: 5.0 };
    let snake = Snake { nodes, direction };

    // Create a new game and run it.
    let mut game = Game {
        window,
        snake,
        food,
        high_score: 0,
    };

    game.place_random_food();

    let mut event_settings = EventSettings::new();
    event_settings.ups = 16;
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut game.window) {
        if let Some(args) = e.render_args() {
            game.render(&args, &e, &mut glyphs);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.handle_input(&key)
        };
    }
}
