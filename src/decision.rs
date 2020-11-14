use crate::requests::MoveResponse;
use crate::structures::{battlesnake, board, game};
use battlesnake::Battlesnake;
use board::Board;
use game::Game;
use crate::constants::DIRECTIONS;

pub fn decision(game: &Game, turn: i32, mut board: Board, mut you: Battlesnake) -> MoveResponse {    
    
    test3(&mut board, &mut you, 0, 2);

    MoveResponse::new(String::from("left"), String::from("Hi!"))
}

/*
    n = number of snakes
    # of possible moves is 4^n
    store level: number of turns simulated
    store count: number of iterations completed in level
    
    use integer division to find position of snakes: for last snake -> (count // 4 ^ 0) % 4, for first snake -> (count // 4 ^ n-1) % 4


*/

fn test3(board: &mut Board, you: &Battlesnake, level: i32, max_level: i32) -> bool {
    let num_snakes = board.get_snakes().len();

    if level >= max_level || num_snakes == 0 {
        return board.get_snakes().contains(you);
    }

    for count in 0..DIRECTIONS.pow(num_snakes as u32) {
        let mut current_board = board.clone();
        current_board.draw(String::from("test"));

        for i in 0..num_snakes {
            let snake = &mut current_board.get_snakes_mut()[i];
            let adjacent = snake.get_head().get_adjacent();

            snake.move_to(adjacent[(count as usize / (DIRECTIONS.pow(i as u32))) % DIRECTIONS]);
            current_board.draw(String::from("test"));
        }

        current_board.game_step();
        current_board.draw(String::from("test"));

        test3(&mut current_board, you, level + 1, max_level);
    }

    false
}
