use crate::requests::MoveResponse;
use crate::structures::{battlesnake, board, game};
use battlesnake::Battlesnake;
use board::Board;
use game::Game;
use std::thread::spawn;
use std::sync::mpsc;

pub fn decision(game: &Game, turn: i32, board: Board, _you: Battlesnake) -> MoveResponse {

    // Create a thread for down
    let down_board = board.clone();
    let (down_tx, down_rx) = mpsc::channel();
    let down_handle = spawn(move || {
        let down = down_board.test_down();
        down_tx.send(down).unwrap();
    });

    // Create a thread for up
    let up_board = board.clone();
    let (up_tx, up_rx) = mpsc::channel();
    let up_handle = spawn(move || {
        let up = up_board.test_up();
        up_tx.send(up).unwrap();
    });

    // Create a thread for right
    let right_board = board.clone();
    let (right_tx, right_rx) = mpsc::channel();
    let right_handle = spawn(move || {
        let right = right_board.test_right();
        right_tx.send(right).unwrap();
    });

    // Create a thread for left
    let left_board = board.clone();
    let (left_tx, left_rx) = mpsc::channel();
    let left_handle = spawn(move || {
        let left = left_board.test_left();
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
    let mut survival = false;

    if down {
        direction = String::from("down");
        survival = true;
    }
    else if up {
        direction = String::from("up");
        survival = true;
    }
    else if right {
        direction = String::from("right");
        survival = true;
    }
    else if left {
        direction = String::from("left");
        survival = true;
    }

    game.log_data(format!("turn: {}\ndirection: {}\nsurvival: {}", turn, direction, survival));

    MoveResponse::new(direction, String::from("Hi!"))
}
