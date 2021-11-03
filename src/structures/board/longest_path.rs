use crate::board::Board;
use crate::constants::YOU_ID;
use crate::coordinate::Coordinate;

impl Board {
    // Find the longest possible route a snake can travel from the current position
    pub fn longest_path(
        &self,
        pos: Coordinate,
        mut current_area: i32,
        max_area: i32,
        gone: &mut Vec<Coordinate>,
        mut food_eaten: usize,
    ) -> i32 {
        // Reached end of search, return
        if current_area >= max_area {
            return current_area;
        }

        // Check out of bounds
        if self.is_out_of_bounds(pos) {
            return current_area;
        }

        // Check if tile has already been visited
        if gone.contains(&pos) {
            return current_area;
        }

        // Increment food counter
        if self.food.contains(&pos) {
            food_eaten += 1;
        }

        // Check for snake collisions, return max_area if I can tail chase
        for snake in &self.snakes {
            let body = snake.get_body();
            // Iterate over snake body
            for (i, tile) in body.iter().enumerate() {
                if pos == *tile {
                    // If snake is me, subtract food from area. Return available area
                    if snake.get_id() == YOU_ID {
                        if snake.get_length() - i - 1 > current_area as usize - food_eaten {
                            return current_area;
                        } else {
                            return max_area;
                        }
                    } else if snake.get_length() - i - 1 > current_area as usize {
                        return current_area;
                    } else {
                        return max_area;
                    }
                }
            }
        }

        current_area += 1;
        gone.push(pos);

        // Find the largest area from the current position
        let mut largest_area = current_area;
        for tile in &pos.get_adjacent() {
            // Discard paths of alternate routes, keep paths used to get here
            gone.truncate(current_area as usize);
            let new_area = self.longest_path(*tile, current_area, max_area, gone, food_eaten);
            if new_area >= max_area {
                return new_area;
            }
            if new_area > largest_area {
                largest_area = new_area;
            }
        }

        largest_area
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::_TEST_PATH;
    use crate::load_object;
    // longest_path
    #[test]
    fn test_longest_path_closed() {
        let board = load_object!(Board, "check_area_closed-01", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_left();

        let result = board.longest_path(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_longest_path_miss() {
        let board = load_object!(Board, "check_area_closed-02", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.longest_path(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_longest_path_hit() {
        let board = load_object!(Board, "check_area_open-02", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.longest_path(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_longest_path_food() {
        let board = load_object!(Board, "check_area_closed-03", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.longest_path(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_longest_path_open() {
        let board = load_object!(Board, "check_area_open-01", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_up();

        let result = board.longest_path(pos, 0, 30, &mut Vec::with_capacity(30), 0);

        assert_eq!(result, 30);
    }

    #[test]
    fn test_longest_path_route() {
        let board = load_object!(Board, "check_area_route-01", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.longest_path(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_longest_path_real() {
        let board = load_object!(Board, "check_area_route-02", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.longest_path(pos, 0, 27, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 27);
    }

    #[test]
    fn test_longest_path_tail() {
        let board = load_object!(Board, "check_area_route-03", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.longest_path(pos, 0, 6, &mut Vec::with_capacity(6), 0);

        assert_eq!(result, 6);
    }
}
