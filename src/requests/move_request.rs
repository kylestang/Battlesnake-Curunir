use serde::{Deserialize, Serialize};

use crate::input_board::InputBoard;
use crate::input_game::InputGame;
use crate::input_snake::InputSnake;

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveRequest {
    game: InputGame,
    turn: i32,
    board: InputBoard,
    you: InputSnake,
}

impl MoveRequest {
    pub fn _new(game: InputGame, turn: i32, board: InputBoard, you: InputSnake) -> MoveRequest {
        MoveRequest {
            game,
            turn,
            board,
            you,
        }
    }

    // Break up self and return it's values
    pub fn into_values(self) -> (InputGame, i32, InputBoard, InputSnake) {
        (self.game, self.turn, self.board, self.you)
    }
}
