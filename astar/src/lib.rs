use std::cell::RefCell;
use std::rc::Rc;

pub struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 {
            x,
            y
        }
    }

    pub fn up(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y + 1
        }
    }

    pub fn right(&self) -> Vector2 {
        Vector2 {
            x: self.x - 1,
            y: self.y
        }
    }

    pub fn down(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y - 1
        }
    }

    pub fn left(&self) -> Vector2 {
        Vector2 {
            x: self.x - 1,
            y: self.y
        }
    }
}

pub struct Node {
    pos: Vector2,
    edges: Rc<RefCell<Node>>
}

pub enum Tile {
    Blank,
    StartingPoint,
    Floor,
    Wall,
    Goal
}

pub struct MazeMap {
    tiles: Vec<Tile>,
}

impl MazeMap {
    pub fn generate(row: i32, col: i32) -> MazeMap {
        MazeMap {
            tiles: vec![]
        }
    }

    pub fn get_tile(&self, pos: Vector2) -> &Tile {
        &self.tiles[0]
    }
}