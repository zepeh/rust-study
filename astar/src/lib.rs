use std::cell::RefCell;
use std::rc::{Rc};
use std::clone::Clone;
use std::cmp::Ordering;

pub enum Tile {
    Blank,
    StartingPoint,
    Floor,
    Wall,
    Goal
}

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
    edges: Vec<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(pos: Vector2) -> Node {
        Node {
            pos,
            edges: vec![]
        }
    }

    pub fn add_edge(&mut self, new_edge: Rc<RefCell<Node>>) {
        self.edges.push(new_edge);
    }
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

pub struct PriorityQueue<T> {
    arr: Vec<Option<T>>,
    size: usize,
}

impl<T: std::cmp::PartialOrd> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            arr: vec![],
            size: 0
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, node: T) {
        let mut cur_idx = self.size;

        if self.arr.len() == cur_idx {
            self.arr.push(Some(node));
        } else {
            self.arr[cur_idx] = Some(node);
        }

        self.size = self.size + 1;

        while cur_idx != 0 {
            let mut parent_idx = Self::get_parent(cur_idx);

            if self.arr[parent_idx] < self.arr[cur_idx] {
                self.arr.swap(parent_idx, cur_idx);
                cur_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;

        let mut root = self.arr[0].take();
        let mut cur_idx = self.size;

        self.arr.swap(0, cur_idx);
        cur_idx = 0;

        loop {
            let left_idx = Self::get_left_child(cur_idx);
            let right_idx = Self::get_right_child(cur_idx);

            if !self.arr[left_idx].is_none() && self.arr[left_idx] > self.arr[cur_idx] {
                self.arr.swap(cur_idx, left_idx);
                cur_idx = left_idx;
            } else if !self.arr[right_idx].is_none() && self.arr[right_idx] > self.arr[cur_idx] {
                self.arr.swap(cur_idx, right_idx);
                cur_idx = right_idx;
            } else {
                break;
            }
        }

        return root;
    }

    // pub fn peek(&self) -> Option<&T> {
    //     if self.size == 0 {
    //         None
    //     } else {
    //         Some(&self.arr[0].as_deref())
    //     }
    // }

    fn get_parent(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn get_left_child(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn get_right_child(idx: usize) -> usize {
        idx * 2 + 2
    }
}
