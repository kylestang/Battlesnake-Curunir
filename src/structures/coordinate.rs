use crate::constants::DIRECTIONS;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    // Returns distance from self to other
    pub fn distance_to(&self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    // Down, up, right, left
    pub fn get_adjacent(&self) -> [Coordinate; DIRECTIONS + 1] {
        [
            self.get_down(),
            self.get_up(),
            self.get_right(),
            self.get_left(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to() {
        let coord_1 = Coordinate::new(3, 5);
        let coord_2 = Coordinate::new(8, 4);

        let distance = coord_1.distance_to(coord_2);

        assert_eq!(distance, 6);
    }
}
