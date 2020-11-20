// Globals

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

// Components

pub struct Walkable {
    pub step_size: i32,
}

pub struct Blocking;

#[derive(Hash, Eq, Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn teleport(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

