use serde::{Deserialize, Serialize};
use std::cmp::{max, min, Ordering};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc;
use std::thread::spawn;

use crate::board::Board;
use crate::constants::{EXPONENT, LENGTH_ADVANTAGE, LOG_LEVEL, LOG_PATH, MAX_SEARCH, YOU_ID};
use crate::ruleset::Ruleset;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: String,
    ruleset: Ruleset,
    timeout: i32,
}

impl Game {
    pub fn new(id: String, ruleset: Ruleset, timeout: i32) -> Game {
        Game {
            id,
            ruleset,
            timeout,
        }
    }

    // Returns the direction to go based on the game board
    pub fn calculate_move(&self, board: Board) -> String {
        // Calculate max recursion depth
        let max_depth = max(EXPONENT / board.get_snakes().len() as i32, 1);

        // Predict future turns
        // Create a thread for down
        let down_board = board.clone();
        let down_ruleset = self.ruleset.clone();
        let (down_tx, down_rx) = mpsc::channel();
        let down_handle = spawn(move || {
            let down = down_board.check_down(&down_ruleset, 0, max_depth);
            down_tx.send(down).unwrap();
        });

        // Create a thread for up
        let up_board = board.clone();
        let up_ruleset = self.ruleset.clone();
        let (up_tx, up_rx) = mpsc::channel();
        let up_handle = spawn(move || {
            let up = up_board.check_up(&up_ruleset, 0, max_depth);
            up_tx.send(up).unwrap();
        });

        // Create a thread for right
        let right_board = board.clone();
        let right_ruleset = self.ruleset.clone();
        let (right_tx, right_rx) = mpsc::channel();
        let right_handle = spawn(move || {
            let right = right_board.check_right(&right_ruleset, 0, max_depth);
            right_tx.send(right).unwrap();
        });

        // Create a thread for left
        let left_board = board.clone();
        let left_ruleset = self.ruleset.clone();
        let (left_tx, left_rx) = mpsc::channel();
        let left_handle = spawn(move || {
            let left = left_board.check_left(&left_ruleset, 0, max_depth);
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
            let down_area = down_area_board.longest_path(
                current_pos.get_down(),
                0,
                max_search,
                &mut Vec::with_capacity(max_search as usize),
                0,
            );
            down_area_tx.send(down_area).unwrap();
        });

        // Check up area
        let up_area_board = board.clone();
        let (up_area_tx, up_area_rx) = mpsc::channel();
        let up_area_handle = spawn(move || {
            let up_area = up_area_board.longest_path(
                current_pos.get_up(),
                0,
                max_search,
                &mut Vec::with_capacity(max_search as usize),
                0,
            );
            up_area_tx.send(up_area).unwrap();
        });

        // Check right area
        let right_area_board = board.clone();
        let (right_area_tx, right_area_rx) = mpsc::channel();
        let right_area_handle = spawn(move || {
            let right_area = right_area_board.longest_path(
                current_pos.get_right(),
                0,
                max_search,
                &mut Vec::with_capacity(max_search as usize),
                0,
            );
            right_area_tx.send(right_area).unwrap();
        });

        // Check left area
        let left_area_board = board.clone();
        let (left_area_tx, left_area_rx) = mpsc::channel();
        let left_area_handle = spawn(move || {
            let left_area = left_area_board.longest_path(
                current_pos.get_left(),
                0,
                max_search,
                &mut Vec::with_capacity(max_search as usize),
                0,
            );
            left_area_tx.send(left_area).unwrap();
        });

        // Find closest food
        let closest_food = board.find_closest_food(current_pos);
        let closest_food_exists = closest_food.is_some()
            && board.get_snakes().iter().any(|snake| {
                snake.get_id() != YOU_ID
                    && snake.get_length() as i32 > you.get_length() as i32 - LENGTH_ADVANTAGE
            });
        let closest_food = closest_food.unwrap_or_default();

        // Find closest weak snake
        let weak_snake_head = board.find_weaker_snake(you, LENGTH_ADVANTAGE);
        let _weak_snake_exists = weak_snake_head.is_some();
        let _weak_snake_head = weak_snake_head.unwrap_or_default();

        // Check if positions are against the wall or if they contain food
        let down_wall = !board.is_against_wall(down_pos) || board.get_food().contains(&down_pos);
        let up_wall = !board.is_against_wall(up_pos) || board.get_food().contains(&up_pos);
        let right_wall = !board.is_against_wall(right_pos) || board.is_against_wall(right_pos);
        let left_wall = !board.is_against_wall(left_pos) || board.get_food().contains(&left_pos);

        // Find best area controls
        let control_areas = board.calculate_areas(&self.ruleset);
        let max_control = *control_areas.iter().max().unwrap();
        let [down_control, up_control, right_control, left_control] = control_areas;

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

        match up_board.cmp(&best_boards[0]) {
            Ordering::Greater => {
                best_boards.clear();
                best_boards.push(&up_board);
            }
            Ordering::Equal => best_boards.push(&up_board),
            Ordering::Less => {}
        }

        match right_board.cmp(&best_boards[0]) {
            Ordering::Greater => {
                best_boards.clear();
                best_boards.push(&right_board);
            }
            Ordering::Equal => best_boards.push(&right_board),
            Ordering::Less => {}
        }

        match left_board.cmp(&best_boards[0]) {
            Ordering::Greater => {
                best_boards.clear();
                best_boards.push(&left_board);
            }
            Ordering::Equal => best_boards.push(&left_board),
            Ordering::Less => {}
        }

        // Store best directions
        let down_best = best_boards.contains(&&down_board);
        let up_best = best_boards.contains(&&up_board);
        let right_best = best_boards.contains(&&right_board);
        let left_best = best_boards.contains(&&left_board);

        // True if I can survive
        let down_survival = down_board > 0;
        let up_survival = up_board > 0;
        let right_survival = right_board > 0;
        let left_survival = left_board > 0;

        // True if other snakes will die
        let will_kill = 100 - ((best_boards[0] / 1_000_000_000) % 100)
            < board.get_snakes().len() as u64;

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
        let outcome;
        let direction;

        // Move towards kill with best move
        if down_survival && will_kill && down_best {
            outcome = 0;
            direction = String::from("down");
        } else if up_survival && will_kill && up_best {
            outcome = 1;
            direction = String::from("up");
        } else if right_survival && will_kill && right_best {
            outcome = 2;
            direction = String::from("right");
        } else if left_survival && will_kill && left_best {
            outcome = 3;
            direction = String::from("left");
        }
        // Move towards closest food with best move avoiding walls
        else if down_survival
            && can_escape_down
            && down_best
            && down_wall
            && closest_food_exists
            && closest_food.get_y() < current_pos.get_y()
        {
            outcome = 4;
            direction = String::from("down");
        } else if down_survival
            && can_escape_up
            && up_best
            && up_wall
            && closest_food_exists
            && closest_food.get_y() > current_pos.get_y()
        {
            outcome = 5;
            direction = String::from("up");
        } else if right_survival
            && can_escape_right
            && right_best
            && right_wall
            && closest_food_exists
            && closest_food.get_x() > current_pos.get_x()
        {
            outcome = 6;
            direction = String::from("right");
        } else if left_survival
            && can_escape_left
            && left_best
            && left_wall
            && closest_food_exists
            && closest_food.get_x() < current_pos.get_x()
        {
            outcome = 7;
            direction = String::from("left");
        } 
        // Move towards largest area of control with best move avoiding walls
        else if down_survival
            && can_escape_down
            && down_best
            && down_control == max_control
            && down_wall
        {
            outcome = 8;
            direction = String::from("down");
        } else if up_survival
            && can_escape_up
            && up_best
            && up_control == max_control
            && up_wall
        {
            outcome = 9;
            direction = String::from("up");
        } else if right_survival
            && can_escape_right
            && right_best
            && right_control == max_control
            && right_wall
        {
            outcome = 10;
            direction = String::from("right");
        } else if left_survival
            && can_escape_left
            && left_best
            && left_control == max_control
            && left_wall
        {
            outcome = 11;
            direction = String::from("left");
        }
        // Move towards escape with best move avoiding walls
        else if down_survival && can_escape_down && down_best && down_wall
        {
            outcome = 12;
            direction = String::from("down");
        } else if up_survival && can_escape_up && up_best && up_wall {
            outcome = 13;
            direction = String::from("up");
        } else if right_survival
            && can_escape_right
            && right_best
            && right_wall
        {
            outcome = 14;
            direction = String::from("right");
        } else if left_survival && can_escape_left && left_best && left_wall
        {
            outcome = 15;
            direction = String::from("left");
        }
        // Move towards closest food with best move
        else if down_survival
            && can_escape_down
            && down_best
            && closest_food_exists
            && closest_food.get_y() < current_pos.get_y()
        {
            outcome = 16;
            direction = String::from("down");
        } else if down_survival
            && can_escape_up
            && up_best
            && closest_food_exists
            && closest_food.get_y() > current_pos.get_y()
        {
            outcome = 17;
            direction = String::from("up");
        } else if right_survival
            && can_escape_right
            && right_best
            && closest_food_exists
            && closest_food.get_x() > current_pos.get_x()
        {
            outcome = 18;
            direction = String::from("right");
        } else if left_survival
            && can_escape_left
            && left_best
            && closest_food_exists
            && closest_food.get_x() < current_pos.get_x()
        {
            outcome = 19;
            direction = String::from("left");
        } 
        // Move towards largest area of control with best move
        else if down_survival
            && can_escape_down
            && down_best
            && down_control == max_control
        {
            outcome = 20;
            direction = String::from("down");
        } else if up_survival
            && can_escape_up
            && up_best
            && up_control == max_control
        {
            outcome = 21;
            direction = String::from("up");
        } else if right_survival
            && can_escape_right
            && right_best
            && right_control == max_control
        {
            outcome = 22;
            direction = String::from("right");
        } else if left_survival
            && can_escape_left
            && left_best
            && left_control == max_control
        {
            outcome = 23;
            direction = String::from("left");
        }
        // Move towards escape with best move
        else if down_survival && can_escape_down && down_best
        {
            outcome = 24;
            direction = String::from("down");
        } else if up_survival && can_escape_up && up_best{
            outcome = 25;
            direction = String::from("up");
        } else if right_survival
            && can_escape_right
            && right_best
        {
            outcome = 26;
            direction = String::from("right");
        } else if left_survival && can_escape_left && left_best
        {
            outcome = 27;
            direction = String::from("left");
        }
        // Move towards escape with largest area of control
        else if down_survival && can_escape_down && down_control == max_control {
            outcome = 28;
            direction = String::from("down");
        } else if up_survival && can_escape_up && up_control == max_control {
            outcome = 29;
            direction = String::from("up");
        } else if right_survival && can_escape_right && right_control == max_control {
            outcome = 30;
            direction = String::from("right");
        } else if left_survival && can_escape_left && left_control == max_control {
            outcome = 31;
            direction = String::from("left");
        }
        // Move towards escape
        else if down_survival && can_escape_down {
            outcome = 32;
            direction = String::from("down");
        } else if up_survival && can_escape_up {
            outcome = 33;
            direction = String::from("up");
        } else if right_survival && can_escape_right {
            outcome = 34;
            direction = String::from("right");
        } else if left_survival && can_escape_left {
            outcome = 35;
            direction = String::from("left");
        }
        // Move towards best move, can escape
        else if can_escape_down && down_best {
            outcome = 36;
            direction = String::from("down");
        } else if can_escape_up && up_best {
            outcome = 37;
            direction = String::from("up");
        } else if can_escape_right && right_best {
            outcome = 38;
            direction = String::from("right");
        } else if can_escape_left && left_best {
            outcome = 39;
            direction = String::from("left");
        }
        // Move towards escape with largest area of control
        else if can_escape_down && down_control == max_control {
            outcome = 40;
            direction = String::from("down");
        } else if can_escape_up && up_control == max_control {
            outcome = 41;
            direction = String::from("up");
        } else if can_escape_right && right_control == max_control {
            outcome = 42;
            direction = String::from("right");
        } else if can_escape_left && left_control == max_control {
            outcome = 43;
            direction = String::from("left");
        }
        // Go for best move with most turns, no survival
        else if down_best && down_area == max_area {
            outcome = 44;
            direction = String::from("down");
        } else if up_best && up_area == max_area {
            outcome = 45;
            direction = String::from("up");
        } else if right_best && right_area == max_area {
            outcome = 46;
            direction = String::from("right");
        } else if left_best && left_area == max_area {
            outcome = 47;
            direction = String::from("left")
        }
        // Go for most turns, no survival
        else if down_area == max_area {
            outcome = 48;
            direction = String::from("down");
        } else if up_area == max_area {
            outcome = 49;
            direction = String::from("up");
        } else if right_area == max_area {
            outcome = 50;
            direction = String::from("right");
        } else if left_area == max_area {
            outcome = 51;
            direction = String::from("left")
        }
        // Default
        else {
            outcome = 52;
            direction = String::from("up");
        }

        // Log decision
        self.log_data(format!(
            "
        turn: {}
   direction: {}
    decision: {}
   will kill: {}
   max turns: {}
 down result: {}
   up result: {}
right result: {}
 left result: {}
    max path: {}
   down path: {}
     up path: {}
  right path: {}
   left path: {}
   down area: {}
     up area: {}
  right area: {}
   left area: {}",
            board.get_turn(),
            direction,
            outcome,
            will_kill,
            max_depth,
            down_board,
            up_board,
            right_board,
            left_board,
            max_search,
            down_area,
            up_area,
            right_area,
            left_area,
            down_control,
            up_control,
            right_control,
            left_control
        ));

        // Return direction
        direction
    }

    // Prints data to stdout and writes to log file
    pub fn log_data(&self, data: String) {
        let data = data + "\n\n";
        if LOG_LEVEL >= 1 {
            print!("{}", data);
        }
        
        if LOG_LEVEL >= 2 {
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
    use crate::constants::_TEST_PATH;
    use crate::load_object;
    use crate::move_request::MoveRequest;

    #[test]
    fn test_decision() {
        let data = load_object!(MoveRequest, String::from("test_board-04"), _TEST_PATH);

        let values = data.into_values();
        let board = values.2.into_board(values.3, values.1);
        let game = values.0.into_game();
        let direction = game.calculate_move(board);

        assert_eq!(direction, String::from("left"));
    }

    #[test]
    fn test_avoid_headon() {
        let data = load_object!(MoveRequest, String::from("test_board-06"), _TEST_PATH);

        let values = data.into_values();
        let board = values.2.into_board(values.3, values.1);
        let game = values.0.into_game();
        let direction = game.calculate_move(board);

        assert_eq!(direction, String::from("down"));
    }
}
