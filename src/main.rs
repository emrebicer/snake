#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
mod game;
mod snake;

use game::Game;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use piston_window::{Glyphs, PistonWindow, TextureSettings, WindowSettings};
use snake::{Direction, Node, Snake};
use std::collections::LinkedList;

fn main() {
    let config = config::load_config_file();

    let mut window: PistonWindow = WindowSettings::new("Snake", [config.screen_w, config.screen_h])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let font = include_bytes!("../assets/PlaymegamesReguler-2OOee.ttf");
    let mut glyphs = Glyphs::from_bytes(
        font,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .unwrap();

    // Create the snake
    let mut nodes: LinkedList<Node> = LinkedList::new();
    nodes.push_back(Node { x: 10.0, y: 10.0 });
    nodes.push_back(Node { x: 11.0, y: 10.0 });
    nodes.push_back(Node { x: 12.0, y: 10.0 });

    // Create obstacles
    let obstacles: LinkedList<Node> = LinkedList::new();

    let direction = Direction::Left;
    let food = Node { x: -1.0, y: -1.0 };
    let snake = Snake {
        nodes,
        direction,
        is_alive: true,
        is_turbo: false,
        movement_delay: 80.0,
        last_movement_duration: 0.0,
    };

    // Create a new game and run it.
    let mut game = Game {
        config,
        window,
        snake,
        food,
        obstacles,
        score: 0,
        high_score: 0,
    };

    game.place_random_food();
    game.place_random_obstacles(config.random_obstacle_count);

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut game.window) {
        if let Some(args) = e.render_args() {
            game.render(&args, &e, &mut glyphs);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.handle_key_press(&key)
        };

        if let Some(Button::Keyboard(key)) = e.release_args() {
            game.handle_key_release(&key)
        };
    }
}
