use image::{Rgb, RgbImage};
use std::convert::TryInto;

use crate::battlesnake::Battlesnake;
use crate::coordinate::Coordinate;

const EYE_RATIO: f32 = 5.0;
const FOOD_RATIO: f32 = 2.5;
const PUPIL_RATIO: f32 = 10.0;
const TILE_SIZE: u32 = 50;

const DRAW_PATH: &'static str = "drawings/";


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

    pub fn get_food(&mut self) -> &mut Vec<Coordinate> {
        &mut self.food
    }

    pub fn _get_hazards(&self) -> &Vec<Coordinate> {
        &self.hazards
    }

    pub fn get_snakes(&mut self) -> &mut Vec<Battlesnake> {
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

                        let mut r: u8;
                        let mut g: u8;
                        let mut b: u8;

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
        }
        img.save(format!("{}{}.png", DRAW_PATH, file_name)).unwrap();
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
