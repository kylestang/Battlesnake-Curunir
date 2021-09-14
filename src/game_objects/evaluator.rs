use crate::board::Board;
use crate::constants::LENGTH_ADVANTAGE;
use std::cmp::{max, min};

pub struct Evaluator {
    board: Board,
}

impl Evaluator {
    pub fn new(board: Board) -> Evaluator {
        Evaluator { board }
    }

    pub fn evaluate(&self) -> Vec<u64> {
        let mut result = Vec::with_capacity(self.board.get_max_snakes() as usize);
        let mut current_snake = 0;

        for i in 0..self.board.get_max_snakes() {
            let mut score: u64 = 0;
            if current_snake < self.board.get_snakes().len()
                && i == self.board.get_snakes()[current_snake].get_id()
            {
                let snake = &self.board.get_snakes()[current_snake];

                // digit 0
                let open_directions = self.board.open_directions(snake) as u64;
                score += open_directions;

                // digits 1,2
                let closest_food = self.board.find_closest_food(snake.get_head());
                if let Some(food_pos) = closest_food {
                    let value = max(0, 100 - snake.get_head().distance_to(food_pos));
                    score += 10 * value as u64;
                }

                // digits 3,4,5
                score += 1_000 * min(999, snake.get_length() as u64);

                // digits 6,7
                let weak_head = self.board.find_weaker_snake(snake, LENGTH_ADVANTAGE);
                if let Some(head_pos) = weak_head {
                    let value = max(0, 100 - snake.get_head().distance_to(head_pos));
                    score += 1_000_000 * value as u64;
                }

                // digit 8
                if open_directions >= 2 {
                    score += 100_000_000
                }

                // digits 9, 10
                score += 1_000_000_000 * max(0, 100 - self.board.get_snakes().len()) as u64;

                current_snake += 1;
            } else {
                score = 0;
            }

            result.push(score);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_object;
    use crate::constants::_TEST_PATH;

    // compare_to
    #[test]
    fn test_compare_to_advantage() {
        let better_board = load_object!(Board, "compare_to_advantage-01-better", _TEST_PATH);
        let worse_board = load_object!(Board, "compare_to_advantage-01-worse", _TEST_PATH);
        let better_evaluator = Evaluator::new(better_board);
        let worse_evaluator = Evaluator::new(worse_board);

        let true_result = better_evaluator.evaluate();
        let false_result = worse_evaluator.evaluate();

        assert!(true_result[0] > false_result[0]);
    }

    #[test]
    fn test_compare_to_alive() {
        let better_board = load_object!(Board, "better_than_alive-01-dead", _TEST_PATH);
        let worse_board = load_object!(Board, "better_than_alive-01-alive", _TEST_PATH);
        let better_evaluator = Evaluator::new(better_board);
        let worse_evaluator = Evaluator::new(worse_board);

        let true_result = better_evaluator.evaluate();
        let false_result = worse_evaluator.evaluate();

        assert!(true_result[0] > false_result[0]);
    }

        #[test]
        fn test_compare_to_dead() {
            let better_board = load_object!(Board, "better_than_dead-01-alive", _TEST_PATH);
            let mut worse_board = load_object!(Board, "better_than_dead-01-dead", _TEST_PATH);
            worse_board.set_max_snakes(2);
            
            let better_evaluator = Evaluator::new(better_board);
            let worse_evaluator = Evaluator::new(worse_board);

            let true_result = better_evaluator.evaluate();
            let false_result = worse_evaluator.evaluate();

            assert!(true_result[1] > false_result[1]);
        }

    #[test]
    fn test_better_than_food() {
        let better_board = load_object!(Board, "better_than_food-01-close", _TEST_PATH);
        let worse_board = load_object!(Board, "better_than_food-01-far", _TEST_PATH);
        let better_evaluator = Evaluator::new(better_board);
        let worse_evaluator = Evaluator::new(worse_board);

        let true_result = better_evaluator.evaluate();
        let false_result = worse_evaluator.evaluate();

        assert!(true_result[0] > false_result[0]);
    }

    #[test]
    fn test_better_than_long() {
        let better_board = load_object!(Board, "better_than_long-01-long", _TEST_PATH);
        let worse_board = load_object!(Board, "better_than_long-01-short", _TEST_PATH);
        let better_evaluator = Evaluator::new(better_board);
        let worse_evaluator = Evaluator::new(worse_board);

        let true_result = better_evaluator.evaluate();
        let false_result = worse_evaluator.evaluate();

        assert!(true_result[0] > false_result[0]);
    }
}
