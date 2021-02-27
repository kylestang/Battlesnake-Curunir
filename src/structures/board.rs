use image::{ImageResult, Rgb, RgbImage};
use std::convert::TryInto;
use std::time::{Duration, Instant};

use crate::battlesnake::Battlesnake;
use crate::constants::{DIRECTIONS, DRAWING, DRAW_PATH, EYE_RATIO, FOOD_RATIO, PUPIL_RATIO, TILE_SIZE, YOU_ID};
use crate::coordinate::Coordinate;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>
}

impl Board {
    pub fn new(height: i32, width: i32, food: Vec<Coordinate>, hazards: Vec<Coordinate>, snakes: Vec<Battlesnake>) -> Board {
        Board {height, width, food, hazards, snakes}
    }

    pub fn _get_height(&self) -> i32 {
        self.height
    }

    pub fn _get_width(&self) -> i32 {
        self.width
    }

    pub fn _get_food(&mut self) -> &mut Vec<Coordinate> {
        &mut self.food
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

    // Return true if self is better than other
    fn better_than(&self, other: &Board, snake_id: i32) -> bool {
        let self_snake = self.get_snake(snake_id);
        let other_snake = other.get_snake(snake_id);

        // Board where self is alive is better
        if self_snake.is_some() && other_snake.is_none() {
            return true;
        } else if self_snake.is_none() && other_snake.is_some() {
            return false;
        }

        // Board where enemy snakes are dead is better
        let self_snakes = self.snakes.len();
        let other_snakes = other.snakes.len();
        if self_snakes < other_snakes {
            return true;
        } else if self_snakes > other_snakes {
            return false;
        }

        // If both snakes are alive
        if self_snake.is_some() && other_snake.is_some() {
            let self_snake = self_snake.unwrap();
            let other_snake = other_snake.unwrap();
            
            // Board where self is longer is better
            let self_length = self_snake.get_length();
            let other_length = other_snake.get_length();
            if self_length > other_length {
                return true;
            } else if self_length < other_length {
                return false;
            }

            // Board where self is closer to food is better
            let self_closest_food = self.find_closest_food(self_snake.get_head());
            let other_closest_food = other.find_closest_food(other_snake.get_head());
            if self_closest_food < other_closest_food {
                return true;
            } else if self_closest_food > other_closest_food {
                return false;
            }
        }

        // Default return true
        true
    }

    pub fn draw(&self, file_name: String) -> ImageResult<()> {

        let imgx = TILE_SIZE * self.width as u32;
        let imgy = TILE_SIZE * self.height as u32;

        let mut img = RgbImage::new(imgx, imgy);

        for x in 0..self.width as u32{
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
                        }
                        else {
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

                    let radius = (((tile_x as i32 - TILE_SIZE as i32 / 2).pow(2) + (tile_y as i32 - TILE_SIZE as i32 / 2).pow(2)) as f32).sqrt();
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

            let r1: u8 = (((snake.get_id() * 90) % 255) as u8).try_into().unwrap();
            let g1: u8 = (((snake.get_id() * 150) % 255) as u8).try_into().unwrap();
            let b1: u8 = (((snake.get_id() * 210) % 255) as u8).try_into().unwrap();

            let r2: u8 = ((((snake.get_id() + 100) * 90) % 255) as u8).try_into().unwrap();
            let g2: u8 = ((((snake.get_id() + 176) * 150) % 255) as u8).try_into().unwrap();
            let b2: u8 = ((((snake.get_id() + 095) * 210) % 255) as u8).try_into().unwrap();

            for tile in snake.get_body() {
                if !(tile.get_x() < 0 || tile.get_x() > self.width - 1 || tile.get_y() < 0 || tile.get_y() > self.height - 1) {
                    if *tile == snake.get_head() {
                        let eye_radius = TILE_SIZE as f32 / EYE_RATIO;
                        let pupil_radius = TILE_SIZE as f32 / PUPIL_RATIO;

                        for tile_x in 1..TILE_SIZE {
                            for tile_y in 1..TILE_SIZE {

                                let radius = (((tile_x as i32 - TILE_SIZE as i32 / 2).pow(2) + (tile_y as i32 - TILE_SIZE as i32 / 2).pow(2)) as f32).sqrt();
                                let x_pixel = tile.get_x() as u32 * TILE_SIZE + tile_x;
                                let y_pixel = imgy - (tile.get_y() as u32 * TILE_SIZE + tile_y) - 1;
                                
                                if radius > eye_radius {
                                    img.put_pixel(x_pixel as u32, y_pixel as u32, Rgb([r1, g1, b1]));
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

    // Returns the distance to the closest food to pos
    fn find_closest_food(&self, pos: Coordinate) -> Option<i32> {
        if self.food.len() > 0 {
            let mut closest_distance = (self.food[0].get_x() - pos.get_x()).abs() + (self.food[0].get_y() - pos.get_y()).abs();
            for i in 1..self.food.len() {
                let current_distance = (self.food[i].get_x() - pos.get_x()).abs() + (self.food[i].get_y() - pos.get_y()).abs();
                if current_distance < closest_distance {
                    closest_distance = current_distance;
                }
            }
            return Some(closest_distance);
        } else {
            None
        }
    }

    // Returns the snake with id snake_id, or None
    pub fn get_snake(&self, snake_id: i32) -> Option<&Battlesnake> {
        for snake in &self.snakes {
            if snake.get_id() == snake_id {
                return Some(snake);
            }
        }
        None
    }

    // Recursive minimax to find score of position
    pub fn minimax(&mut self, current_level: i32, max_level: i32) -> Board {
        if DRAWING {
            self.draw(String::from("test")).unwrap();
        }

        // End case. Return is self is dead or current_level >= max_level
        if current_level >= max_level || self.snakes.len() <= 0 || self.snakes[0].get_id() != YOU_ID {
            return self.clone();
        }

        let num_snakes = self.snakes.len();
        let mut worst_outcomes: Vec<[i32; 4]> = vec![[-1; DIRECTIONS]; num_snakes];
        let mut outcomes = Vec::with_capacity(DIRECTIONS.pow(num_snakes as u32));
        let mut best_worst_outcomes = vec![0; num_snakes];

        // Iterate through all possible boards
        for i in 0..DIRECTIONS.pow(num_snakes as u32) {
            // Create new Board to modify
            let mut new_board = self.clone();
            // Move each snake to new position on new Board
            for j in 0..num_snakes {
                let snake = &mut new_board.get_snakes_mut()[j];
                let pos = snake.get_head().get_adjacent()[(i / DIRECTIONS.pow(j as u32)) % 4];
                snake.move_to(pos);
            }

            new_board.game_step();

            // Get the maximin result from this position
            let current_board = new_board.minimax(current_level + 1, max_level);
            

            // Update worst outcomes
            for j in 0..num_snakes {
                let direction = (i / DIRECTIONS.pow(j as u32)) % 4;
                let best_board = worst_outcomes[j][direction];
                if best_board < 0 || !current_board.better_than(&outcomes[best_board as usize], self.snakes[j].get_id()) {
                    worst_outcomes[j][direction] = i as i32;
                }
            }

            outcomes.push(current_board);
        }

        for i in 0..num_snakes {
            for j in 1..DIRECTIONS {
                let best_direction = worst_outcomes[i][best_worst_outcomes[i]] as usize;
                let current_best = &outcomes[best_direction];

                let test_direction = worst_outcomes[i][j] as usize;
                let current_test = &outcomes[test_direction];

                if current_test.better_than(current_best, self.snakes[i].get_id()) {
                    best_worst_outcomes[i] = j;
                }
            }
        }

        let mut return_board = 0;
        for i in 0..num_snakes {
            return_board += best_worst_outcomes[i] * DIRECTIONS.pow(i as u32);
        }

        outcomes.swap_remove(return_board)
    }

    fn game_step(&mut self) {
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
                self.food.remove(i);
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

            if snake.get_health() <= 0 || (x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1) {
                self.snakes.remove(i);
            } else {
                i += 1;
            }
        }

        // TODO break out of outer loop if snake has been eliminated
        // Check for collisions
        let mut to_remove = Vec::with_capacity(self.snakes.len());
        for snake in &self.snakes {
            for other_snake in &self.snakes {
                if snake.lost_headon(other_snake) || snake.body_collision_with(other_snake) {
                    to_remove.push(snake.get_id());
                }
            }
        }

        // Eliminate collided snakes
        for snake_id in to_remove {
            let mut i = 0;
            while i < self.snakes.len() {
                if self.snakes[i].get_id() == snake_id {
                    self.snakes.remove(i);
                    break;
                } else {
                    i += 1;
                }
            }
        }
    }
}

#[macro_export]
macro_rules! load_object {
    (Board, $filename:expr) => {
        {
            let file: std::fs::File = std::fs::OpenOptions::new()
                .read(true).open(format!("{}{}.json", crate::constants::TEST_PATH, $filename)).unwrap();
            let board: crate::move_request::MoveRequest = serde_json::from_reader(file).unwrap();
            let board = board.into_values();
            let board = board.2.into_board(board.3);
            board
        }
    };
    (Battlesnake, $filename:expr) => {
        {
            let file: std::fs::File =std::fs::OpenOptions::new()
                .read(true).open(format!("{}{}.json", crate::constants::TEST_PATH, $filename)).unwrap();
            let snake: crate::input_snake::InputSnake = from_reader(file).unwrap();
            let snake = snake.into_battlesnake();
            snake
        }
    };
    ($type:ident, $filename:expr) => {
        {
            let file: std::fs::File = std::fs::OpenOptions::new()
                .read(true).open(format!("{}{}.json", crate::constants::TEST_PATH, $filename)).unwrap();
            let object: $type = serde_json::from_reader(file).unwrap();
            object
        }
    };
}

#[cfg(test)]
mod tests {
    // better_than()
    #[test]
    fn test_better_than_alive() {
        let better_board = load_object!(Board, "better_than_alive-01-dead");
        let worse_board = load_object!(Board, "better_than_alive-01-alive");

        let true_result = better_board.better_than(&worse_board, 0);
        let false_result = worse_board.better_than(&better_board, 0);
        
        assert_eq!(true_result && !false_result, true);
    }

    #[test]
    fn test_better_than_dead() {
        let better_board = load_object!(Board, "better_than_dead-01-alive");
        let worse_board = load_object!(Board, "better_than_dead-01-dead");

        let true_result = better_board.better_than(&worse_board, 1);
        let false_result = worse_board.better_than(&better_board, 1);
        
        assert_eq!(true_result && !false_result, true);
    }

    #[test]
    fn test_better_than_food() {
        let better_board = load_object!(Board, "better_than_food-01-close");
        let worse_board = load_object!(Board, "better_than_food-01-far");

        let true_result = better_board.better_than(&worse_board, 0);
        let false_result = worse_board.better_than(&better_board, 0);

        assert_eq!(true_result && !false_result, true);
    }

    #[test]
    fn test_better_than_long() {
        let better_board = load_object!(Board, "better_than_long-01-long");
        let worse_board = load_object!(Board, "better_than_long-01-short");

        let true_result = better_board.better_than(&worse_board, 0);
        let false_result = worse_board.better_than(&better_board, 0);

        assert_eq!(true_result && !false_result, true);
    }
    
    // draw()
    #[test]
    fn test_draw() {
        let board = load_object!(Board, "test_board-02");

        let result = board.draw(String::from("test_board-02"));
        
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

        assert_eq!(food.unwrap(), 4);
    }

    #[test]
    fn test_find_closest_food_two() {
        let board = load_object!(Board, "find_closest_food_two-01");

        let food = board.find_closest_food(board.get_snakes()[0].get_head());

        assert_eq!(food.unwrap(), 3);
    }

    // game_step()
    #[test]
    fn test_body_collision() {
        let mut before_collision = load_object!(Board, "body_collision-01-before");
        let after_collision = load_object!(Board, "body_collision-01-after");

        before_collision.game_step();

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_double_headon_collision() {
        let mut before_collision = load_object!(Board, "double_headon_collision-01-before");
        let after_collision = load_object!(Board, "double_headon_collision-01-after");

        before_collision.game_step();

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_eat_food() {
        let mut before_eat = load_object!(Board, "eat-01-before");
        let after_eat = load_object!(Board, "eat-01-after");

        before_eat.game_step();

        assert_eq!(before_eat, after_eat);
    }

    #[test]
    fn test_headon_collision() {
        let mut before_collision = load_object!(Board, "headon_collision-01-before");
        let after_collision = load_object!(Board, "headon_collision-01-after");

        before_collision.game_step();

        assert_eq!(before_collision, after_collision);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut before = load_object!(Board, "out_of_bounds-01-before");
        let after = load_object!(Board, "out_of_bounds-01-after");

        before.game_step();

        assert_eq!(before, after);
    }

    #[test]
    fn test_out_of_health() {
        let mut before = load_object!(Board, "out_of_health-01-before");
        let after = load_object!(Board, "out_of_health-01-after");

        before.game_step();

        assert_eq!(before, after);
    }

    #[test]
    fn test_simple() {
        let mut before = load_object!(Board, "simple-02");
        let after = load_object!(Board, "simple-02");

        before.game_step();

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

    // minimax()
    #[test]
    fn test_minimax() {
        let mut board = load_object!(Board, "test_board-01");

        let result = board.minimax(0, 6);
        result.draw(String::from("test")).unwrap();

        assert_eq!(true, true);
    }
}
