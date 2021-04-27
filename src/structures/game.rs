use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc;
use std::thread::spawn;

use crate::board::Board;
use crate::constants::{EXPONENT, LENGTH_ADVANTAGE, LOGGING, LOG_PATH, MAX_SEARCH, YOU_ID};
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

    // Returns the direction to go based on the game board
    pub fn decision(&self, board: Board) -> String {
        // Calculate max recursion depth
        let max_depth = max(EXPONENT / board.get_snakes().len() as i32, 1);

        // Predict future turns
        // Create a thread for down
        let mut down_board = board.clone();
        let (down_tx, down_rx) = mpsc::channel();
        let down_handle = spawn(move || {
            let down = down_board.check_down(0, max_depth);
            down_tx.send(down).unwrap();
        });
    
        // Create a thread for up
        let mut up_board = board.clone();
        let (up_tx, up_rx) = mpsc::channel();
        let up_handle = spawn(move || {
            let up = up_board.check_up(0, max_depth);
            up_tx.send(up).unwrap();
        });
    
        // Create a thread for right
        let mut right_board = board.clone();
        let (right_tx, right_rx) = mpsc::channel();
        let right_handle = spawn(move || {
            let right = right_board.check_right(0, max_depth);
            right_tx.send(right).unwrap();
        });
    
        // Create a thread for left
        let mut left_board = board.clone();
        let (left_tx, left_rx) = mpsc::channel();
        let left_handle = spawn(move || {
            let left = left_board.check_left(0, max_depth);
            left_tx.send(left).unwrap();
        });

        // About myself
        let you = &board.get_snakes()[0];
        let current_pos = you.get_head();
        let down_pos = current_pos.get_down();
        let up_pos = current_pos.get_up();
        let right_pos = current_pos.get_right();
        let left_pos = current_pos.get_left();

        let max_search = min(you.get_length() as i32, MAX_SEARCH);

        // Check area I can move in each direction
        // Check down area
        let down_area_board = board.clone();
        let (down_area_tx, down_area_rx) = mpsc::channel();
        let down_area_handle = spawn(move || {
            let down_area = down_area_board.check_area(
                current_pos.get_down(), 0, max_search,
                &mut Vec::with_capacity(max_search as usize), 0
            );
            down_area_tx.send(down_area).unwrap();
        });

        // Check up area
        let up_area_board = board.clone();
        let (up_area_tx, up_area_rx) = mpsc::channel();
        let up_area_handle = spawn(move || {
            let up_area = up_area_board.check_area(
                current_pos.get_up(), 0, max_search,
                &mut Vec::with_capacity(max_search as usize), 0
            );
            up_area_tx.send(up_area).unwrap();
        });

        // Check right area
        let right_area_board = board.clone();
        let (right_area_tx, right_area_rx) = mpsc::channel();
        let right_area_handle = spawn(move || {
            let right_area = right_area_board.check_area(
                current_pos.get_right(), 0, max_search,
                &mut Vec::with_capacity(max_search as usize), 0
            );
            right_area_tx.send(right_area).unwrap();
        });

        // Check left area
        let left_area_board = board.clone();
        let (left_area_tx, left_area_rx) = mpsc::channel();
        let left_area_handle = spawn(move || {
            let left_area = left_area_board.check_area(
                current_pos.get_left(), 0, max_search,
                &mut Vec::with_capacity(max_search as usize), 0
            );
            left_area_tx.send(left_area).unwrap();
        });

        // Find closest food
        let closest_food = board.find_closest_food(current_pos);
        let food_exists = closest_food.is_some();
        let closest_food = closest_food.unwrap_or_default();

        // Find closest weak snake
        let weak_snake_head = board.find_weaker_snake(you, LENGTH_ADVANTAGE);
        let weak_snake_exists = weak_snake_head.is_some();
        let weak_snake_head = weak_snake_head.unwrap_or_default();

        // Check if positions contain food
        let food_down = board.get_food().contains(&down_pos);
        let food_up = board.get_food().contains(&up_pos);
        let food_right = board.get_food().contains(&right_pos);
        let food_left = board.get_food().contains(&left_pos);

        // Check if positions are against wall
        let against_wall_down = board.is_against_wall(down_pos);
        let against_wall_up = board.is_against_wall(up_pos);
        let against_wall_right = board.is_against_wall(right_pos);
        let against_wall_left = board.is_against_wall(left_pos);
    
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

        // Find the best directions
        let mut best_boards = Vec::with_capacity(4);
        best_boards.push(&down_board);

        let you_index = YOU_ID as usize;
    
        if up_board[you_index] > best_boards[0][you_index] {
            best_boards.clear();
            best_boards.push(&up_board);
        } else if up_board[you_index] == best_boards[0][you_index] {
            best_boards.push(&up_board);
        }
        
        if right_board[you_index] > best_boards[0][you_index] {
            best_boards.clear();
            best_boards.push(&right_board);
        } else if right_board[you_index] == best_boards[0][you_index] {
            best_boards.push(&right_board);
        }
        
        if left_board[you_index] > best_boards[0][you_index] {
            best_boards.clear();
            best_boards.push(&left_board);
        } else if left_board[you_index] == best_boards[0][you_index] {
            best_boards.push(&left_board);
        }

        // Store best directions
        let down_best = best_boards.contains(&&down_board);
        let up_best = best_boards.contains(&&up_board);
        let right_best = best_boards.contains(&&right_board);
        let left_best = best_boards.contains(&&left_board);

        // True if I can survive
        let down_survival = down_board[you_index] > 0;
        let up_survival = up_board[you_index] > 0;
        let right_survival = right_board[you_index] > 0;
        let left_survival = left_board[you_index] > 0;

        // True if other snakes will die
        let will_kill = 100 - ((best_boards[0][you_index] / 1_000_000_000) % 100) < board.get_snakes().len() as u64;

        /*// Find max survival time
        let down_turn = down_board.get_turn();
        let up_turn = up_board.get_turn();
        let right_turn = right_board.get_turn();
        let left_turn = left_board.get_turn();
        // let max_turns = max(max(down_turn, up_turn), max(right_turn, left_turn));*/

        // Finish down_area thread
        let down_area = down_area_rx.recv().unwrap();
        down_area_handle.join().unwrap();
        let can_escape_down = down_area >= max_search;

        // Finish up_area thread
        let up_area = up_area_rx.recv().unwrap();
        up_area_handle.join().unwrap();
        let can_escape_up = up_area >= max_search;

        // Finish right_area thread
        let right_area = right_area_rx.recv().unwrap();
        right_area_handle.join().unwrap();
        let can_escape_right = right_area >= max_search;

        // Finish left_area thread
        let left_area = left_area_rx.recv().unwrap();
        left_area_handle.join().unwrap();
        let can_escape_left = left_area >= max_search;
    
        // Find max area available
        let max_area = max(max(down_area, up_area), max(right_area, left_area));


        // Take all data and decide on direction
        let decision;
        let direction;

        // Move towards kill with best move
        if down_survival
        && will_kill
        && down_best {
            decision = 0;
            direction = String::from("down");
        } else if
        up_survival
        && will_kill
        && up_best {
            decision = 1;
            direction = String::from("up");
        } else if
        right_survival
        && will_kill
        && right_best {
            decision = 2;
            direction = String::from("right");
        } else if
        left_survival
        && will_kill
        && left_best {
            decision = 3;
            direction = String::from("left");
        }

        // Move towards closest weak snake with best move, avoiding walls
        else if
        down_survival
        && can_escape_down
        && down_best
        && (!against_wall_down || food_down)
        && weak_snake_exists && weak_snake_head.get_y() < current_pos.get_y() {
            decision = 4;
            direction = String::from("down");
        } else if
        up_survival
        && can_escape_up
        && up_best
        && (!against_wall_up || food_up)
        && weak_snake_exists && weak_snake_head.get_y() > current_pos.get_y() {
            decision = 5;
            direction = String::from("up");
        } else if
        right_survival
        && can_escape_right
        && right_best
        && (!against_wall_right || food_right)
        && weak_snake_exists && weak_snake_head.get_x() > current_pos.get_x() {
            decision = 6;
            direction = String::from("right");
        } else if
        left_survival
        && can_escape_left
        && left_best
        && (!against_wall_left || food_left)
        && weak_snake_exists && weak_snake_head.get_x() < current_pos.get_x() {
            decision = 7;
            direction = String::from("left");
        }

        // Move towards closest food with best move, avoiding walls
        else if
        down_survival
        && can_escape_down
        && down_best
        && (!against_wall_down || food_down)
        && food_exists && closest_food.get_y() < current_pos.get_y() {
            decision = 8;
            direction = String::from("down");
        } else if
        up_survival
        && can_escape_up
        && up_best
        && (!against_wall_up || food_up)
        && food_exists && closest_food.get_y() > current_pos.get_y() {
            decision = 9;
            direction = String::from("up");
        } else if
        right_survival
        && can_escape_right
        && right_best
        && (!against_wall_right || food_right)
        && food_exists && closest_food.get_x() > current_pos.get_x() {
            decision = 10;
            direction = String::from("right");
        } else if
        left_survival
        && can_escape_left
        && left_best
        && (!against_wall_left || food_left)
        && food_exists && closest_food.get_x() < current_pos.get_x() {
            decision = 11;
            direction = String::from("left");
        }

        // Move towards escape with best move, avoiding walls
        else if
        down_survival
        && can_escape_down
        && down_best
        && (!against_wall_down || food_down) {
            decision = 12;
            direction = String::from("down");
        } else if 
        up_survival
        && can_escape_up
        && up_best 
        && (!against_wall_up || food_up) {
            decision = 13;
            direction = String::from("up");
        } else if
        right_survival
        && can_escape_right
        && right_best 
        && (!against_wall_right || food_right) {
            decision = 14;
            direction = String::from("right");
        } else if
        left_survival
        && can_escape_left
        && left_best 
        && (!against_wall_left || food_left) {
            decision = 15;
            direction = String::from("left");
        }

        // Move towards closest weak snake with best move
        else if
        down_survival
        && can_escape_down
        && down_best
        && weak_snake_exists && weak_snake_head.get_y() < current_pos.get_y() {
            decision = 16;
            direction = String::from("down");
        } else if 
        up_survival
        && can_escape_up
        && up_best
        && weak_snake_exists && weak_snake_head.get_y() > current_pos.get_y() {
            decision = 17;
            direction = String::from("up");
        } else if
        right_survival
        && can_escape_right
        && right_best
        && weak_snake_exists && weak_snake_head.get_x() > current_pos.get_x() {
            decision = 18;
            direction = String::from("right");
        } else if
        left_survival
        && can_escape_left
        && left_best
        && weak_snake_exists && weak_snake_head.get_x() < current_pos.get_x() {
            decision = 19;
            direction = String::from("left");
        }

        // Move towards closest food with best move
        else if
        down_survival
        && can_escape_down
        && down_best
        && food_exists && closest_food.get_y() < current_pos.get_y() {
            decision = 20;
            direction = String::from("down");
        } else if 
        up_survival
        && can_escape_up
        && up_best
        && food_exists && closest_food.get_y() > current_pos.get_y() {
            decision = 21;
            direction = String::from("up");
        } else if
        right_survival
        && can_escape_right
        && right_best
        && food_exists && closest_food.get_x() > current_pos.get_x() {
            decision = 22;
            direction = String::from("right");
        } else if
        left_survival
        && can_escape_left
        && left_best
        && food_exists && closest_food.get_x() < current_pos.get_x() {
            decision = 23;
            direction = String::from("left");
        }

        // Move towards escape with best move
        else if
        down_survival
        && can_escape_down
        && down_best {
            decision = 24;
            direction = String::from("down");
        } else if 
        up_survival
        && can_escape_up
        && up_best {
            decision = 25;
            direction = String::from("up");
        } else if
        right_survival
        && can_escape_right
        && right_best {
            decision = 16;
            direction = String::from("right");
        } else if
        left_survival
        && can_escape_left
        && left_best {
            decision = 27;
            direction = String::from("left");
        }

        // Move towards escape
        else if
        down_survival
        && can_escape_down {
            decision = 28;
            direction = String::from("down");
        } else if
        up_survival
        && can_escape_up {
            decision = 29;
            direction = String::from("up");
        } else if
        right_survival
        && can_escape_right {
            decision = 30;
            direction = String::from("right");
        } else if
        left_survival
        && can_escape_left {
            decision = 31;
            direction = String::from("left");
        }

        // Move towards best move, can escape
        else if
        can_escape_down
        && down_best {
            decision = 32;
            direction = String::from("down");
        } else if
        can_escape_up
        && up_best {
            decision = 33;
            direction = String::from("up");
        } else if
        can_escape_right
        && right_best {
            decision = 34;
            direction = String::from("right");
        } else if
        can_escape_left
        && left_best {
            decision = 35;
            direction = String::from("left");
        }

        // Move towards escape
        else if
        can_escape_down {
            decision = 36;
            direction = String::from("down");
        } else if
        can_escape_up {
            decision = 37;
            direction = String::from("up");
        } else if
        can_escape_right {
            decision = 38;
            direction = String::from("right");
        } else if
        can_escape_left {
            decision = 39;
            direction = String::from("left");
        }

        // Go for best move with most turns, no survival
        else if 
        down_best
        && down_area == max_area {
            decision = 40;
            direction = String::from("down");
        } else if
        up_best
        && up_area == max_area {
            decision = 41;
            direction = String::from("up");
        } else if
        right_best
        && right_area == max_area {
            decision = 42;
            direction = String::from("right");
        } else if
        left_best
        && left_area == max_area {
            decision = 43;
            direction = String::from("left")
        }

        // Go for most turns, no survival
        else if
        down_area == max_area {
            decision = 44;
            direction = String::from("down");
        } else if
        up_area == max_area {
            decision = 45;
            direction = String::from("up");
        } else if
        right_area == max_area {
            decision = 46;
            direction = String::from("right");
        } else if
        left_area == max_area {
            decision = 47;
            direction = String::from("left")
        }

        // Default
        else {
            decision = 48;
            direction = String::from("up");
        }

        // Log decision        
        self.log_data(format!("
       turn: {}
  direction: {}
   decision: {}
  will kill: {}
  max turns: {}
 down turns: {}
   up turns: {}
right turns: {}
 left turns: {}
   max area: {}
  down area: {}
    up area: {}
 right area: {}
  left area: {}",
board.get_turn(), direction, decision, will_kill, max_depth, -1, -1, -1, -1, max_search, down_area, up_area, right_area, left_area)
);
    
        // Return direction
        direction
    }

    // Prints data to stdout and writes to log file
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

#[cfg(test)]
mod tests {
    use crate::load_object;
    use crate::move_request::MoveRequest;

    #[test]
    fn test_decision() {
        let data = load_object!(MoveRequest, String::from("test_board-03"));
        
        let values = data.into_values();
        let board = values.2.into_board(values.3, values.1);
        let game = values.0;
        let direction = game.decision(board);

        assert_eq!(direction, String::from("right"));
    }
}
