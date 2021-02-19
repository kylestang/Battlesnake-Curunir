use crate::constants::YOU_ID;
use crate::move_response::MoveResponse;
use crate::structures::{board, game};
use board::Board;
use game::Game;
use std::cmp::max;
use std::sync::mpsc;
use std::thread::spawn;
use std::time::{Duration, Instant};

pub fn decision(game: Game, turn: i32, board: Board) -> MoveResponse {
    let max_level = 4;
    /*
    // Create a thread for down
    let down_board = board.clone();
    let (down_tx, down_rx) = mpsc::channel();
    let down_handle = spawn(move || {
        let down = down_board.test_down(end_time);
        down_tx.send(down).unwrap();
    });

    // Create a thread for up
    let up_board = board.clone();
    let (up_tx, up_rx) = mpsc::channel();
    let up_handle = spawn(move || {
        let up = up_board.test_up(end_time);
        up_tx.send(up).unwrap();
    });

    // Create a thread for right
    let right_board = board.clone();
    let (right_tx, right_rx) = mpsc::channel();
    let right_handle = spawn(move || {
        let right = right_board.test_right(end_time);
        right_tx.send(right).unwrap();
    });

    // Create a thread for left
    let left_board = board.clone();
    let (left_tx, left_rx) = mpsc::channel();
    let left_handle = spawn(move || {
        let left = left_board.test_left(end_time);
        left_tx.send(left).unwrap();
    });

    // Finish down thread
    let down = down_rx.recv().unwrap();
    down_handle.join().unwrap();

    // Finish up thread
    let up = up_rx.recv().unwrap();
    up_handle.join().unwrap();

    // Finish right thread
    let right = right_rx.recv().unwrap();
    right_handle.join().unwrap();

    // Finish left thread
    let left = left_rx.recv().unwrap();
    left_handle.join().unwrap();

    let mut direction = String::from("up");

    let max_turns = max(max(down, up), max(right, left));

    if down == max_turns {
        direction = String::from("down");
    }
    else if up == max_turns {
        direction = String::from("up");
    }
    else if right == max_turns {
        direction = String::from("right");
    }
    else if left == max_turns {
        direction = String::from("left");
    }
    

    game.log_data(format!(
    "       turn: {}\n  direction: {}\n down turns: {:?}\n   up turns: {:?}\nright turns: {:?}\n left turns: {:?}\n",
    turn, direction, down, up, right, left
    ));
    */
    let mut temp_board = board.clone();
    let result = temp_board.minimax(0, max_level);
    let mut direction = String::from("up");

    if result.get_snake(YOU_ID).is_some() {
        let result_head = result.get_snake(YOU_ID).unwrap().get_head();
        let board_snake = board.get_snake(YOU_ID).unwrap();

        if result_head == board_snake.get_down() {
            direction = String::from("down");
        } else if result_head == board_snake.get_right() {
            direction = String::from("right");
        } else if result_head == board_snake.get_left() {
            direction = String::from("left");
        }
    }

    MoveResponse::new(direction, String::from("Hi!"))
}
