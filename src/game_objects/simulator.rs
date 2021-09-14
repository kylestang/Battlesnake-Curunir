use crate::board::Board;
use crate::constants::{DIRECTIONS, DRAWING};
use crate::engine::Engine;
use crate::evaluator::Evaluator;
use crate::ruleset::Ruleset;

pub struct Simulator {
    ruleset: Ruleset
}

impl Simulator {
    pub fn new(ruleset: Ruleset) -> Simulator {
        Simulator {
            ruleset
        }
    }

    // Moves self down and predicts future turns
    pub fn check_down(self, mut board: Board, current_level: i32, max_level: i32) -> Vec<u64> {
        let snake = &mut board.get_snakes_mut()[0];
        let down = snake.get_down();
        snake.move_to(down);
        self.recursion_entry(board, current_level, max_level)
    }

    // Moves self up and predicts future turns
    pub fn check_up(self, mut board: Board, current_level: i32, max_level: i32) -> Vec<u64> {
        let snake = &mut board.get_snakes_mut()[0];
        let up = snake.get_up();
        snake.move_to(up);
        self.recursion_entry(board, current_level, max_level)
    }

    // Moves self right and predicts future turns
    pub fn check_right(self, mut board: Board, current_level: i32, max_level: i32) -> Vec<u64> {
        let snake = &mut board.get_snakes_mut()[0];
        let right = snake.get_right();
        snake.move_to(right);
        self.recursion_entry(board, current_level, max_level)
    }

    // Moves self left and predicts future turns
    pub fn check_left(self, mut board: Board, current_level: i32, max_level: i32) -> Vec<u64> {
        let snake = &mut board.get_snakes_mut()[0];
        let left = snake.get_left();
        snake.move_to(left);
        self.recursion_entry(board, current_level, max_level)
    }

    // First level of recursion, my snake has already moved
    fn recursion_entry(self, board: Board, current_level: i32, max_level: i32) -> Vec<u64> {
        if DRAWING {
            board.draw(String::from("test")).unwrap();
        }

        // End case. Return if self is dead or current_level >= max_level
        if current_level >= max_level || board.get_snakes().is_empty() {
            return Evaluator::new(board).evaluate();
        }

        let num_snakes = board.get_snakes().len();
        let mut worst_boards: Vec<[i32; DIRECTIONS]> = vec![[-1; DIRECTIONS]; num_snakes - 1];
        let mut result_boards: Vec<Vec<u64>> =
            Vec::with_capacity(DIRECTIONS.pow(num_snakes as u32 - 1));

        // Iterate through all possible boards
        for i in 0..DIRECTIONS.pow(num_snakes as u32 - 1) {
            // Create new Board to modify
            let mut new_board = board.clone();

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Move each snake to new position on new_board
            for j in 0..num_snakes - 1 {
                let snake = &mut new_board.get_snakes_mut()[j + 1];
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Update new_board
            Engine::new().game_step(&mut new_board, &self.ruleset);

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Get the maximin result from this position
            let result = self.minimax(new_board, current_level + 1, max_level);

            // Update worst outcomes
            for (j, snake_boards) in worst_boards.iter_mut().enumerate() {
                let direction = (i / DIRECTIONS.pow(j as u32)) % DIRECTIONS;
                let current_worst = snake_boards[direction];
                let id = board.get_snakes()[j + 1].get_id() as usize;

                if current_worst == -1 || result[id] < result_boards[current_worst as usize][id] {
                    snake_boards[direction] = i as i32;
                }
            }
            // Store calculated board
            result_boards.push(result);
        }

        if DRAWING {
            board.draw(String::from("test")).unwrap();
        }

        // Find the index of the board to return
        let mut return_board = 0;

        // Iterate over the worst boards for each snake
        for (i, snake_boards) in worst_boards.iter().enumerate() {
            let mut best_direction = 0;
            let id = board.get_snakes()[i + 1].get_id() as usize;

            // Find the best of the worst directions
            for j in 1..DIRECTIONS {
                if result_boards[snake_boards[j] as usize][id]
                    > result_boards[snake_boards[best_direction] as usize][id]
                {
                    best_direction = j;
                }
            }

            return_board += best_direction * DIRECTIONS.pow(i as u32);
        }

        // Return the best board
        result_boards.swap_remove(return_board)
    }

    // Recursive minimax-ish to find score of position
    pub fn minimax(&self, board: Board, current_level: i32, max_level: i32) -> Vec<u64> {
        if DRAWING {
            board.draw(String::from("test")).unwrap();
        }

        // End case. Return if all snakes are dead or current_level >= max_level
        if current_level >= max_level || board.get_snakes().is_empty() {
            return Evaluator::new(board).evaluate();
        }

        let num_snakes = board.get_snakes().len();
        let mut worst_boards: Vec<[i32; DIRECTIONS]> = vec![[-1; DIRECTIONS]; num_snakes];
        let mut result_boards: Vec<Vec<u64>> =
            Vec::with_capacity(DIRECTIONS.pow(num_snakes as u32));

        // Iterate through all possible boards
        for i in 0..DIRECTIONS.pow(num_snakes as u32) {
            // Create new Board to modify
            let mut new_board = board.clone();
            // Move each snake to new position on new_board
            for j in 0..num_snakes {
                let snake = &mut new_board.get_snakes_mut()[j];
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Update new_board
            Engine::new().game_step(&mut new_board, &self.ruleset);

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Get the maximin result from this position
            let result = self.minimax(new_board, current_level + 1, max_level);

            // Update worst outcomes
            for (j, snake_boards) in worst_boards.iter_mut().enumerate() {
                let direction = (i / DIRECTIONS.pow(j as u32)) % DIRECTIONS;
                let current_worst = snake_boards[direction];
                let id = board.get_snakes()[j].get_id() as usize;

                if current_worst == -1 || result[id] < result_boards[current_worst as usize][id] {
                    snake_boards[direction] = i as i32;
                }
            }
            // Store calculated board
            result_boards.push(result);
        }

        if DRAWING {
            board.draw(String::from("test")).unwrap();
        }

        // Find the index of the board to return
        let mut return_board = 0;

        // Iterate over the worst boards for each snake
        for (i, snake_boards) in worst_boards.iter().enumerate() {
            let mut best_direction = 0;
            let id = board.get_snakes()[i].get_id() as usize;

            // Find the best of the worst directions
            for j in 1..DIRECTIONS {
                if result_boards[snake_boards[j] as usize][id]
                    > result_boards[snake_boards[best_direction] as usize][id]
                {
                    best_direction = j;
                }
            }

            // Add the best direction to the return_board
            return_board += best_direction * DIRECTIONS.pow(i as u32);
        }

        // Return the best board
        result_boards.swap_remove(return_board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_object;
    use crate::constants::_TEST_PATH;
    // minimax()
    #[test]
    fn test_minimax() {
        let board = load_object!(Board, "test_board-03", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "test_board-03", _TEST_PATH);

        let simulator = Simulator::new(ruleset);

        let result = simulator.minimax(board, 0, 6);

        assert!(result[0] > 0);
    }
}
