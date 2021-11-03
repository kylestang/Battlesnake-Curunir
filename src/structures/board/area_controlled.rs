use std::collections::VecDeque;

use crate::board::Board;
use crate::constants::{DIRECTIONS, YOU_ID};
use crate::ruleset::Ruleset;

impl Board {
    pub fn area_controlled(&self) -> Vec<i32> {
        #[derive(Clone, Copy, PartialEq)]
        enum TileStatus {
            Empty,
            Gone,
            Taken(u8),
        }

        // Initialization
        let mut areas = vec![0; self.max_snakes];

        let mut queue = VecDeque::with_capacity(self.height as usize);

        // pos(x,y) = grid[board.get_width() * y + x]
        let mut grid = vec![TileStatus::Empty; (self.height * self.width) as usize];

        for snake in &self.snakes {
            queue.push_back((snake.get_id(), snake.get_head()));

            grid[(self.width * snake.get_head().get_y() + snake.get_head().get_x()) as usize] =
                TileStatus::Taken(snake.get_id());

            for pos in snake.get_body().range(1..snake.get_body().len() - 1) {
                grid[(self.width * pos.get_y() + pos.get_x()) as usize] = TileStatus::Gone;
            }
        }

        while let Some((current_snake_id, current_pos)) = queue.pop_front() {
            if grid[(self.width * current_pos.get_y() + current_pos.get_x()) as usize]
                != TileStatus::Gone
            {
                for &pos in current_pos
                    .get_adjacent()
                    .iter()
                    .filter(|&&pos| !self.is_out_of_bounds(pos))
                {
                    let grid_value = (self.width * pos.get_y() + pos.get_x()) as usize;

                    match grid[grid_value] {
                        TileStatus::Empty => {
                            grid[grid_value] = TileStatus::Taken(current_snake_id);
                            queue.push_back((current_snake_id, pos));
                            areas[current_snake_id as usize] += 1;
                        }
                        TileStatus::Gone => (),
                        TileStatus::Taken(other_snake_id) => {
                            if current_snake_id != other_snake_id
                                && self.get_snake(current_snake_id).unwrap().get_length()
                                    == self.get_snake(other_snake_id).unwrap().get_length()
                            {
                                grid[grid_value] = TileStatus::Gone;
                                areas[other_snake_id as usize] -= 1;
                            }
                        }
                    }
                }
            }
        }
        areas
    }

    pub fn calculate_areas(&self, ruleset: &Ruleset) -> [i32; 4] {
        let mut values = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        let num_snakes = self.snakes.len();
        let iterations = DIRECTIONS.pow(num_snakes as u32);

        for i in 0..iterations {
            let mut new_board = self.clone();
            let mut direction = 0;

            for j in 0..num_snakes {
                let snake = &mut new_board.get_snakes_mut()[j];
                if snake.get_id() == YOU_ID {
                    direction = snake.get_direction((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                }
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            new_board.game_step(ruleset);

            let area = new_board.area_controlled()[YOU_ID as usize];

            values[direction].push(area);
        }

        [
            *values[0].iter().min().unwrap_or(&0),
            *values[1].iter().min().unwrap_or(&0),
            *values[2].iter().min().unwrap_or(&0),
            *values[3].iter().min().unwrap_or(&0),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::_TEST_PATH;
    use crate::load_object;
    // area_controlled
    #[test]
    fn test_area_controlled_one() {
        let mut board = load_object!(Board, "simple-01", _TEST_PATH);

        board
            .get_snakes_mut()
            .sort_unstable_by_key(|snake| 0 - snake.get_length() as i32);

        let result = board.area_controlled();
        let mut correct = Vec::new();
        correct.insert(0, 47);
        assert_eq!(result, correct)
    }

    #[test]
    fn test_area_controlled_two() {
        let mut board = load_object!(Board, "simple-02", _TEST_PATH);
        board
            .get_snakes_mut()
            .sort_unstable_by_key(|snake| 0 - snake.get_length() as i32);

        let result = board.area_controlled();
        let mut correct = Vec::new();
        correct.insert(0, 19);
        correct.insert(1, 19);
        assert_eq!(result, correct)
    }

    #[test]
    fn test_area_controlled_three() {
        let mut board = load_object!(Board, "test_board-04", _TEST_PATH);
        board
            .get_snakes_mut()
            .sort_unstable_by_key(|snake| 0 - snake.get_length() as i32);

        let result = board.area_controlled();
        let mut correct = Vec::new();
        correct.insert(0, 32);
        correct.insert(1, 39);
        correct.insert(2, 17);
        assert_eq!(result, correct)
    }

    // calculate_areas
    #[test]
    fn test_calculate_areas_one() {
        let board = load_object!(Board, "simple-01", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "simple-01", _TEST_PATH);

        let areas = board.calculate_areas(&ruleset);

        assert_eq!(areas, [0, 47, 47, 47]);
    }

    #[test]
    fn test_calculate_areas_two() {
        let board = load_object!(Board, "test_board-04", _TEST_PATH);
        let ruleset = load_object!(Ruleset, "test_board-04", _TEST_PATH);

        let areas = board.calculate_areas(&ruleset);

        assert_eq!(areas, [0, 0, 1, 30])
    }
}
