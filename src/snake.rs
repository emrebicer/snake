#[path = "config.rs"] mod config;
use config::*;
use std::collections::LinkedList;

pub struct Snake {
    pub nodes: LinkedList<Node>,
    pub direction: Direction,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub x: f64,
    pub y: f64,
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Snake {
    pub fn update_node_locations(self: &mut Snake, x_change: f64, y_change: f64) {

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
            head.x = SCREEN_W / CELL_W;
        } else if head.x == SCREEN_W / CELL_W {
            head.x = 0.0;
        }

        if head.y == -1.0 {
            head.y = SCREEN_H / CELL_W;
        } else if head.y == SCREEN_H / CELL_W {
            head.y = 0.0;
        }

        self.nodes = clone_nodes;
    }
}
