use board::Board;
use game::Game;
use std::sync::mpsc;
use std::thread::spawn;

use crate::constants::{MAX_LEVEL, YOU_ID};
use crate::move_response::MoveResponse;
use crate::structures::{board, game};

pub fn decision(game: Game, turn: i32, board: Board) -> MoveResponse {
    // Create a thread for down
    let mut down_board = board.clone();
    let (down_tx, down_rx) = mpsc::channel();
    let down_handle = spawn(move || {
        let down = down_board.check_down(0, MAX_LEVEL);
        down_tx.send(down).unwrap();
    });

    // Create a thread for up
    let mut up_board = board.clone();
    let (up_tx, up_rx) = mpsc::channel();
    let up_handle = spawn(move || {
        let up = up_board.check_up(0, MAX_LEVEL);
        up_tx.send(up).unwrap();
    });

    // Create a thread for right
    let mut right_board = board.clone();
    let (right_tx, right_rx) = mpsc::channel();
    let right_handle = spawn(move || {
        let right = right_board.check_right(0, MAX_LEVEL);
        right_tx.send(right).unwrap();
    });

    // Create a thread for left
    let mut left_board = board.clone();
    let (left_tx, left_rx) = mpsc::channel();
    let left_handle = spawn(move || {
        let left = left_board.check_left(0, MAX_LEVEL);
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

    let mut best_board = down;
    let mut direction = String::from("down");

    if up.better_than(&best_board, YOU_ID) {
        direction = String::from("up");
        best_board = up;
    }
    
    if right.better_than(&best_board, YOU_ID) {
        direction = String::from("right");
        best_board = right;
    }
    
    if left.better_than(&best_board, YOU_ID) {
        direction = String::from("left");
    }
    
    game.log_data(format!("turn: {}\n  direction: {}\n", turn, direction));

    MoveResponse::new(direction, String::from("Hi!"))
}
