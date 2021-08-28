mod config;
mod snake;
mod game;

use config::*;
use game::*;
use game::snake::*;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, PressEvent, RenderEvent, UpdateEvent};
use piston_window::*;
use std::collections::LinkedList;


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

    // Create obstacles
    let obstacles: LinkedList<Node> = LinkedList::new();

    let direction = Direction::Left;
    let food = Node { x: -1.0, y: -1.0 };
    let snake = Snake { nodes, direction, is_alive: true };

    // Create a new game and run it.
    let mut game = Game {
        window,
        snake,
        food,
        obstacles,
        score: 0,
        high_score: 0,
    };

    game.place_random_food();
    game.place_random_obstacles(10);

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
