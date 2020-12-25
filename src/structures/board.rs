use image::{Rgb, RgbImage};
use std::cmp::{max, min};
use std::convert::TryInto;
use std::time::{Duration, Instant};

use crate::battlesnake::Battlesnake;
use crate::constants::{DIRECTIONS, DRAWING, DRAW_PATH, EYE_RATIO, FOOD_RATIO, PUPIL_RATIO, TILE_SIZE, YOU_ID};
use crate::coordinate::Coordinate;
use crate::route::{DEFAULT_ROUTE, MAX_ROUTE, MIN_ROUTE, Route};

#[derive(Clone, Debug)]
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

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
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

    pub fn draw(&self, file_name: String) {

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
        img.save(format!("{}{}.png", DRAW_PATH, file_name)).unwrap();
    }

    pub fn test_down(&self, end_time: Instant) -> Route {
        let mut new_board = self.clone();
        new_board.snakes[0].move_to(self.snakes[0].get_down());
        new_board.minimax(DEFAULT_ROUTE.clone(), MIN_ROUTE, MAX_ROUTE, false, end_time)
    }

    pub fn test_up(&self, end_time: Instant) -> Route {
        let mut new_board = self.clone();
        new_board.snakes[0].move_to(self.snakes[0].get_up());
        new_board.minimax(DEFAULT_ROUTE.clone(), MIN_ROUTE, MAX_ROUTE, false, end_time)
    }

    pub fn test_right(&self, end_time: Instant) -> Route {
        let mut new_board = self.clone();
        new_board.snakes[0].move_to(self.snakes[0].get_right());
        new_board.minimax(DEFAULT_ROUTE.clone(), MIN_ROUTE, MAX_ROUTE, false, end_time)
    }

    pub fn test_left(&self, end_time: Instant) -> Route {
        let mut new_board = self.clone();
        new_board.snakes[0].move_to(self.snakes[0].get_left());
        new_board.minimax(DEFAULT_ROUTE.clone(), MIN_ROUTE, MAX_ROUTE, false, end_time)
    }

    // Recursive minimax to find score of position
    fn minimax(&mut self, current_route: Route, mut alpha: Route, mut beta: Route, my_turn: bool, end_time: Instant) -> Route {
        
        if DRAWING {
            self.draw(String::from("test"));
        }
        
        // If I'm dead or time has run out, return current level
        if !current_route.get_survival() || Instant::now() > end_time {
            return current_route;
        }

        // My turn
        if my_turn {
            let you = &self.snakes[0];
            let mut best_route = MIN_ROUTE;
            // Try each direction
            for pos in &you.get_head().get_adjacent() {
                let mut new_board = self.clone();
                new_board.get_snakes_mut()[0].move_to(*pos);
                // Let other snakes move
                let new_route = new_board.minimax(current_route, alpha, beta, false, end_time);
                best_route = max(best_route, new_route);
                alpha = max(alpha, best_route);
                if alpha >= beta {
                    break;
                }
            }
            // Return best route found
            return best_route;
        }
    
        // Other snakes
        else {

            // Get number of snakes
            let num_snakes = self.snakes.len() as u32;
            let mut worst_route = MAX_ROUTE;
            // Iterate through all possible combinations of snake movements
            let possibilities = DIRECTIONS.pow(num_snakes - 1);
            for count in 0..possibilities {
                let mut new_board = self.clone();
                // Move each snake
                for i in 0..num_snakes as usize - 1 {
                    let snake = &mut new_board.get_snakes_mut()[i + 1];
                    let adjacent = snake.get_head().get_adjacent();
        
                    snake.move_to(adjacent[(count / DIRECTIONS.pow(i as u32)) % DIRECTIONS]);
                }

                if DRAWING {
                    new_board.draw(String::from("test"));
                }

                // Update board
                let new_route = new_board.game_step(current_route.clone());

                if DRAWING {
                    new_board.draw(String::from("test"));
                }

                // Let me move
                let alloted_time = end_time.saturating_duration_since(Instant::now()).as_nanos() / (possibilities - count) as u128;
                let duration = Duration::from_nanos(alloted_time as u64);
                let turns = new_board.minimax(new_route, alpha, beta, true, Instant::now() + duration);
                worst_route = min(worst_route, turns);
                beta = min(beta, worst_route);
                if beta <= alpha {
                    break;
                }
            }
            // Return minimum number of turns survived
            return worst_route;
        }
    }

    pub fn game_step(&mut self, mut current_route: Route) -> Route {
        // Any Battlesnake that has found food will consume it
        for snake in &mut self.snakes {
            let mut i = 0;
            while i < self.food.len() {
                if self.food[i] == snake.get_head() {
                    snake.eat_food();
                    if snake.get_id() == YOU_ID {
                        current_route.increment_my_food();
                    } else {
                        current_route.increment_opponent_food();
                    }
                    self.food.remove(i);
                    continue;
                }
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
        let mut i = 0;
        'snake_loop: while i < self.snakes.len() {
            if self.snakes[i].get_health() <= 0 {
                self.snakes.remove(i);
                current_route.increment_snakes_killed();
                continue;
            }

            if self.snakes[i].did_collide(self){
                self.snakes.remove(i);
                current_route.increment_snakes_killed();
                continue;
            }

            let snakes = &mut self.snakes;

            let mut j = i + 1;
            while j < snakes.len() {
                if snakes[i].get_head() == snakes[j].get_head() {
                    if snakes[i].get_length() < snakes[j].get_length() {
                        snakes.remove(i);
                        current_route.increment_snakes_killed();
                        continue 'snake_loop;
                    } else if snakes[i].get_length() > snakes[j].get_length() {
                        snakes.remove(j);
                        current_route.increment_snakes_killed();
                        continue;
                    } else {
                        snakes.remove(j);
                        current_route.increment_snakes_killed();
                        snakes.remove(i);
                        current_route.increment_snakes_killed();
                        continue 'snake_loop;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        // Update current route
        current_route.increment_turns();
        if self.snakes.len() == 0 || self.snakes[0].get_id() != YOU_ID {
            current_route.set_survival(false);
        } else if self.snakes.len() == 1 {
            current_route.set_solo(true);
        }
        return current_route;
    }
}

#[cfg(test)]
mod test_draw {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_basic(){
        let board = Board::new(
            11,
            11,
            vec![
                Coordinate::new(5, 5),
                Coordinate::new(9, 0),
                Coordinate::new(2, 6),
                Coordinate::new(5, 4)
            ],
            vec![
                Coordinate::new(0, 0)
            ],
            vec![
                Battlesnake::new(
                    0,
                    54,
                    VecDeque::from(vec![
                        Coordinate::new(0, 0),
                        Coordinate::new(1, 0),
                        Coordinate::new(2, 0)
                    ]),
                    111,
                    Coordinate::new(0, 0),
                    3
                ),
                Battlesnake::new(
                    1,
                    16,
                    VecDeque::from(vec![
                        Coordinate::new(5, 4),
                        Coordinate::new(5, 3),
                        Coordinate::new(6, 3),
                        Coordinate::new(6, 2)
                    ]),
                    222,
                    Coordinate::new(5, 4),
                    4,
                )
            ]
        );

        board.draw(String::from("hello"));
        
        assert!(true);
    }
}
