use std::collections::VecDeque;

use crate::board::Board;
use crate::constants::{DIRECTIONS, YOU_ID};
use crate::engine::Engine;
use crate::ruleset::Ruleset;

pub struct Mapper {
    board: Board,
}

impl Mapper {
    pub fn new(mut board: Board) -> Mapper {
        // Thanks to Hannah for the idea to sort by length
        board
            .get_snakes_mut()
            .sort_unstable_by_key(|snake| 0 - snake.get_length() as i32);

        Mapper { board }
    }

    pub fn area_controlled(&self, board: &mut Board) -> Vec<i32> {
        #[derive(Clone, Copy, PartialEq)]
        enum TileStatus {
            Empty,
            Gone,
            Taken(u8),
        }

        // Initialization
        let mut areas = vec![0; board.get_max_snakes()];

        let mut queue = VecDeque::with_capacity(board.get_height() as usize);

        // pos(x,y) = grid[board.get_width() * y + x]
        let mut grid = vec![TileStatus::Empty; (board.get_height() * board.get_width()) as usize];

        let board_width = board.get_width();
        for snake in board.get_snakes_mut() {
            queue.push_back((snake.get_id(), snake.get_head()));

            grid[(board_width * snake.get_head().get_y() + snake.get_head().get_x()) as usize] =
                TileStatus::Taken(snake.get_id());

            for pos in snake.get_body().range(1..snake.get_body().len() - 1) {
                grid[(board_width * pos.get_y() + pos.get_x()) as usize] = TileStatus::Gone;
            }
        }

        while let Some((current_snake_id, current_pos)) = queue.pop_front() {
            if grid[(board.get_width() * current_pos.get_y() + current_pos.get_x()) as usize]
                != TileStatus::Gone
            {
                for &pos in current_pos
                    .get_adjacent()
                    .iter()
                    .filter(|&&pos| !board.is_out_of_bounds(pos))
                {
                    let grid_value = (board.get_width() * pos.get_y() + pos.get_x()) as usize;

                    match grid[grid_value] {
                        TileStatus::Empty => {
                            grid[grid_value] = TileStatus::Taken(current_snake_id);
                            queue.push_back((current_snake_id, pos));
                            areas[current_snake_id as usize] += 1;
                        }
                        TileStatus::Gone => (),
                        TileStatus::Taken(other_snake_id) => {
                            if current_snake_id != other_snake_id
                                && board.get_snake(current_snake_id).unwrap().get_length()
                                    == board.get_snake(other_snake_id).unwrap().get_length()
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

        let num_snakes = self.board.get_snakes().len();
        let iterations = (DIRECTIONS + 1).pow(num_snakes as u32);

        for i in 0..iterations {
            let mut new_board = self.board.clone();
            let mut direction = 0;

            for j in 0..num_snakes {
                let snake = &mut new_board.get_snakes_mut()[j];
                if snake.get_id() == YOU_ID {
                    direction = (i / DIRECTIONS.pow(j as u32)) % DIRECTIONS;
                }
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            Engine::new().game_step(&mut new_board, ruleset);

            let area = self.area_controlled(&mut new_board)[YOU_ID as usize];

            values[direction].push(area);
        }

        for vector in &mut values {
            if vector.is_empty() {
                vector.push(0);
            } else {
                vector.sort_unstable();
            }
        }

        let median_index = values[0].len() / 2;

        [
            values[0][median_index],
            values[1][median_index],
            values[2][median_index],
            values[3][median_index],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::_TEST_PATH;
    use crate::load_object;
    // area_controlled
    #[test]
    fn test_area_controlled_one() {
        let mut board = load_object!(Board, "simple-01", _TEST_PATH);
        let mapper = Mapper::new(board.clone());

        board
            .get_snakes_mut()
            .sort_unstable_by_key(|snake| 0 - snake.get_length() as i32);

        let result = mapper.area_controlled(&mut board);
        let mut correct = Vec::new();
        correct.insert(0, 47);
        assert_eq!(result, correct)
    }

    #[test]
    fn test_area_controlled_two() {
        let mut board = load_object!(Board, "simple-02", _TEST_PATH);
        let mapper = Mapper::new(board.clone());

        board
            .get_snakes_mut()
            .sort_unstable_by_key(|snake| 0 - snake.get_length() as i32);

        let result = mapper.area_controlled(&mut board);
        let mut correct = Vec::new();
        correct.insert(0, 19);
        correct.insert(1, 19);
        assert_eq!(result, correct)
    }

    #[test]
    fn test_area_controlled_three() {
        let mut board = load_object!(Board, "test_board-04", _TEST_PATH);
        let mapper = Mapper::new(board.clone());

        board
            .get_snakes_mut()
            .sort_unstable_by_key(|snake| 0 - snake.get_length() as i32);

        let result = mapper.area_controlled(&mut board);
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
        let mapper = Mapper::new(board);

        let areas = mapper.calculate_areas(&ruleset);

        assert_eq!(areas, [0, 47, 47, 47]);
    }
}
