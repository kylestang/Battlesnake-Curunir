use serde::{Deserialize, Serialize};
use crate::constants::DIRECTIONS;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate {x, y}
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    // Down, up, right, left
    pub fn get_adjacent(&self) -> [Coordinate; DIRECTIONS] {
        [
            self.get_down(),
            self.get_up(),
            self.get_right(),
            self.get_left()
        ]
    }

    pub fn get_down(&self) -> Coordinate {
        Coordinate::new(self.x, self.y - 1)
    }

    pub fn get_up(&self) -> Coordinate {
        Coordinate::new(self.x, self.y + 1)
    }

    pub fn get_right(&self) -> Coordinate {
        Coordinate::new(self.x + 1, self.y)
    }

    pub fn get_left(&self) -> Coordinate {
        Coordinate::new(self.x - 1, self.y)
    }
}
