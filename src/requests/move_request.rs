use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::input_board::InputBoard;
use crate::input_snake::InputSnake;

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveRequest {
    game: Game,
    turn: i32,
    board: InputBoard,
    you: InputSnake,
}

impl MoveRequest {
    pub fn _new(game: Game, turn: i32, board: InputBoard, you: InputSnake) -> MoveRequest {
        MoveRequest {
            game,
            turn,
            board,
            you,
        }
    }

    // Break up self and return it's values
    pub fn into_values(self) -> (Game, i32, InputBoard, InputSnake) {
        (self.game, self.turn, self.board, self.you)
    }
}
