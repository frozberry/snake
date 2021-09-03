use crate::helpers::*;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: (i32, i32),
    pub tail: Vec<(i32, i32)>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            head: (10, 10),
            tail: vec![(11, 10), (12, 10), (31, 10), (14, 10)],
            direction: Direction::Up,
        }
    }

    pub fn tick(&mut self, fruit: bool) {
        let old_snake = self.head.clone();
        let old_tail = self.tail.clone();

        match self.direction {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Right => self.head.0 += 1,
        }
        if fruit {
            self.tail.insert(0, (self.head.0, self.head.1));
        }

        self.tail[0] = old_snake;
        for i in 1..self.tail.len() {
            self.tail[i] = old_tail[i - 1];
        }

        if self.tail.contains(&self.head) || !valid_coord(self.head.0, self.head.1) {
            self.head = (10, 10);
            self.tail = vec![(11, 10), (12, 10), (13, 10), (14, 10)];
            self.direction = Direction::Up;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
