pub mod area_controlled;
pub mod draw;
pub mod evaluate;
pub mod game_step;
pub mod longest_path;
pub mod simulate;

use crate::battlesnake::Battlesnake;
use crate::constants::DIRECTIONS;
use crate::coordinate::Coordinate;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>,
    max_snakes: usize,
    turn: i32,
}

impl Board {
    pub fn new(
        height: i32,
        width: i32,
        food: Vec<Coordinate>,
        hazards: Vec<Coordinate>,
        snakes: Vec<Battlesnake>,
        max_snakes: usize,
        turn: i32,
    ) -> Board {
        Board {
            height,
            width,
            food,
            hazards,
            snakes,
            max_snakes,
            turn,
        }
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_food(&self) -> &Vec<Coordinate> {
        &self.food
    }

    pub fn get_food_mut(&mut self) -> &mut Vec<Coordinate> {
        &mut self.food
    }

    pub fn get_hazards(&self) -> &Vec<Coordinate> {
        &self.hazards
    }

    pub fn get_hazards_mut(&mut self) -> &mut Vec<Coordinate> {
        &mut self.hazards
    }

    pub fn get_snakes(&self) -> &Vec<Battlesnake> {
        &self.snakes
    }

    pub fn get_snakes_mut(&mut self) -> &mut Vec<Battlesnake> {
        &mut self.snakes
    }

    pub fn get_max_snakes(&self) -> usize {
        self.max_snakes
    }

    pub fn set_max_snakes(&mut self, max_snakes: usize) {
        self.max_snakes = max_snakes;
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }

    pub fn set_turn(&mut self, turn: i32) {
        self.turn = turn;
    }

    pub fn body_collision(&self, pos: Coordinate) -> bool {
        for snake in &self.snakes {
            if snake.get_body().contains(&pos) {
                return true;
            }
        }
        false
    }

    // Returns the closest food to pos
    pub fn find_closest_food(&self, pos: Coordinate) -> Option<Coordinate> {
        // If food exists
        if !self.food.is_empty() {
            let mut closest_food = self.food[0];
            let mut closest_distance = pos.distance_to(closest_food);
            // Iterate over food
            for &food in &self.food[1..] {
                let current_distance = pos.distance_to(food);
                if current_distance < closest_distance {
                    closest_distance = current_distance;
                    closest_food = food;
                }
            }
            // Return the closest food
            Some(closest_food)
        } else {
            None
        }
    }

    // Returns closest snake that I am longer than by advantage, if it exists
    pub fn find_weaker_snake(
        &self,
        current_snake: &Battlesnake,
        advantage: i32,
    ) -> Option<Coordinate> {
        let pos = current_snake.get_head();
        let mut closest_head = None;
        let mut closest_distance = i32::MAX;

        // Iterate through all snakes
        for snake in &self.snakes {
            // Check if snake is short enough, closer, and not the same as current_snake
            if snake.get_id() != current_snake.get_id() {
                let current_distance = pos.distance_to(snake.get_head());
                if snake.get_length() as i32 <= current_snake.get_length() as i32 - advantage
                    && current_distance < closest_distance
                {
                    closest_distance = current_distance;
                    closest_head = Some(snake.get_head());
                }
            }
        }

        // Return closest head matching the criteria, or None
        closest_head
    }

    // Returns the snake with id snake_id, or None
    pub fn get_snake(&self, snake_id: u8) -> Option<&Battlesnake> {
        for snake in &self.snakes {
            if snake.get_id() == snake_id {
                return Some(snake);
            }
        }
        None
    }

    pub fn increment_turn(&mut self) {
        self.turn += 1;
    }

    // Returns true if pos is against the board walls
    pub fn is_against_wall(&self, pos: Coordinate) -> bool {
        pos.get_x() == 0
            || pos.get_x() == self.width - 1
            || pos.get_y() == 0
            || pos.get_y() == self.height - 1
    }

    pub fn is_out_of_bounds(&self, pos: Coordinate) -> bool {
        pos.get_x() < 0
            || pos.get_x() >= self.width
            || pos.get_y() < 0
            || pos.get_y() >= self.height
    }

    pub fn open_directions(&self, snake: &Battlesnake) -> i32 {
        let mut options = DIRECTIONS as i32 + 1;
        let pos = snake.get_head();

        for snake in &self.snakes {
            for &tile in snake.get_body().range(..snake.get_length() - 1) {
                if tile == pos.get_down()
                    || tile == pos.get_up()
                    || tile == pos.get_left()
                    || tile == pos.get_right()
                {
                    options -= 1;
                }
            }
        }

        if pos.get_x() == 0 {
            options -= 1;
        }
        if pos.get_x() == self.width - 1 {
            options -= 1;
        }
        if pos.get_y() == 0 {
            options -= 1;
        }
        if pos.get_y() == self.height - 1 {
            options -= 1;
        }

        options
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::_TEST_PATH;
    use crate::load_object;
    // find_closest_food
    #[test]
    fn test_find_closest_food_none() {
        let board = load_object!(Board, "find_closest_food_none-01", _TEST_PATH);

        let food = board.find_closest_food(board.get_snakes()[0].get_head());

        assert!(food.is_none());
    }

    #[test]
    fn test_find_closest_food_one() {
        let board = load_object!(Board, "find_closest_food_one-01", _TEST_PATH);

        let food = board.find_closest_food(board.get_snakes()[0].get_head());

        assert_eq!(food.unwrap(), Coordinate::new(5, 5));
    }

    #[test]
    fn test_find_closest_food_two() {
        let board = load_object!(Board, "find_closest_food_two-01", _TEST_PATH);

        let food = board.find_closest_food(board.get_snakes()[0].get_head());

        assert_eq!(food.unwrap(), Coordinate::new(0, 3));
    }

    // find_weaker_snake()
    #[test]
    fn test_find_weaker_snake_none() {
        let board = load_object!(Board, "find_weaker_snake_none-01", _TEST_PATH);
        let snake = &board.get_snakes()[0];

        let snake_head = board.find_weaker_snake(snake, 5);

        assert!(snake_head.is_none());
    }

    #[test]
    fn test_find_weaker_snake_one() {
        let board = load_object!(Board, "find_weaker_snake_one-01", _TEST_PATH);
        let snake = &board.snakes[0];

        let snake_head = board.find_weaker_snake(snake, 5);

        assert_eq!(snake_head.unwrap(), board.snakes[1].get_head());
    }

    // get_snake()
    #[test]
    fn test_get_snake_none() {
        let board = load_object!(Board, "get_snake_none-01", _TEST_PATH);

        let snake = board.get_snake(1);

        assert!(snake.is_none());
    }

    #[test]
    fn test_get_snake_one() {
        let board = load_object!(Board, "get_snake_one-01", _TEST_PATH);

        let snake = board.get_snake(0);

        assert_eq!(snake.unwrap(), &board.get_snakes()[0]);
    }

    // is_against_wall()
    #[test]
    fn test_is_against_wall_false() {
        let board = load_object!(Board, "against_wall_false-01", _TEST_PATH);

        let result = board.is_against_wall(Coordinate::new(3, 2));

        assert!(!result);
    }

    #[test]
    fn test_is_against_wall_true() {
        let board = load_object!(Board, "against_wall_true-01", _TEST_PATH);

        let result = board.is_against_wall(Coordinate::new(0, 3));

        assert!(result);
    }

    // open_directions()
    #[test]
    fn test_open_directions_2() {
        let board = load_object!(Board, "open_directions_2-01", _TEST_PATH);
        let snake = &board.get_snakes()[0];

        assert_eq!(board.open_directions(snake), 2);
    }

    #[test]
    fn test_open_directions_3() {
        let board = load_object!(Board, "open_directions_3-01", _TEST_PATH);
        let snake = &board.get_snakes()[0];

        assert_eq!(board.open_directions(snake), 3);
    }
}
