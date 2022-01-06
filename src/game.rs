use crate::config::Config;
use crate::snake::{Direction, Node, Snake};
use piston::input::{Key, RenderArgs, UpdateArgs};
use piston_window::*;
use rand::thread_rng;
use rand::Rng;
use std::collections::LinkedList;

const SEPERATOR_LINE_RADIUS: f64 = 0.5;

pub struct Game {
    pub config: Config,
    pub window: PistonWindow,
    pub snake: Snake,
    pub food: Node,
    pub obstacles: LinkedList<Node>,
    pub score: u16,
    pub high_score: u16,
    pub direction_queue: Vec<Direction>,
}

impl Game {
    pub fn render(&mut self, _args: &RenderArgs, event: &Event, glyphs: &mut Glyphs) {
        let font_size = 32;
        let text_padding = 10.0;

        self.window.draw_2d(event, |c, g, device| {
            // Check if the snake is dead
            if !self.snake.is_alive {
                render_game_over(c, g, glyphs, self.score, self.high_score, self.config);
                glyphs.factory.encoder.flush(device);
                return;
            }

            // Clear the screen
            clear(self.config.background_color, g);
            let num_of_cells_horizontal = (self.config.screen_w / self.config.cell_w) as i32;
            let num_of_cells_vertical = (self.config.screen_h / self.config.cell_w) as i32;

            // Draw the food
            rectangle(
                self.config.food_color,
                [
                    self.config.cell_w * self.food.x,
                    self.config.cell_w * self.food.y,
                    self.config.cell_w,
                    self.config.cell_w,
                ],
                c.transform,
                g,
            );

            // Draw the snake
            let mut node_index = 1.0;

            let (snake_first_color, snake_second_color) = match self.snake.is_turbo {
                true => (
                    self.config.snake_turbo_first_color,
                    self.config.snake_turbo_second_color,
                ),
                false => (
                    self.config.snake_first_color,
                    self.config.snake_second_color,
                ),
            };

            for node in self.snake.nodes.iter().rev() {
                rectangle(
                    [
                        lerp(
                            *snake_second_color.get(0).unwrap(),
                            *snake_first_color.get(0).unwrap(),
                            self.snake.nodes.len(),
                            node_index,
                        ),
                        lerp(
                            *snake_second_color.get(1).unwrap(),
                            *snake_first_color.get(1).unwrap(),
                            self.snake.nodes.len(),
                            node_index,
                        ),
                        lerp(
                            *snake_second_color.get(2).unwrap(),
                            *snake_first_color.get(2).unwrap(),
                            self.snake.nodes.len(),
                            node_index,
                        ),
                        lerp(
                            *snake_second_color.get(3).unwrap(),
                            *snake_first_color.get(3).unwrap(),
                            self.snake.nodes.len(),
                            node_index,
                        ),
                    ],
                    [
                        self.config.cell_w * node.x,
                        self.config.cell_w * node.y,
                        self.config.cell_w,
                        self.config.cell_w,
                    ],
                    c.transform,
                    g,
                );

                node_index += 1.0;
            }

            // Redraw the head in a different color
            rectangle(
                self.config.snake_head_color,
                [
                    self.config.cell_w * self.snake.nodes.front().unwrap().x,
                    self.config.cell_w * self.snake.nodes.front().unwrap().y,
                    self.config.cell_w,
                    self.config.cell_w,
                ],
                c.transform,
                g,
            );

            // Draw the obstacles
            for obstacle in self.obstacles.iter() {
                rectangle(
                    self.config.obstacle_color,
                    [
                        self.config.cell_w * obstacle.x,
                        self.config.cell_w * obstacle.y,
                        self.config.cell_w,
                        self.config.cell_w,
                    ],
                    c.transform,
                    g,
                );

                node_index += 1.0;
            }

            // Draw the seperator lines
            for i in 1..num_of_cells_horizontal {
                line_from_to(
                    self.config.seperator_line_color,
                    SEPERATOR_LINE_RADIUS,
                    [self.config.cell_w * i as f64, 0.0],
                    [self.config.cell_w * i as f64, self.config.screen_h],
                    c.transform,
                    g,
                );
            }

            for i in 1..num_of_cells_vertical {
                line_from_to(
                    self.config.seperator_line_color,
                    SEPERATOR_LINE_RADIUS,
                    [0.0, self.config.cell_w * i as f64],
                    [self.config.screen_w, self.config.cell_w * i as f64],
                    c.transform,
                    g,
                );
            }

            text(
                self.config.snake_first_color,
                font_size,
                self.score.to_string().as_str(),
                glyphs,
                c.transform
                    .trans(text_padding, font_size as f64 + text_padding),
                g,
            )
            .unwrap();

            glyphs.factory.encoder.flush(device);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let dt_ms = args.dt * 1000.0;
        self.snake.last_movement_duration += dt_ms;

        let mut delay = self.snake.movement_delay;
        if self.snake.is_turbo {
            delay /= 2.0;
        }

        if self.snake.last_movement_duration > delay {
            self.snake.last_movement_duration = 0.0;

            if self.snake.is_alive {
                // Check if there is a new direction in the input direction queue
                if !self.direction_queue.is_empty() {
                    let key = self.direction_queue.remove(0);

                    match (key, &self.snake.direction) {
                        (Direction::Up, Direction::Down) => {}
                        (Direction::Down, Direction::Up) => {}
                        (Direction::Right, Direction::Left) => {}
                        (Direction::Left, Direction::Right) => {}

                        (Direction::Up, _) => self.snake.direction = Direction::Up,
                        (Direction::Down, _) => self.snake.direction = Direction::Down,
                        (Direction::Right, _) => self.snake.direction = Direction::Right,
                        (Direction::Left, _) => self.snake.direction = Direction::Left,
                    }
                }

                // Update the snakes location
                match self.snake.direction {
                    Direction::Up => self.snake.update_node_locations(0.0, -1.0, self.config),
                    Direction::Down => self.snake.update_node_locations(0.0, 1.0, self.config),
                    Direction::Right => self.snake.update_node_locations(1.0, 0.0, self.config),
                    Direction::Left => self.snake.update_node_locations(-1.0, 0.0, self.config),
                };

                // Check if the snake did bite itself
                let mut snake_nodes_iter = self.snake.nodes.iter();
                let head = snake_nodes_iter.next().unwrap().clone();

                if snake_nodes_iter.any(|node| node.eq(head)) {
                    self.game_over();
                    return;
                }

                // Check if the snake hit an obstacle
                let mut obstacles_iter = self.obstacles.iter();
                if obstacles_iter.any(|node| node.eq(head)) {
                    self.game_over();
                    return;
                }

                // Check if the snake has eaten the food
                if head.eq(self.food) {
                    if self.snake.is_turbo {
                        self.score += 2;
                    } else {
                        self.score += 1;
                    }
                    // Just push back a new random node
                    // it will be updated automatically
                    self.snake.nodes.push_back(Node { x: -1.0, y: -1.0 });
                    self.place_random_food();
                }
            }
        }
    }

    pub fn handle_key_press(&mut self, key: Key) {
        if !self.snake.is_alive {
            if key == Key::Space {
                self.reset_game();
            }
            return;
        } else {
            // Check for the turbo key
            if key == Key::LShift {
                self.snake.is_turbo = true;
            }

            match key {
                Key::Up | Key::W => self.direction_queue.push(Direction::Up),
                Key::Down | Key::S => self.direction_queue.push(Direction::Down),
                Key::Right | Key::D => self.direction_queue.push(Direction::Right),
                Key::Left | Key::A => self.direction_queue.push(Direction::Left),
                _ => {}
            }
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        if self.snake.is_alive {
            if key == Key::LShift {
                self.snake.is_turbo = false;
            }
        }
    }

    pub fn place_random_obstacles(&mut self, count: u32) {
        for _ in 0..count {
            let (x, y) = self.find_random_available_node();
            self.obstacles.push_back(Node { x, y });
        }
    }

    pub fn place_random_food(&mut self) {
        let (rand_x, rand_y) = self.find_random_available_node();
        self.food.x = rand_x;
        self.food.y = rand_y;
    }

    fn find_random_available_node(&self) -> (f64, f64) {
        let x_len = self.config.screen_w / self.config.cell_w;
        let y_len = self.config.screen_h / self.config.cell_w;

        let mut rng = thread_rng();
        loop {
            let random_node = Node {
                x: rng.gen_range(0, x_len as i32) as f64,
                y: rng.gen_range(0, y_len as i32) as f64,
            };

            // Check if the snake is on those coordinates
            let mut snake_nodes_iter = self.snake.nodes.iter();
            if snake_nodes_iter.any(|node| node.eq(random_node)) {
                continue;
            }

            // Check if the food is on those coordinates
            if self.food.eq(random_node) {
                continue;
            }

            // Check if one of the obstacles are on this coordinates
            let mut obstacles_iter = self.obstacles.iter();
            if obstacles_iter.any(|node| node.eq(random_node)) {
                continue;
            }

            return (random_node.x, random_node.y);
        }
    }

    fn game_over(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }

        self.snake.is_alive = false;
    }

    pub fn reset_game(&mut self) {
        // Create the snake
        let nodes: LinkedList<Node> = LinkedList::from([
            Node { x: 10.0, y: 10.0 },
            Node { x: 11.0, y: 10.0 },
            Node { x: 12.0, y: 10.0 },
        ]);

        self.snake.nodes = nodes;
        self.snake.direction = Direction::Left;
        self.snake.is_alive = true;
        self.snake.is_turbo = false;
        self.snake.last_movement_duration = 0.0;

        self.score = 0;
        self.obstacles = LinkedList::new();

        self.place_random_obstacles(self.config.random_obstacle_count);
        self.place_random_food();
    }
}

fn render_text_center(
    color: types::Color,
    font_size: types::FontSize,
    text_content: &str,
    glyphs: &mut Glyphs,
    y: f64,
    c: Context,
    g: &mut G2d,
    config: &Config,
) {
    let text_width = glyphs.width(font_size, text_content).unwrap();
    let text_x = (config.screen_w - text_width) / 2.0;

    text(
        color,
        font_size,
        text_content,
        glyphs,
        c.transform.trans(text_x, y),
        g,
    )
    .unwrap();
}

fn render_game_over(
    c: Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
    score: u16,
    high_score: u16,
    config: Config,
) {
    let font_size = 32;
    let pop_up_offset = 50.0;
    let game_over_font_size = 48;

    // Pop-up square
    rectangle(
        [0.0, 0.0, 0.0, 0.1],
        [
            pop_up_offset,
            pop_up_offset,
            config.screen_w - 2.0 * pop_up_offset,
            config.screen_h - 2.0 * pop_up_offset,
        ],
        c.transform,
        g,
    );

    // Game over text
    render_text_center(
        config.snake_first_color,
        game_over_font_size,
        "Game Over",
        glyphs,
        120.0,
        c,
        g,
        &config,
    );

    // Render current score
    let current_score_text = format!("Score: {}", score);

    render_text_center(
        config.food_color,
        font_size,
        current_score_text.as_str(),
        glyphs,
        200.0,
        c,
        g,
        &config,
    );

    // Render high score
    let high_score_text = format!("High Score: {}", high_score);

    render_text_center(
        config.food_color,
        font_size,
        high_score_text.as_str(),
        glyphs,
        280.0,
        c,
        g,
        &config,
    );

    // Render info
    render_text_center(
        config.snake_first_color,
        24,
        "Press space to restart!",
        glyphs,
        360.0,
        c,
        g,
        &config,
    );
}

fn lerp(from: f32, to: f32, step_count: usize, current_step: f32) -> f32 {
    let should_increment = to > from;
    let current_addition = (f32::abs(from - to) / step_count as f32) * current_step as f32;

    if should_increment {
        return from + current_addition;
    } else {
        return from - current_addition;
    }
}
