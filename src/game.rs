use crate::config::Config;
use crate::snake::{Direction, Node, Snake};
use piston::input::{Key, RenderArgs, UpdateArgs};
use piston_window::*;
use rand::thread_rng;
use rand::Rng;
use std::collections::LinkedList;

pub struct Game {
    pub config: Config,
    pub window: PistonWindow,
    pub snake: Snake,
    pub food: Node,
    pub obstacles: LinkedList<Node>,
    pub score: u16,
    pub high_score: u16,
}

impl Game {
    pub fn render(&mut self, _args: &RenderArgs, event: &Event, glyphs: &mut Glyphs) {
        const SEPERATOR_LINE_RADIUS: f64 = 0.5;

        let snake = &self.snake;
        let obstacles = &self.obstacles;
        let food = &self.food;
        let font_size: u32 = 32;
        let text_padding: f64 = 10.0;
        let score = &self.score;
        let high_score = &self.high_score;
        let config = &self.config;

        self.window.draw_2d(event, |c, g, device| {
            // Check if the snake is dead
            if !snake.is_alive {
                render_game_over(c, g, glyphs, *score, *high_score, *config);
                glyphs.factory.encoder.flush(device);
                return;
            }

            // Clear the screen
            clear(config.background_color, g);
            let num_of_cells_horizontal = (config.screen_w / config.cell_w) as i32;
            let num_of_cells_vertical = (config.screen_h / config.cell_w) as i32;

            // Draw the food
            rectangle(
                config.food_color,
                [
                    config.cell_w * food.x,
                    config.cell_w * food.y,
                    config.cell_w,
                    config.cell_w,
                ],
                c.transform,
                g,
            );

            // Draw the snake
            let mut node_index = 1.0;
            let mut snake_first_color = config.snake_first_color;
            let mut snake_second_color = config.snake_second_color;

            if snake.is_turbo {
                snake_first_color = config.snake_turbo_first_color;
                snake_second_color = config.snake_turbo_second_color;
            }

            for node in snake.nodes.iter().rev() {
                rectangle(
                    [
                        lerp(*snake_second_color.get(0).unwrap(),
                            *snake_first_color.get(0).unwrap(), snake.nodes.len(), node_index),
                        lerp(*snake_second_color.get(1).unwrap(),
                            *snake_first_color.get(1).unwrap(), snake.nodes.len(), node_index),
                        lerp(*snake_second_color.get(2).unwrap(),
                            *snake_first_color.get(2).unwrap(), snake.nodes.len(), node_index),
                        lerp(*snake_second_color.get(3).unwrap(),
                            *snake_first_color.get(3).unwrap(), snake.nodes.len(), node_index)
                    ],
                    [
                        config.cell_w * node.x,
                        config.cell_w * node.y,
                        config.cell_w,
                        config.cell_w,
                    ],
                    c.transform,
                    g,
                );

                node_index += 1.0;
            }

            // Redraw the head in a different color
            rectangle(
                config.snake_head_color,
                [
                    config.cell_w * snake.nodes.front().unwrap().x,
                    config.cell_w * snake.nodes.front().unwrap().y,
                    config.cell_w,
                    config.cell_w,
                ],
                c.transform,
                g,
            );

            // Draw the obstacles
            for obstacle in obstacles.iter() {
                rectangle(
                    config.obstacle_color,
                    [
                        config.cell_w * obstacle.x,
                        config.cell_w * obstacle.y,
                        config.cell_w,
                        config.cell_w,
                    ],
                    c.transform,
                    g,
                );

                node_index += 1.0;
            }

            // Draw the seperator lines
            for i in 1..num_of_cells_horizontal {
                line_from_to(
                    config.seperator_line_color,
                    SEPERATOR_LINE_RADIUS,
                    [config.cell_w * i as f64, 0.0],
                    [config.cell_w * i as f64, config.screen_h],
                    c.transform,
                    g,
                );
            }

            for i in 1..num_of_cells_vertical {
                line_from_to(
                    config.seperator_line_color,
                    SEPERATOR_LINE_RADIUS,
                    [0.0, config.cell_w * i as f64],
                    [config.screen_w, config.cell_w * i as f64],
                    c.transform,
                    g,
                );
            }

            text(
                config.snake_first_color,
                font_size,
                score.to_string().as_str(),
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
                // Update the snakes location
                match self.snake.direction {
                    Direction::Up => self.snake.update_node_locations(0.0, -1.0, self.config),
                    Direction::Down => self.snake.update_node_locations(0.0, 1.0, self.config),
                    Direction::Right => self.snake.update_node_locations(1.0, 0.0, self.config),
                    Direction::Left => self.snake.update_node_locations(-1.0, 0.0, self.config),
                };

                // Check if the snake did bite itself
                let mut game_over = false;
                let mut snake_nodes_iter = self.snake.nodes.iter();
                let head = snake_nodes_iter.next().unwrap().clone();

                while let Some(node) = snake_nodes_iter.next() {
                    if head.x == node.x && head.y == node.y {
                        // Bit itself...
                        game_over = true;
                        break;
                    }
                }

                // Check if the snake hit an obstacle
                let mut obstacles_iter = self.obstacles.iter();

                while let Some(node) = obstacles_iter.next() {
                    if head.x == node.x && head.y == node.y {
                        // Hit an obstacle
                        game_over = true;
                        break;
                    }
                }

                if game_over {
                    self.game_over();
                }

                // Check if the snake has eaten the food
                if head.x == self.food.x && head.y == self.food.y {
                    // Just push back a new random node
                    // it will be updated automatically
                    if self.snake.is_turbo {
                        self.score += 2;
                    } else {
                        self.score += 1;
                    }
                    self.snake.nodes.push_back(Node { x: -1.0, y: -1.0 });
                    self.place_random_food();
                }
            }
        }
    }

    pub fn handle_key_press(&mut self, key: &Key) {
        if !self.snake.is_alive {
            if *key == Key::Space {
                // Create the snake
                let mut nodes: LinkedList<Node> = LinkedList::new();
                nodes.push_back(Node { x: 10.0, y: 10.0 });
                nodes.push_back(Node { x: 11.0, y: 10.0 });
                nodes.push_back(Node { x: 12.0, y: 10.0 });

                let direction = Direction::Left;
                let snake = Snake {
                    nodes,
                    direction,
                    is_alive: true,
                    is_turbo: false,
                    movement_delay: 80.0,
                    last_movement_duration: 0.0,
                };
                self.score = 0;
                self.snake = snake;
                self.obstacles = LinkedList::new();

                self.place_random_obstacles(self.config.random_obstacle_count);
                self.place_random_food();
            }
        } else {
            // Check for the turbo key
            if *key == Key::LShift {
                self.snake.is_turbo = true;
            }

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
    }

    pub fn handle_key_release(&mut self, key: &Key) {
        if self.snake.is_alive {
            if *key == Key::LShift {
                self.snake.is_turbo = false;
            }
        }
    }

    pub fn place_random_obstacles(&mut self, count: u32) {
        for _ in 0..count {
            let (rand_x, rand_y) = self.find_random_available_node();
            self.obstacles.push_back(Node {
                x: rand_x,
                y: rand_y,
            });
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
            let rand_x: i32 = rng.gen_range(0, x_len as i32);
            let rand_y: i32 = rng.gen_range(0, y_len as i32);

            let mut flag = true;

            // Check if the snake is on those coordinates
            let mut snake_nodes_iter = self.snake.nodes.iter();
            while let Some(current) = snake_nodes_iter.next() {
                if current.x == rand_x as f64 && current.y == rand_y as f64 {
                    flag = false;
                    break;
                }
            }

            // Check if the food is on those coordinates
            if self.food.x == rand_x as f64 && self.food.y == rand_y as f64 {
                flag = false;
            }

            // Check if one of the obstacles are on this coordinates
            let mut obstacles_iter = self.obstacles.iter();
            while let Some(current) = obstacles_iter.next() {
                if current.x == rand_x as f64 && current.y == rand_y as f64 {
                    flag = false;
                    break;
                }
            }

            if flag {
                return (rand_x as f64, rand_y as f64);
            }
        }
    }

    fn game_over(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }

        self.snake.is_alive = false;
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
    let font_size: u32 = 32;
    let pop_up_offset = 50.0;
    let game_over_font_size: u32 = 48;

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
    let mut current_score_text = String::new();
    current_score_text.push_str("Score: ");
    current_score_text.push_str(score.to_string().as_str());

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
    let mut high_score_text = String::new();
    high_score_text.push_str("High Score: ");
    high_score_text.push_str(high_score.to_string().as_str());

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
        24 as u32,
        "Press space to restart!",
        glyphs,
        360.0,
        c,
        g,
        &config,
    );
}


fn lerp(from: f32, to: f32, step_count: usize, current_step: f32) -> f32{
    let should_increment = to > from;
    let current_addition = (f32::abs(from - to) / step_count as f32) * current_step as f32;

    if should_increment {
        return from + current_addition;
    } else {
        return from - current_addition;
    }
}
