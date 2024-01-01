use std::cell::RefCell;
use std::rc::Rc;
use std::clone::Clone;
use std::cmp::Ordering;

pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y -1
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y
        }
    }
}

impl Clone for Vector2 {
    fn clone(&self) -> Self {
        Vector2::new(self.x, self.y)
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            // x 좌표가 같으면 y 좌표끼리 비교
            Some(Ordering::Equal) => self.y.partial_cmp(&other.y),
            // x 좌표가 다르면 그 결과를 반환
            _ => None
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
    size: Vector2,
}

impl MazeMap {
    pub fn generate(size: Vector2) -> MazeMap {
        let mut tiles : Vec<Tile> = vec![];

        for y in 0..size.y {
            for x in 0 .. size.x {
                tiles.push(Tile::Floor);
            }
        }

        tiles[0] = Tile::StartingPoint;
        *tiles.last_mut().unwrap() = Tile::Goal;

        MazeMap {
            tiles,
            size
        }
    }

    pub fn size(&self) -> Vector2 {
        self.size.clone()
    }

    pub fn get_tile(&self, pos: Vector2) -> &Tile {
        let idx = self.get_idx_from_pos(&pos);
        &self.tiles[idx]
    }

    pub fn set_tile(&mut self, pos: Vector2, tile: Tile) {
        let idx = self.get_idx_from_pos(&pos);
        self.tiles[idx] = tile;
    }

    fn get_idx_from_pos(&self, pos: &Vector2) -> usize {
        (pos.y * self.size.x + pos.x) as usize
    }
}