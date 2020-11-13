use crate::structures::{Game, Battlesnake, Board, MoveResponse};
use crate::constants::DIRECTIONS;
use crate::functions::game_step;
use crate::draw::draw_board;

pub fn decision(game: &Game, turn: i32, mut board: Board, mut you: Battlesnake) -> MoveResponse {    
    
    test(&mut board, &mut you, 0, 1);

    MoveResponse::new(String::from("left"), String::from("Hi!"))
}


/*
    n = number of snakes
    # of possible moves is 4^n
    store level: number of turns simulated
    store count: number of iterations completed in level
    
    use integer division to find position of snakes: for last snake -> (count // 4 ^ 0) % 4, for first snake -> (count // 4 ^ n-1) % 4


*/
fn test(board: &mut Board, you: &mut Battlesnake, level: i32, max_level: i32) -> bool {
    if !board.get_snakes().contains(you) {
        return false;
    }

    if level >= max_level {
        return board.get_snakes().contains(you);
    }

    let num_snakes = board.get_snakes().len();

    for count in 0..DIRECTIONS.pow(num_snakes as u32) {

        let mut current_board = board.clone();
        
        draw_board(&mut current_board, String::from("test"));

        for i in 0..num_snakes {
            let snake = &mut current_board.get_snakes()[i];
            let adjacent = snake.get_head().get_adjacent();

            snake.move_to(adjacent[(count as usize / (DIRECTIONS.pow(i as u32))) % DIRECTIONS]);
            draw_board(&mut current_board, String::from("test"));
        }

        game_step(&mut current_board);
        draw_board(&mut current_board, String::from("test"));

        if test(&mut current_board, you, level + 1, max_level) {
            return true;
        }

    }

    return false;

}
