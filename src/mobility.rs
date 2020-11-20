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
    pub point: (i32, i32),
}

impl Position {
    pub fn zero() -> Self {
        Self { point: (0, 0) }
    }

    pub fn teleport(&mut self, point: (i32, i32)) {
        self.point = point;
    }

    pub fn translate(&mut self, point: (i32, i32)) {
        self.point = (self.point.0 + point.0, self.point.1 + point.1);
    }

    pub fn check(&self, point: (i32, i32)) -> (i32, i32) {
        (self.point.0 + point.0, self.point.1 + point.1)
    }

    pub fn x(&self) -> i32 {
        self.point.0
    }

    pub fn y(&self) -> i32 {
        self.point.1
    }
}

