use crate::structures::{Coordinate, Board, Battlesnake};
use image::{RgbImage, Rgb};
use crate::constants::{DRAW_PATH, TILE_SIZE};
use std::convert::TryInto;
use std::collections::VecDeque;

pub fn draw_board(board: &mut Board, name: String) {

    let imgx = TILE_SIZE * board.get_width() as u32;
    let imgy = TILE_SIZE * board.get_height() as u32;

    let mut img = RgbImage::new(imgx, imgy);

    for x in 0..board.get_width() as u32 {
        for y in 0..board.get_height() as u32 {

            let b: u8 = 255;
            let g: u8 = 255;
            let r: u8 = 255;

            for tile_x in 0..TILE_SIZE {
                for tile_y in 0..TILE_SIZE {
                    let x_pixel= x * TILE_SIZE + tile_x;
                    let y_pixel = imgy - (y * TILE_SIZE + tile_y) - 1;
                    img.put_pixel(x_pixel, y_pixel, Rgb([r, g, b]));
                }
            }

        }
    }

    let width = board.get_width();
    let height = board.get_height();

    for snake in board.get_snakes() {
        draw_snake(&mut img, snake, width, height, imgy as i32);
    }

    for tile in board.get_food() {
        draw_food(&mut img, tile, imgy as i32);
    }

    img.save(format!("{}{}.png", DRAW_PATH, name)).unwrap();
    
}

fn draw_snake(img: &mut RgbImage, snake: &Battlesnake, width: i32, height: i32, imgy: i32) {

    let r: u8 = (((snake.get_id() * 90) % 255) as u8).try_into().unwrap();
    let g: u8 = (((snake.get_id() * 150) % 255) as u8).try_into().unwrap();
    let b: u8 = (((snake.get_id() * 220) % 255) as u8).try_into().unwrap();

    for tile in snake.get_body() {
        if !(tile.get_x() < 0 || tile.get_x() > width - 1
        || tile.get_y() < 0 || tile.get_y() > height - 1) {
            for tile_x in 0..TILE_SIZE as i32 {
                for tile_y in 0..TILE_SIZE as i32 {
                    let x_pixel = tile.get_x() * TILE_SIZE as i32 + tile_x;
                    let y_pixel = imgy - (tile.get_y() * TILE_SIZE as i32 + tile_y) - 1;
                    img.put_pixel(x_pixel as u32, y_pixel as u32, Rgb([r, g, b]))
                }
            }
        }
    }

}

fn draw_food(img: &mut RgbImage, tile: &Coordinate, imgy: i32) {
    let food_radius = TILE_SIZE as f32 / 2.5;

    for tile_x in 0..TILE_SIZE as i32 {
        for tile_y in 0..TILE_SIZE as i32 {
            let radius = (((tile_x - TILE_SIZE as i32 / 2).pow(2) + (tile_y - TILE_SIZE as i32 / 2).pow(2)) as f32).sqrt();
            let x_pixel = tile.get_x() * TILE_SIZE as i32 + tile_x;
            let y_pixel = imgy - (tile.get_y() * TILE_SIZE as i32 + tile_y) - 1;
            if radius <= food_radius {
                img.put_pixel(x_pixel as u32, y_pixel as u32, Rgb([255, 0, 0]));
            }
        }
    }
}

#[cfg(test)]
mod test_draw {
    use super::*;

    #[test]
    fn test_basic(){
        let mut board = Board::new(
            11,
            11,
            vec![
                Coordinate::new(5, 5),
                Coordinate::new(9, 0),
                Coordinate::new(2, 6)
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

        draw_board(&mut board, String::from("hi"));
        
        assert!(true);
    }
}
