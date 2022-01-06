use crate::config::Config;
use std::collections::LinkedList;

type Milliseconds = f64;

pub struct Snake {
    pub nodes: LinkedList<Node>,
    pub direction: Direction,
    pub is_alive: bool,
    pub is_turbo: bool,
    pub movement_delay: Milliseconds,
    pub last_movement_duration: Milliseconds
}

#[derive(Clone, Debug, Copy)]
pub struct Node {
    pub x: f64,
    pub y: f64,
}

impl Node {
    pub fn eq(&self, other_node: Node) -> bool {
        return self.x == other_node.x && self.y == other_node.y;
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Snake {
    pub fn update_node_locations(self: &mut Snake, x_change: f64, y_change: f64, config: Config) {

        let mut clone_nodes = self.nodes.clone();
        let mut iter = clone_nodes.iter_mut();
        let mut prev = iter.next().unwrap().clone();

        while let Some(current) = iter.next() {
            let temp = current.clone();
            current.x = prev.x;
            current.y = prev.y;
            prev = temp;
        }

        iter = clone_nodes.iter_mut();

        let head = iter.next().unwrap();
        head.x += x_change;
        head.y += y_change;

        if head.x == -1.0 {
            head.x = (config.screen_w / config.cell_w) - 1.0;
        } else if head.x == config.screen_w / config.cell_w {
            head.x = 0.0;
        }

        if head.y == -1.0 {
            head.y = (config.screen_h / config.cell_w) - 1.0;
        } else if head.y == config.screen_h / config.cell_w {
            head.y = 0.0;
        }

        self.nodes = clone_nodes;
    }
}
