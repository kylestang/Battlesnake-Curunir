use serde::{Deserialize, Serialize};

use crate::board::Board;
use crate::constants::YOU_ID;
use crate::coordinate::Coordinate;
use crate::input_snake::InputSnake;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputBoard {
    height: i32,
    width: i32,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<InputSnake>
}

impl InputBoard {

    // Convert self to a Board struct
    pub fn into_board(mut self, you: InputSnake) -> Board {
        let num_snakes = self.snakes.len();
        // Create a vector to store the snakes
        let mut snakes = Vec::with_capacity(num_snakes);
        // Push my snake as a Battlesnake onto the snakes vector
        let you_id = you.get_id().clone();
        snakes.push(you.into_battlesnake(YOU_ID));
        // Push all the other snakes as Battlesnakes onto the snakes vector
        for i in 0..num_snakes {
            let snake = self.snakes.pop().unwrap();
            if snake.get_id() != &you_id {
                snakes.push(snake.into_battlesnake(i as i32 + 1));
            }
        }

        // Return the Board object
        Board::new(
            self.height,
            self.width,
            self.food,
            self.hazards,
            snakes
        )
    }
}