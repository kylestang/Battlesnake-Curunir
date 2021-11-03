#[macro_export]
macro_rules! load_object {
    (Board, $filename:expr, $test_path:expr) => {{
        let file: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .open(format!("{}{}.json", $test_path, $filename))
            .unwrap();
        let board: crate::move_request::MoveRequest = serde_json::from_reader(file).unwrap();
        let (_, turn, board, you) = board.into_values();
        let board = board.into_board(you, turn);
        board
    }};
    (Battlesnake, $filename:expr, $test_path:expr) => {{
        let file: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .open(format!("{}{}.json", $test_path, $filename))
            .unwrap();
        let snake: crate::input_snake::InputSnake = from_reader(file).unwrap();
        let snake = snake.into_battlesnake();
        snake
    }};
    (Ruleset, $filename:expr, $test_path:expr) => {{
        let file: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .open(format!("{}{}.json", $test_path, $filename))
            .unwrap();
        let board: crate::move_request::MoveRequest = serde_json::from_reader(file).unwrap();
        let (input_game, _, _, _) = board.into_values();
        input_game.into_ruleset()
    }};
    ($type:ident, $filename:expr, $test_path:expr) => {{
        let file: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .open(format!("{}{}.json", $test_path, $filename))
            .unwrap();
        let object: $type = serde_json::from_reader(file).unwrap();
        object
    }};
}
