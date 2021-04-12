use std::collections::VecDeque;

use crate::constants::MAX_HEALTH;
use crate::coordinate::Coordinate;

// Define the Battlesnake struct
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Battlesnake {
    id: i32,
    health: i32,
    body: VecDeque<Coordinate>,
    latency: i32,
    head: Coordinate,
    length: usize
}

impl Battlesnake {
    // Battlesnake constructor
    pub fn new(
        id: i32, health: i32, body: VecDeque<Coordinate>, latency: i32,
        head: Coordinate, length: usize
    ) -> Battlesnake {
        Battlesnake {id, health, body, latency, head, length}
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_body(&self) -> &VecDeque<Coordinate> {
        &self.body
    }

    pub fn _get_latency(&self) -> i32 {
        self.latency
    }

    pub fn get_head(&self) -> Coordinate {
        self.head
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_down(&self) -> Coordinate {
        self.head.get_down()
    }

    pub fn get_up(&self) -> Coordinate {
        self.head.get_up()
    }

    pub fn get_right(&self) -> Coordinate {
        self.head.get_right()
    }

    pub fn get_left(&self) -> Coordinate {
        self.head.get_left()
    }

    // Returns true if self has collided with the body of other
    pub fn body_collision_with(&self, other: &Battlesnake) -> bool {
        other.get_body().range(1..).any(|tile| self.head == *tile)
    }

    // Eat food and update snake
    pub fn eat_food(&mut self) {
        // Reset health to full
        self.health = MAX_HEALTH;
        // Add piece to back of self
        self.body.push_back(*self.body.back().unwrap());
        // Increase length by 1
        self.length += 1;
    }

    // Return all tiles adjacent to head other than body[1]
    pub fn get_option(&self, direction: usize) -> Coordinate {
        let second = self.body[1];
        let result;

        if second == self.get_down() {
            result = match direction {
                0 => self.get_up(),
                1 => self.get_right(),
                2 => self.get_left(),
                _ => panic!("Wrong direction")
            };
        } else if second == self.get_up() {
            result = match direction {
                0 => self.get_down(),
                1 => self.get_right(),
                2 => self.get_left(),
                _ => panic!("Wrong direction")
            }
        } else if second == self.get_right() {
            result = match direction {
                0 => self.get_down(),
                1 => self.get_up(),
                2 => self.get_left(),
                _ => panic!("Wrong direction")
            }
        } else {
            result = match direction {
                0 => self.get_down(),
                1 => self.get_up(),
                2 => self.get_right(),
                _ => panic!("Wrong direction")
            }
        }

        result
    }

    // Returns true if self lost head-to-head against other
    pub fn lost_headon(&self, other: &Battlesnake) -> bool {
        self.id != other.get_id() && self.head == other.get_head() && self.length <= other.get_length()
    }

    // Move self to position pos
    pub fn move_to(&mut self, pos: Coordinate) {
        // Remove tail from self
        self.body.pop_back();
        // Add pos to front of self
        self.body.push_front(pos);
        // Set head to new pos
        self.head = pos;
        // Decrese health by 1
        self.health -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::load_object;

    // body_collision_with
    #[test]
    fn test_collision() {
        let board = load_object!(Board, "body_collision-01-before");
        let snake1 = &board.get_snakes()[0];
        let snake2 = &board.get_snakes()[1];

        assert_eq!(snake2.body_collision_with(snake1), true);
    }

    #[test]
    fn test_no_collision() {
        let board = load_object!(Board, "body_collision-01-before");
        let snake1 = &board.get_snakes()[0];
        let snake2 = &board.get_snakes()[1];
        
        assert_eq!(snake1.body_collision_with(snake2), false);
    }

    // eat_food
    #[test]
    fn test_eat_food() {
        let mut before_board = load_object!(Board, "eat-01-before");
        let after_board = load_object!(Board, "eat-01-after");
        let before_eat = &mut before_board.get_snakes_mut()[0];
        let after_eat = &after_board.get_snakes()[0];

        before_eat.eat_food();
        
        assert_eq!(before_eat, after_eat);
    }

    // lost_head_to_head
    #[test]
    fn test_lose_headon_collision() {
        let board = load_object!(Board, "headon_collision-01-before");
        let snake1 = &board.get_snakes()[0];
        let snake2 = &board.get_snakes()[1];

        assert_eq!(snake2.lost_headon(snake1), true);
    }

    #[test]
    fn test_no_headon_collision() {
        let board = load_object!(Board, "simple-02");
        let snake1 = &board.get_snakes()[0];
        let snake2 = &board.get_snakes()[1];

        assert_eq!(snake1.lost_headon(snake2), false);
    }

    #[test]
    fn test_win_headon_collision() {
        let board = load_object!(Board, "headon_collision-01-before");
        let snake1 = &board.get_snakes()[0];
        let snake2 = &board.get_snakes()[1];

        assert_eq!(snake1.lost_headon(snake2), false);
    }

    // move_to
    #[test]
    fn test_move_to() {
        let mut before_board = load_object!(Board, "move-01-before");
        let after_board = load_object!(Board, "move-01-after");
        let before_move = &mut before_board.get_snakes_mut()[0];
        let after_move = &after_board.get_snakes()[0];
        
        let destination = crate::coordinate::Coordinate::new(2, 3);
        before_move.move_to(destination);

        assert_eq!(before_move, after_move);
    }
}
