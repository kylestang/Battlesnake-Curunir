use crate::board::Board;
use crate::constants::{DRAW_PATH, EYE_RATIO, FOOD_RATIO, PUPIL_RATIO, TILE_SIZE};

use image::{ImageResult, Rgb, RgbImage};

impl Board {
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
}

#[cfg(test)]
mod tests {
    use crate::constants::_TEST_PATH;
    use crate::load_object;
    // draw()
    #[test]
    fn test_draw() {
        let filename = "check_area_closed-03";
        let board = load_object!(Board, filename, _TEST_PATH);

        let result = board.draw(String::from(filename));

        assert!(result.is_ok());
    }
}
