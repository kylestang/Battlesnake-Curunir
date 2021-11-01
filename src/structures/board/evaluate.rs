use crate::board::Board;
use crate::constants::LENGTH_ADVANTAGE;
use std::cmp::{max, min};

impl Board {
    pub fn evaluate(&self) -> Vec<u64> {
        let mut result = vec![0; self.max_snakes];

        for snake in &self.snakes {
            let mut score: u64 = 0;

            // digit 0
            let open_directions = self.open_directions(snake) as u64;
            score += open_directions;

            // digits 1,2
            let closest_food = self.find_closest_food(snake.get_head());
            if let Some(food_pos) = closest_food {
                let value = max(0, 100 - snake.get_head().distance_to(food_pos));
                score += 10 * value as u64;
            }

            // digits 3,4,5
            score += 1_000 * min(999, snake.get_length() as u64);

            // digits 6,7
            let weak_head = self.find_weaker_snake(snake, LENGTH_ADVANTAGE);
            if let Some(head_pos) = weak_head {
                let value = max(0, 100 - snake.get_head().distance_to(head_pos));
                score += 1_000_000 * value as u64;
            }

            // digit 8
            if open_directions >= 2 {
                score += 100_000_000
            }

            // digits 9, 10
            score += 1_000_000_000 * max(0, 100 - self.snakes.len()) as u64;

            result[snake.get_id() as usize] = score;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::_TEST_PATH;
    use crate::load_object;

    // compare_to
    #[test]
    fn test_compare_to_advantage() {
        let better_board = load_object!(Board, "compare_to_advantage-01-better", _TEST_PATH);
        let worse_board = load_object!(Board, "compare_to_advantage-01-worse", _TEST_PATH);

        assert!(better_board.evaluate()[0] > worse_board.evaluate()[0]);
    }

    #[test]
    fn test_compare_to_alive() {
        let better_board = load_object!(Board, "better_than_alive-01-dead", _TEST_PATH);
        let worse_board = load_object!(Board, "better_than_alive-01-alive", _TEST_PATH);

        assert!(better_board.evaluate()[0] > worse_board.evaluate()[0]);
    }

    #[test]
    fn test_compare_to_dead() {
        let better_board = load_object!(Board, "better_than_dead-01-alive", _TEST_PATH);
        let mut worse_board = load_object!(Board, "better_than_dead-01-dead", _TEST_PATH);
        worse_board.max_snakes = 2;

        assert!(better_board.evaluate()[1] > worse_board.evaluate()[1]);
    }

    #[test]
    fn test_better_than_food() {
        let better_board = load_object!(Board, "better_than_food-01-close", _TEST_PATH);
        let worse_board = load_object!(Board, "better_than_food-01-far", _TEST_PATH);

        assert!(better_board.evaluate()[0] > worse_board.evaluate()[0]);
    }

    #[test]
    fn test_better_than_long() {
        let better_board = load_object!(Board, "better_than_long-01-long", _TEST_PATH);
        let worse_board = load_object!(Board, "better_than_long-01-short", _TEST_PATH);

        assert!(better_board.evaluate()[0] > worse_board.evaluate()[0]);
    }
}
