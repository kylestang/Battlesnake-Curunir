use crate::board::Board;
use crate::constants::{DIRECTIONS, DRAWING, YOU_ID};
use crate::ruleset::Ruleset;

impl Board {
    // Moves self down and predicts future turns
    pub fn check_down(mut self, ruleset: &Ruleset, current_level: i32, max_level: i32) -> Vec<u64> {
        let snake = &mut self.snakes[YOU_ID as usize];
        let down = snake.get_down();
        snake.move_to(down);
        self.recursion_entry(ruleset, current_level, max_level)
    }

    // Moves self up and predicts future turns
    pub fn check_up(mut self, ruleset: &Ruleset, current_level: i32, max_level: i32) -> Vec<u64> {
        let snake = &mut self.snakes[YOU_ID as usize];
        let up = snake.get_up();
        snake.move_to(up);
        self.recursion_entry(ruleset, current_level, max_level)
    }

    // Moves self right and predicts future turns
    pub fn check_right(
        mut self,
        ruleset: &Ruleset,
        current_level: i32,
        max_level: i32,
    ) -> Vec<u64> {
        let snake = &mut self.snakes[YOU_ID as usize];
        let right = snake.get_right();
        snake.move_to(right);
        self.recursion_entry(ruleset, current_level, max_level)
    }

    // Moves self left and predicts future turns
    pub fn check_left(mut self, ruleset: &Ruleset, current_level: i32, max_level: i32) -> Vec<u64> {
        let snake = &mut self.snakes[YOU_ID as usize];
        let left = snake.get_left();
        snake.move_to(left);
        self.recursion_entry(ruleset, current_level, max_level)
    }

    // First level of recursion, my snake has already moved
    fn recursion_entry(self, ruleset: &Ruleset, current_level: i32, max_level: i32) -> Vec<u64> {
        if DRAWING {
            self.draw(String::from("test")).unwrap();
        }

        // End case. Return if all snakes are dead or current_level >= max_level
        if current_level >= max_level || self.snakes.is_empty() {
            return self.evaluate();
        }

        let num_snakes = self.snakes.len();
        let mut worst_boards: Vec<[i32; DIRECTIONS]> = vec![[-1; DIRECTIONS]; num_snakes - 1];
        let mut result_boards: Vec<Vec<u64>> =
            Vec::with_capacity(DIRECTIONS.pow(num_snakes as u32 - 1));

        // Iterate through all possible boards
        for i in 0..DIRECTIONS.pow(num_snakes as u32 - 1) {
            // Create new Board to modify
            let mut new_board = self.clone();

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Move each snake to new position on new_board
            for j in 0..num_snakes - 1 {
                let snake = &mut new_board.snakes[j + 1];
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Update new_board
            new_board.game_step(ruleset);

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Get the maximin result from this position
            let result = self.minimax(ruleset, current_level + 1, max_level);

            // Update worst outcomes
            for (j, snake_boards) in worst_boards.iter_mut().enumerate() {
                let direction = (i / DIRECTIONS.pow(j as u32)) % DIRECTIONS;
                let current_worst = snake_boards[direction];
                let id = self.snakes[j + 1].get_id() as usize;

                if current_worst == -1 || result[id] < result_boards[current_worst as usize][id] {
                    snake_boards[direction] = i as i32;
                }
            }
            // Store calculated board
            result_boards.push(result);
        }

        if DRAWING {
            self.draw(String::from("test")).unwrap();
        }

        // Find the index of the board to return
        let mut return_board = 0;

        // Iterate over the worst boards for each snake
        for (i, snake_boards) in worst_boards.iter().enumerate() {
            let mut best_direction = 0;
            let id = self.snakes[i + 1].get_id() as usize;

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
    pub fn minimax(&self, ruleset: &Ruleset, current_level: i32, max_level: i32) -> Vec<u64> {
        if DRAWING {
            self.draw(String::from("test")).unwrap();
        }

        // End case. Return if all snakes are dead or current_level >= max_level
        if current_level >= max_level || self.snakes.is_empty() {
            return self.evaluate();
        }

        let num_snakes = self.snakes.len();
        let mut worst_boards: Vec<[i32; DIRECTIONS]> = vec![[-1; DIRECTIONS]; num_snakes];
        let mut result_boards: Vec<Vec<u64>> =
            Vec::with_capacity(DIRECTIONS.pow(num_snakes as u32));

        // Iterate through all possible boards
        for i in 0..DIRECTIONS.pow(num_snakes as u32) {
            // Create new Board to modify
            let mut new_board = self.clone();
            // Move each snake to new position on new_board
            for j in 0..num_snakes {
                let snake = &mut new_board.snakes[j];
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Update new_board
            new_board.game_step(ruleset);

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Get the maximin result from this position
            let result = new_board.minimax(ruleset, current_level + 1, max_level);

            // Update worst outcomes
            for (j, snake_boards) in worst_boards.iter_mut().enumerate() {
                let direction = (i / DIRECTIONS.pow(j as u32)) % DIRECTIONS;
                let current_worst = snake_boards[direction];
                let id = self.snakes[j].get_id() as usize;

                if current_worst == -1 || result[id] < result_boards[current_worst as usize][id] {
                    snake_boards[direction] = i as i32;
                }
            }
            // Store calculated board
            result_boards.push(result);
        }

        if DRAWING {
            self.draw(String::from("test")).unwrap();
        }

        // Find the index of the board to return
        let mut return_board = 0;

        // Iterate over the worst boards for each snake
        for (i, snake_boards) in worst_boards.iter().enumerate() {
            let mut best_direction = 0;
            let id = self.snakes[i].get_id() as usize;

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
    use crate::constants::_TEST_PATH;
    use crate::load_object;
    // minimax()
    #[test]
    fn test_minimax() {
        let board = load_object!(Board, "test_board-03", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "test_board-03", _TEST_PATH);

        let result = board.minimax(&ruleset, 0, 6);

        assert!(result[0] > 0);
    }
}
