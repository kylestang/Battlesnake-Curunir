use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc;
use std::thread::spawn;

use crate::board::Board;
use crate::board_order::BoardOrder::*;
use crate::constants::{LOGGING, LOG_PATH, MAX_LEVEL, YOU_ID};
use crate::ruleset::Ruleset;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: String,
    ruleset: Ruleset,
    timeout: i32
}

impl Game {
    pub fn _new(id: String, ruleset: Ruleset, timeout: i32) -> Game {
        Game {id, ruleset, timeout}
    }

    pub fn _get_id(&self) -> &String {
        &self.id
    }

    pub fn _get_ruleset(&self) -> &Ruleset {
        &self.ruleset
    }

    pub fn _get_timeout(&self) -> i32 {
        self.timeout
    }

    pub fn decision(&self, board: Board) -> String {
        // Predict future turns
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
        let down_board = down_rx.recv().unwrap();
        down_handle.join().unwrap();
    
        // Finish up thread
        let up_board = up_rx.recv().unwrap();
        up_handle.join().unwrap();
    
        // Finish right thread
        let right_board = right_rx.recv().unwrap();
        right_handle.join().unwrap();
    
        // Finish left thread
        let left_board = left_rx.recv().unwrap();
        left_handle.join().unwrap();
    
        let mut best_boards = Vec::with_capacity(4);
        
        best_boards.push(&down_board);
    
        let up_comp = up_board.compare_to(best_boards[0], YOU_ID);
        if up_comp == Greater {
            best_boards.clear();
            best_boards.push(&up_board);
        } else if up_comp == Equal {
            best_boards.push(&up_board);
        }
        
        let right_comp = right_board.compare_to(best_boards[0], YOU_ID);
        if right_comp == Greater {
            best_boards.clear();
            best_boards.push(&right_board);
        } else if right_comp == Equal {
            best_boards.push(&right_board);
        }
        
        let left_comp = left_board.compare_to(best_boards[0], YOU_ID);
        if left_comp == Greater {
            best_boards.clear();
            best_boards.push(&left_board);
        } else if left_comp == Equal {
            best_boards.push(&left_board);
        }

        let you = &board.get_snakes()[0];
        let current_pos = you.get_head();

        let down_best = best_boards.contains(&&down_board);
        let up_best = best_boards.contains(&&up_board);
        let right_best = best_boards.contains(&&right_board);
        let left_best = best_boards.contains(&&left_board);

        let survival = best_boards[0].get_snake(YOU_ID).is_some();

        // Find closest food
        let closest_food = board.find_closest_food(current_pos);
        let food_exists = closest_food.is_some();

        // Find max survival time
        let down_turn = down_board.get_turn();
        let up_turn = up_board.get_turn();
        let right_turn = right_board.get_turn();
        let left_turn = left_board.get_turn();
        let max_turns = max(max(down_turn, up_turn), max(right_turn, left_turn));


        let decision;
        let direction;

        // Move towards closest food with best move
        if
            survival
            && down_best
            && food_exists && closest_food.unwrap().get_y() < current_pos.get_y() {
            decision = 0;
            direction = String::from("down");
        } else if 
            survival
            && up_best
            && food_exists && closest_food.unwrap().get_y() > current_pos.get_y() {
            decision = 1;
            direction = String::from("up");
        } else if
            survival
            && right_best
            && food_exists && closest_food.unwrap().get_x() > current_pos.get_x() {
            decision = 2;
            direction = String::from("right");
        } else if
            survival
            && left_best
            && food_exists && closest_food.unwrap().get_x() < current_pos.get_x() {
            decision = 3;
            direction = String::from("left");
        }

        // Move towards best move
        else if 
            survival
            && down_best {
            decision = 4;
            direction = String::from("down");
        } else if 
            survival
            && up_best {
            decision = 5;
            direction = String::from("up");
        } else if 
            survival
            && right_best {
            decision = 6;
            direction = String::from("right");
        } else if 
            survival
            && left_best {
            decision = 7;
            direction = String::from("left");
        }

        // Go for most turns, no survival
        else if 
            down_best
            && down_turn == max_turns {
            decision = 8;
            direction = String::from("down");
        } else if
            up_best
            && up_turn == max_turns {
            decision = 9;
            direction = String::from("up");
        } else if
            right_best
            && right_turn == max_turns {
            decision = 10;
            direction = String::from("right");
        } else if
            left_best
            && left_turn == max_turns {
            decision = 11;
            direction = String::from("left")
        }

        // Default
        else {
            decision = 11;
            direction = String::from("up");
        }

        // Log decision        
        self.log_data(format!("turn: {}\ndirection: {}\ndecision: {}\n", board.get_turn(), direction, decision));
    
        // Return direction
        direction
    }

    pub fn log_data(&self, data: String) {
        let data = data + "\n\n";
        print!("{}", data);
        if LOGGING {    
            let mut file: File = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{}{}.log", LOG_PATH, self.id))
                .unwrap();
            
            file.write_all(data.as_bytes()).unwrap();
        }
    }
}
