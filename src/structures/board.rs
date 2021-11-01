pub mod area_controlled;
pub mod evaluate;
pub mod game_step;
pub mod simulate;

use image::{ImageResult, Rgb, RgbImage};

use crate::battlesnake::Battlesnake;
use crate::constants::{
    DIRECTIONS, DRAW_PATH, EYE_RATIO, FOOD_RATIO, PUPIL_RATIO, TILE_SIZE, YOU_ID,
};
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

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
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
                        if snake.get_length() - i >= current_area as usize - food_eaten {
                            return current_area;
                        } else {
                            return max_area;
                        }
                    } else if snake.get_length() - i >= current_area as usize {
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
    // check_area
    #[test]
    fn test_check_area_closed() {
        let board = load_object!(Board, "check_area_closed-01", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_left();

        let result = board.check_area(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_check_area_open() {
        let board = load_object!(Board, "check_area_open-01", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_up();

        let result = board.check_area(pos, 0, 30, &mut Vec::with_capacity(30), 0);

        assert_eq!(result, 30);
    }

    // TODO get new test case
    #[test]
    fn test_check_area_route() {
        let board = load_object!(Board, "check_area_route-01", _TEST_PATH);
        let pos = board.get_snakes()[0].get_head().get_down();

        let result = board.check_area(pos, 0, 10, &mut Vec::with_capacity(10), 0);

        assert_eq!(result, 10);
    }

    // draw()
    #[test]
    fn test_draw() {
        let board = load_object!(Board, "empty_board-11x11", _TEST_PATH);

        let result = board.draw(String::from("empty_board-11x11"));

        assert!(result.is_ok());
    }

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
