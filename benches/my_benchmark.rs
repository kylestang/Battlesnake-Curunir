use criterion::{black_box, criterion_group, criterion_main, Criterion};
use curunir::requests::*;
use curunir::constants::*;

macro_rules! load_object {
    (Board, $filename:expr) => {
        {
            let file: std::fs::File = std::fs::OpenOptions::new()
                .read(true).open(format!("{}{}.json", curunir::constants::_TEST_PATH, $filename)).unwrap();
            let board: move_request::MoveRequest = serde_json::from_reader(file).unwrap();
            let board = board.into_values();
            let board = board.2.into_board(board.3, 0);
            board
        }
    };
    (Battlesnake, $filename:expr) => {
        {
            let file: std::fs::File =std::fs::OpenOptions::new()
                .read(true).open(format!("{}{}.json", curunir::constants::_TEST_PATH, $filename)).unwrap();
            let snake: crate::input_snake::InputSnake = from_reader(file).unwrap();
            let snake = snake.into_battlesnake();
            snake
        }
    };
    ($type:ident, $filename:expr) => {
        {
            let file: std::fs::File = std::fs::OpenOptions::new()
                .read(true).open(format!("{}{}.json", curunir::constants::_TEST_PATH, $filename)).unwrap();
            let object: $type = serde_json::from_reader(file).unwrap();
            object
        }
    };
}

pub fn body_collision_with_bench(c: &mut Criterion) {
    let board = load_object!(Board, "test_board-03");
    let snake1 = black_box(&board.get_snakes()[1]);
    let snake2 = black_box(&board.get_snakes()[0]);

    c.bench_function("body_collision_with", |b| b.iter(|| snake1.body_collision_with(snake2)));
}

pub fn get_option_bench(c: &mut Criterion) {
    let board = load_object!(Board, "simple-01");
    let snake = black_box(&board.get_snakes()[0]);
    let value = black_box(0);

    c.bench_function("get_options", |b| b.iter(|| snake.get_option(value)));
}

pub fn game_step_bench(c: &mut Criterion) {
    let board = black_box(load_object!(Board, "food-01"));

    c.bench_function("game_step", |b| b.iter(|| board.clone().game_step()));

}

pub fn minimax_bench(c: &mut Criterion) {
    let board = black_box(load_object!(Board, "test_board-03"));
    let current = black_box(0);
    let max_level = black_box(MAX_LEVEL);

    c.bench_function("minimax", |b| b.iter(|| board.clone().minimax(current, max_level)));
}

criterion_group!(benches, body_collision_with_bench, get_option_bench, game_step_bench, minimax_bench);
criterion_main!(benches);
