use serde::{Deserialize, Serialize};
use crate::constants::DIRECTIONS;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Serialize)]
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
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y)
        ]
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
