use crate::structures::{Board, Coordinate, Battlesnake};
use crate::constants;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn will_collide(width: i32, height: i32, snakes: &Vec<Battlesnake>, position: &Coordinate) -> bool{

    if position.get_x() < 0 || position.get_x() > width - 1
    || position.get_y() < 0 || position.get_y() > height - 1
    {
        return true;
    }

    for snake in snakes {
        for i in 1..snake.get_length() as usize - 1 {
            if &snake.get_body()[i] == position {
                return true;
            }
        }

    }

    return false;
}

pub fn game_step(board: &mut Board) {
    let (width, height, food, _hazards, snakes) = board.get_mut();

    // Any Battlesnake that has found food will consume it
    for i in 0..snakes.len() {
        let mut j = 0;
        while j < food.len() {
            if *snakes[i].get_head() == food[i] {
                food.remove(i);
                snakes[i].eat_food();
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
    while i < snakes.len() {
        if snakes[i].get_health() <= 0 {
            snakes.remove(i);
            continue;
        }

        if will_collide(width, height, snakes, snakes[i].get_head()) {
            snakes.remove(i);
            continue;
        }

        let mut j = i + 1;
        let pos = *snakes[i].get_head();
        let length = snakes[i].get_length();
        while j < snakes.len() {
            if *snakes[j].get_head() == pos {
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
    if constants::LOGGING{

        let mut file: File = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}{}.log", constants::LOG_PATH, file_name))
            .unwrap();
        
        file.write_all(data.as_bytes()).unwrap();
    }
}
