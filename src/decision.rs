use crate::constants::SEARCH_DEPTH;
use crate::requests::MoveResponse;
use crate::structures::{battlesnake, board, game};
use battlesnake::Battlesnake;
use board::Board;
use game::Game;

pub fn decision(game: &Game, turn: i32, board: Board, you: Battlesnake) -> MoveResponse {

    let down = board.test_down(you.get_id(), SEARCH_DEPTH);
    let up = board.test_up(you.get_id(), SEARCH_DEPTH);
    let right = board.test_right(you.get_id(), SEARCH_DEPTH);
    let left = board.test_left(you.get_id(), SEARCH_DEPTH);

    let mut direction = String::from("up");

    if down {
        direction = String::from("down");
    }
    else if up {
        direction = String::from("up");
    }
    else if right {
        direction = String::from("right");
    }
    else if left {
        direction = String::from("left");
    }

    game.log_data(format!("turn: {} direction: {}", turn, direction));

    MoveResponse::new(direction, String::from("Hi!"))
}
