use image::{ImageResult, Rgb, RgbImage};
use std::cmp::{max, min};

use crate::battlesnake::Battlesnake;
use crate::constants::{
    DIRECTIONS, DRAWING, DRAW_PATH, EYE_RATIO, FOOD_RATIO, LENGTH_ADVANTAGE, PUPIL_RATIO,
    TILE_SIZE, YOU_ID,
};
use crate::coordinate::Coordinate;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>,
    max_snakes: u8,
    turn: i32,
}

impl Board {
    pub fn new(
        height: i32,
        width: i32,
        food: Vec<Coordinate>,
        hazards: Vec<Coordinate>,
        snakes: Vec<Battlesnake>,
        max_snakes: u8,
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

    pub fn _get_height(&self) -> i32 {
        self.height
    }

    pub fn _get_width(&self) -> i32 {
        self.width
    }

    pub fn get_food(&self) -> &Vec<Coordinate> {
        &self.food
    }

    pub fn _get_hazards(&self) -> &Vec<Coordinate> {
        &self.hazards
    }

    pub fn get_snakes(&self) -> &Vec<Battlesnake> {
        &self.snakes
    }

    pub fn get_snakes_mut(&mut self) -> &mut Vec<Battlesnake> {
        &mut self.snakes
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }

    // Find the longest possible route a snake can travel from the current position
    pub fn check_area(
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
        if pos.get_x() < 0
            || pos.get_x() > self.width - 1
            || pos.get_y() < 0
            || pos.get_y() > self.height - 1
        {
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
        let mut largest_area = 0;
        for tile in &pos.get_adjacent() {
            // Discard paths of alternate routes, keep paths used to get here
            gone.truncate(current_area as usize);
            let new_area = self.check_area(*tile, current_area, max_area, gone, food_eaten);
            if new_area >= max_area {
                return new_area;
            }
            if new_area > largest_area {
                largest_area = new_area;
            }
        }

        largest_area
    }

    pub fn draw(&self, file_name: String) -> ImageResult<()> {
        let imgx = TILE_SIZE * self.width as u32;
        let imgy = TILE_SIZE * self.height as u32;

        let mut img = RgbImage::new(imgx, imgy);

        for x in 0..self.width as u32 {
            for y in 0..self.height as u32 {
                // Fill in grid
                for tile_x in 0..TILE_SIZE {
                    for tile_y in 0..TILE_SIZE {
                        let r: u8;
                        let g: u8;
                        let b: u8;

                        if tile_x == 0 || tile_y == 0 {
                            b = 100;
                            g = 0;
                            r = 0;
                        } else {
                            b = 255;
                            g = 255;
                            r = 255;
                        }

                        let x_pixel = x * TILE_SIZE + tile_x;
                        let y_pixel = imgy - (y * TILE_SIZE + tile_y) - 1;
                        img.put_pixel(x_pixel, y_pixel, Rgb([r, g, b]));
                    }
                }
            }
        }

        // Draw food
        let food_radius = TILE_SIZE as f32 / FOOD_RATIO;
        for food in &self.food {
            for tile_x in 0..TILE_SIZE {
                for tile_y in 0..TILE_SIZE {
                    let radius = (((tile_x as i32 - TILE_SIZE as i32 / 2).pow(2)
                        + (tile_y as i32 - TILE_SIZE as i32 / 2).pow(2))
                        as f32)
                        .sqrt();
                    let x_pixel = food.get_x() as u32 * TILE_SIZE + tile_x;
                    let y_pixel = imgy - (food.get_y() as u32 * TILE_SIZE + tile_y) - 1;

                    if radius <= food_radius {
                        img.put_pixel(x_pixel, y_pixel, Rgb([255, 0, 0]));
                    }
                }
            }
        }

        // Draw snakes
        for snake in &self.snakes {
            let r1: u8 = ((snake.get_id() as u32 * 90) % 255) as u8;
            let g1: u8 = ((snake.get_id() as u32 * 150) % 255) as u8;
            let b1: u8 = ((snake.get_id() as u32 * 210) % 255) as u8;

            let r2: u8 = (((snake.get_id() as u32 + 100) * 90) % 255) as u8;
            let g2: u8 = (((snake.get_id() as u32 + 176) * 150) % 255) as u8;
            let b2: u8 = (((snake.get_id() as u32 + 95) * 210) % 255) as u8;

            for tile in snake.get_body() {
                if !(tile.get_x() < 0
                    || tile.get_x() > self.width - 1
                    || tile.get_y() < 0
                    || tile.get_y() > self.height - 1)
                {
                    if *tile == snake.get_head() {
                        let eye_radius = TILE_SIZE as f32 / EYE_RATIO;
                        let pupil_radius = TILE_SIZE as f32 / PUPIL_RATIO;

                        for tile_x in 1..TILE_SIZE {
                            for tile_y in 1..TILE_SIZE {
                                let radius = (((tile_x as i32 - TILE_SIZE as i32 / 2).pow(2)
                                    + (tile_y as i32 - TILE_SIZE as i32 / 2).pow(2))
                                    as f32)
                                    .sqrt();
                                let x_pixel = tile.get_x() as u32 * TILE_SIZE + tile_x;
                                let y_pixel = imgy - (tile.get_y() as u32 * TILE_SIZE + tile_y) - 1;

                                if radius > eye_radius {
                                    img.put_pixel(
                                        x_pixel as u32,
                                        y_pixel as u32,
                                        Rgb([r1, g1, b1]),
                                    );
                                } else if radius <= pupil_radius {
                                    img.put_pixel(x_pixel as u32, y_pixel as u32, Rgb([r2, g2, b2]))
                                }
                            }
                        }
                    } else {
                        for tile_x in 1..TILE_SIZE {
                            for tile_y in 1..TILE_SIZE {
                                let x_pixel = tile.get_x() as u32 * TILE_SIZE + tile_x;
                                let y_pixel = imgy - (tile.get_y() as u32 * TILE_SIZE + tile_y) - 1;
                                img.put_pixel(x_pixel as u32, y_pixel as u32, Rgb([r1, g1, b1]));
                            }
                        }
                    }
                }
            }
        }
        img.save(format!("{}{}.png", DRAW_PATH, file_name))
    }

    pub fn evaluate(&self) -> Vec<u64> {
        let mut result = Vec::with_capacity(self.max_snakes as usize);
        let mut current_snake = 0;

        for i in 0..self.max_snakes {
            let mut score: u64 = 0;
            if current_snake < self.snakes.len() && i == self.snakes[current_snake].get_id() {
                let snake = &self.snakes[current_snake];

                // digit 0
                let open_directions = self.open_directions(&snake) as u64;
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
                let weak_head = self.find_weaker_snake(&snake, LENGTH_ADVANTAGE);
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

                current_snake += 1;
            } else {
                score = 0;
            }

            result.push(score);
        }
        result
    }

    // Returns the closest food to pos
    pub fn find_closest_food(&self, pos: Coordinate) -> Option<Coordinate> {
        // If food exists
        if !self.food.is_empty() {
            let mut closest_food = self.food[0];
            let mut closest_distance = pos.distance_to(closest_food);
            // Iterate over food
            for i in 1..self.food.len() {
                let current_distance = pos.distance_to(self.food[i]);
                if current_distance < closest_distance {
                    closest_distance = current_distance;
                    closest_food = self.food[i];
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
                } else {
                    return None;
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

    // Returns true if pos is against the board walls
    pub fn is_against_wall(&self, pos: Coordinate) -> bool {
        pos.get_x() == 0
            || pos.get_x() == self.width - 1
            || pos.get_y() == 0
            || pos.get_y() == self.height - 1
    }

    // Moves self down and predicts future turns
    pub fn check_down(&mut self, current_level: i32, max_level: i32) -> Vec<u64> {
        let down = self.snakes[0].get_down();
        self.snakes[0].move_to(down);
        self.recursion_entry(current_level, max_level)
    }

    // Moves self up and predicts future turns
    pub fn check_up(&mut self, current_level: i32, max_level: i32) -> Vec<u64> {
        let up = self.snakes[0].get_up();
        self.snakes[0].move_to(up);
        self.recursion_entry(current_level, max_level)
    }

    // Moves self right and predicts future turns
    pub fn check_right(&mut self, current_level: i32, max_level: i32) -> Vec<u64> {
        let right = self.snakes[0].get_right();
        self.snakes[0].move_to(right);
        self.recursion_entry(current_level, max_level)
    }

    // Moves self left and predicts future turns
    pub fn check_left(&mut self, current_level: i32, max_level: i32) -> Vec<u64> {
        let left = self.snakes[0].get_left();
        self.snakes[0].move_to(left);
        self.recursion_entry(current_level, max_level)
    }

    // First level of recursion, my snake has already moved
    fn recursion_entry(&mut self, current_level: i32, max_level: i32) -> Vec<u64> {
        if DRAWING {
            self.draw(String::from("test")).unwrap();
        }

        // End case. Return if self is dead or current_level >= max_level
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
                let snake = &mut new_board.get_snakes_mut()[j + 1];
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Update new_board
            new_board.game_step();

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Get the maximin result from this position
            let result = new_board.minimax(current_level + 1, max_level);

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

    // Recursive minimax to find score of position
    pub fn minimax(self, current_level: i32, max_level: i32) -> Vec<u64> {
        if DRAWING {
            self.draw(String::from("test")).unwrap();
        }

        // End case. Return if self is dead or current_level >= max_level
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
                let snake = &mut new_board.get_snakes_mut()[j];
                let pos = snake.get_option((i / DIRECTIONS.pow(j as u32)) % DIRECTIONS);
                snake.move_to(pos);
            }

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Update new_board
            new_board.game_step();

            if DRAWING {
                new_board.draw(String::from("test")).unwrap();
            }

            // Get the maximin result from this position
            let result = new_board.minimax(current_level + 1, max_level);

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

    pub fn game_step(&mut self) {
        // Check all food
        let mut i = 0;
        while i < self.food.len() {
            let mut food_eaten = false;

            // Check all snakes
            for snake in &mut self.snakes {
                if snake.get_head() == self.food[i] {
                    food_eaten = true;
                    snake.eat_food();
                }
            }

            // Remove food if eaten
            if food_eaten {
                self.food.swap_remove(i);
            } else {
                i += 1;
            }
        }

        /*
        Any Battlesnake that has been eliminated is removed from the game board:
            Health less than or equal to 0
            Moved out of bounds
            Collided with themselves
            Collided with another Battlesnake
            Collided head-to-head and lost
        */

        // Eliminate snakes that are out of health or out of bounds
        let mut i = 0;
        while i < self.snakes.len() {
            let snake = &self.snakes[i];
            let x = snake.get_head().get_x();
            let y = snake.get_head().get_y();

            if snake.get_health() <= 0
                || (x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1)
            {
                self.snakes.remove(i);
            } else {
                i += 1;
            }
        }

        // Check for collisions
        let mut to_remove = Vec::with_capacity(self.snakes.len());
        for snake in &self.snakes {
            for other_snake in &self.snakes {
                if snake.lost_headon(other_snake) || snake.body_collision_with(other_snake) {
                    to_remove.push(snake.get_id());
                    break;
                }
            }
        }

        self.snakes
            .retain(|snake| !to_remove.contains(&snake.get_id()));

        self.turn += 1;
    }

    pub fn open_directions(&self, snake: &Battlesnake) -> i32 {
        let mut options = DIRECTIONS as i32 + 1;
        let pos = snake.get_head();

        for snake in &self.snakes {
            for tile in snake.get_body().range(..snake.get_length() - 1) {
                if *tile == pos.get_down()
                    || *tile == pos.get_up()
                    || *tile == pos.get_left()
                    || *tile == pos.get_right()
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

#[macro_export]
macro_rules! load_object {
    (Board, $filename:expr) => {{
        let file: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .open(format!(
                "{}{}.json",
                crate::constants::_TEST_PATH,
                $filename
            ))
            .unwrap();
        let board: crate::move_request::MoveRequest = serde_json::from_reader(file).unwrap();
        let board = board.into_values();
        let board = board.2.into_board(board.3, 0);
        board
    }};
    (Battlesnake, $filename:expr) => {{
        let file: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .open(format!(
                "{}{}.json",
                crate::constants::_TEST_PATH,
                $filename
            ))
            .unwrap();
        let snake: crate::input_snake::InputSnake = from_reader(file).unwrap();
        let snake = snake.into_battlesnake();
        snake
    }};
    ($type:ident, $filename:expr) => {{
        let file: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .open(format!(
                "{}{}.json",
                crate::constants::_TEST_PATH,
                $filename
            ))
            .unwrap();
        let object: $type = serde_json::from_reader(file).unwrap();
        object
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    // check_area
    #[test]
    fn test_check_area_closed() {
        let board = load_object!(Board, "check_area_closed-01");
        let pos = board.get_snakes()[0].get_head().get_left();

        let result = board.check_area(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_check_area_open() {
        let board = load_object!(Board, "check_area_open-01");
        let pos = board.get_snakes()[0].get_head().get_up();

        let result = board.check_area(pos, 0, 5, &mut Vec::with_capacity(5), 0);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_check_area_route() {
        let board = load_object!(Board, "check_area_route-01");
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.check_area(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 10);
    }

    // compare_to
    #[test]
    fn test_compare_to_advantage() {
        let better_board = load_object!(Board, "compare_to_advantage-01-better");
        let worse_board = load_object!(Board, "compare_to_advantage-01-worse");

        let true_result = better_board.evaluate();
        let false_result = worse_board.evaluate();

        assert_eq!(true_result[0] > false_result[0], true);
    }

    #[test]
    fn test_compare_to_alive() {
        let better_board = load_object!(Board, "better_than_alive-01-dead");
        let worse_board = load_object!(Board, "better_than_alive-01-alive");

        let true_result = better_board.evaluate();
        let false_result = worse_board.evaluate();

        assert_eq!(true_result[0] > false_result[0], true);
    }

    #[test]
    fn test_compare_to_dead() {
        let better_board = load_object!(Board, "better_than_dead-01-alive");
        let mut worse_board = load_object!(Board, "better_than_dead-01-dead");
        worse_board.max_snakes = 2;

        let true_result = better_board.evaluate();
        let false_result = worse_board.evaluate();

        assert_eq!(true_result[1] > false_result[1], true);
    }

    #[test]
    fn test_better_than_food() {
        let better_board = load_object!(Board, "better_than_food-01-close");
        let worse_board = load_object!(Board, "better_than_food-01-far");

        let true_result = better_board.evaluate();
        let false_result = worse_board.evaluate();

        assert_eq!(true_result[0] > false_result[0], true);
    }

    #[test]
    fn test_better_than_long() {
        let better_board = load_object!(Board, "better_than_long-01-long");
        let worse_board = load_object!(Board, "better_than_long-01-short");

        let true_result = better_board.evaluate();
        let false_result = worse_board.evaluate();

        assert_eq!(true_result[0] > false_result[0], true);
    }

    // draw()
    #[test]
    fn test_draw() {
        let board = load_object!(Board, "test_board-04");

        let result = board.draw(String::from("test_board-04"));

        assert_eq!(result.is_ok(), true);
    }

    // find_closest_food
    #[test]
    fn test_find_closest_food_none() {
        let board = load_object!(Board, "find_closest_food_none-01");

        let food = board.find_closest_food(board.get_snakes()[0].get_head());

        assert_eq!(food.is_none(), true);
    }

    #[test]
    fn test_find_closest_food_one() {
        let board = load_object!(Board, "find_closest_food_one-01");

        let food = board.find_closest_food(board.get_snakes()[0].get_head());

        assert_eq!(food.unwrap(), Coordinate::new(5, 5));
    }

    #[test]
    fn test_find_closest_food_two() {
        let board = load_object!(Board, "find_closest_food_two-01");

        let food = board.find_closest_food(board.get_snakes()[0].get_head());

        assert_eq!(food.unwrap(), Coordinate::new(0, 3));
    }

    // find_weaker_snake()
    #[test]
    fn test_find_weaker_snake_none() {
        let board = load_object!(Board, "find_weaker_snake_none-01");
        let snake = &board.get_snakes()[0];

        let snake_head = board.find_weaker_snake(snake, 5);

        assert_eq!(snake_head.is_none(), true);
    }

    #[test]
    fn test_find_weaker_snake_one() {
        let board = load_object!(Board, "find_weaker_snake_one-01");
        let snake = &board.snakes[0];

        let snake_head = board.find_weaker_snake(snake, 5);

        assert_eq!(snake_head.unwrap(), board.snakes[1].get_head());
    }

    // game_step()
    #[test]
    fn test_body_collision() {
        let mut before_collision = load_object!(Board, "body_collision-01-before");
        let mut after_collision = load_object!(Board, "body_collision-01-after");
        after_collision.max_snakes = 2;

        before_collision.game_step();
        after_collision.turn += 1;

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_double_headon_collision() {
        let mut before_collision = load_object!(Board, "double_headon_collision-01-before");
        let mut after_collision = load_object!(Board, "double_headon_collision-01-after");
        after_collision.max_snakes = 3;

        before_collision.game_step();
        after_collision.turn += 1;

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_eat_food() {
        let mut before_eat = load_object!(Board, "eat-01-before");
        let mut after_eat = load_object!(Board, "eat-01-after");

        before_eat.game_step();
        after_eat.turn += 1;

        assert_eq!(before_eat, after_eat);
    }

    #[test]
    fn test_headon_collision() {
        let mut before_collision = load_object!(Board, "headon_collision-01-before");
        let mut after_collision = load_object!(Board, "headon_collision-01-after");
        after_collision.max_snakes = 2;

        before_collision.game_step();
        after_collision.turn += 1;

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut before = load_object!(Board, "out_of_bounds-01-before");
        let mut after = load_object!(Board, "out_of_bounds-01-after");
        after.max_snakes = 2;

        before.game_step();
        after.turn += 1;

        assert_eq!(before, after);
    }

    #[test]
    fn test_out_of_health() {
        let mut before = load_object!(Board, "out_of_health-01-before");
        let mut after = load_object!(Board, "out_of_health-01-after");
        after.max_snakes = 2;

        before.game_step();
        after.turn += 1;

        assert_eq!(before, after);
    }

    #[test]
    fn test_simple() {
        let mut before = load_object!(Board, "simple-02");
        let mut after = load_object!(Board, "simple-02");

        before.game_step();
        after.turn += 1;

        assert_eq!(before, after);
    }

    // get_snake()
    #[test]
    fn test_get_snake_none() {
        let board = load_object!(Board, "get_snake_none-01");

        let snake = board.get_snake(1);

        assert_eq!(snake.is_none(), true);
    }

    #[test]
    fn test_get_snake_one() {
        let board = load_object!(Board, "get_snake_one-01");

        let snake = board.get_snake(0);

        assert_eq!(snake.unwrap(), &board.get_snakes()[0]);
    }

    // is_against_wall()
    #[test]
    fn test_is_against_wall_false() {
        let board = load_object!(Board, "against_wall_false-01");

        let result = board.is_against_wall(Coordinate::new(3, 2));

        assert_eq!(result, false);
    }

    #[test]
    fn test_is_against_wall_true() {
        let board = load_object!(Board, "against_wall_true-01");

        let result = board.is_against_wall(Coordinate::new(0, 3));

        assert_eq!(result, true);
    }

    // minimax()
    #[test]
    fn test_minimax() {
        let board = load_object!(Board, "test_board-03");

        let result = board.minimax(0, 6);

        assert_eq!(result[0] > 0, true);
    }

    // open_directions()
    #[test]
    fn test_open_directions_2() {
        let board = load_object!(Board, "open_directions_2-01");
        let snake = &board.get_snakes()[0];

        assert_eq!(board.open_directions(snake), 2);
    }

    #[test]
    fn test_open_directions_3() {
        let board = load_object!(Board, "open_directions_3-01");
        let snake = &board.get_snakes()[0];

        assert_eq!(board.open_directions(snake), 3);
    }
}
