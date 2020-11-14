use crate::structures::{Board, Coordinate, Battlesnake};
use crate::constants::{LOG_PATH, LOGGING, YOU_ID};
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn will_collide(board: &mut Board, position: &Coordinate) -> bool{

    if position.get_x() < 0 || position.get_x() > board.get_width() - 1
    || position.get_y() < 0 || position.get_y() > board.get_height() - 1
    {
        return true;
    }

    for snake in board.get_snakes() {
        if snake.get_id() == YOU_ID {
            for i in 1..snake.get_length() as usize {
                if &snake.get_body()[i] == position {
                    return true;
                }
            }
        } else if snake.get_body().contains(position) {
            return true;
        }
    }

    return false;
}

pub fn game_step(board: &mut Board) {

    // Any Battlesnake that has found food will consume it
    for i in 0..board.get_snakes().len() {
        let mut j = 0;
        while j < board.get_food().len() {
            let food = board.get_food()[j];
            if board.get_snakes()[i].get_head() == food {
                board.get_food().remove(j);
                board.get_snakes()[i].eat_food();
                continue;
            }
            j += 1;
        }
    }

    /* 
    Any Battlesnake that has been eliminated is removed from the game board:
        Health less than or equal to 0
        Moved out of bounds
        Collided with themselves
        Collided with another Battlesnake
        Collided head-to-head and lost
    */
    let mut i = 0;
    while i < board.get_snakes().len() {
        if board.get_snakes()[i].get_health() <= 0 {
            board.get_snakes().remove(i);
            continue;
        }

        let position = board.get_snakes()[i].get_head();
        if will_collide(board, &position) {
            board.get_snakes().remove(i);
            continue;
        }

        let snakes = board.get_snakes();

        let mut j = i + 1;
        let pos = snakes[i].get_head();
        let length = snakes[i].get_length();
        while j < snakes.len() {
            if snakes[j].get_head() == pos {
                if length > snakes[j].get_length() {
                    snakes.remove(j);
                }
                else if length < snakes[j].get_length() {
                    snakes.remove(i);
                }
                else {
                    snakes.remove(j);
                    snakes.remove(i);
                }
            }
            j += 1;
        }
        i += 1;
    }
}

pub fn log_data(data: String, file_name: &String) {
    println!("{}", data);
    if LOGGING{

        let mut file: File = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}{}.log", LOG_PATH, file_name))
            .unwrap();
        
        file.write_all(data.as_bytes()).unwrap();
    }
}
