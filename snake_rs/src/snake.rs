use std::{collections::VecDeque, ops::Add};
pub struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    pub const UP: Direction = Direction { x: 0, y: 1};
    pub const DOWN: Direction = Direction { x: 0, y: -1};
    pub const LEFT: Direction = Direction { x: -1, y: 0};
    pub const RIGHT: Direction = Direction { x: 1, y: 0};
}

impl Add<Direction> for (usize,usize) {
    type Output = (isize,isize);

    fn add(self, rhs: Direction) -> Self::Output {
        (self.0 as isize+rhs.x, self.1 as isize+rhs.y)
    }
}
pub struct Snake {
    body: VecDeque<(usize,usize)>,
    direction: Direction,
}

impl Snake {
    pub fn new(body: VecDeque<(usize,usize)>, direction: Direction) -> Snake {
        Snake {body, direction}
    }

    pub fn head(&self) -> (usize,usize) {
        *self.body.back().unwrap()
    }

    pub fn body(&self) -> impl Iterator<Item = &(usize,usize)> {
        self.body.range(0..self.body.len()-1)
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn step(&mut self, position: (usize,usize)) {
        self.body.pop_front();
        self.body.push_back(position);
    }
}