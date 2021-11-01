use criterion::{black_box, criterion_group, criterion_main, Criterion};
use curunir::constants;
use curunir::game_objects::engine::Engine;
use curunir::game_objects::mapper::Mapper;
use curunir::game_objects::simulator::Simulator;
use curunir::load_object;
use curunir::requests::*;
use curunir::structures::coordinate::Coordinate;

use std::cmp::max;

pub fn area_controlled_bench(c: &mut Criterion) {
    let mut board = load_object!(Board, "test_board-03", constants::_TEST_PATH);
    let mapper = Mapper::new(board.clone());

    c.bench_function("area_controlled", |b| {
        b.iter(|| mapper.area_controlled(&mut board))
    });
}

pub fn body_collision_with_bench(c: &mut Criterion) {
    let board = load_object!(Board, "test_board-03", constants::_TEST_PATH);
    let snake1 = black_box(&board.get_snakes()[1]);
    let snake2 = black_box(&board.get_snakes()[0]);

    c.bench_function("body_collision_with", |b| {
        b.iter(|| snake1.body_collision_with(snake2))
    });
}

pub fn check_area_bench(c: &mut Criterion) {
    let board = black_box(load_object!(
        Board,
        "check_area_closed-01",
        constants::_TEST_PATH
    ));

    c.bench_function("check_area", |b| {
        b.iter(|| {
            board.check_area(
                Coordinate::new(7, 10),
                0,
                30,
                &mut Vec::with_capacity(45),
                0,
            )
        })
    });
}

pub fn get_option_bench(c: &mut Criterion) {
    let board = load_object!(Board, "simple-01", constants::_TEST_PATH);
    let snake = black_box(&board.get_snakes()[0]);
    let value = black_box(0);

    c.bench_function("get_options", |b| b.iter(|| snake.get_option(value)));
}

pub fn game_step_bench(c: &mut Criterion) {
    let board = black_box(load_object!(Board, "food-01", constants::_TEST_PATH));
    let ruleset = load_object!(Ruleset, "food-01", constants::_TEST_PATH);
    let engine = Engine::new();

    c.bench_function("game_step", |b| {
        b.iter(|| engine.game_step(&mut board.clone(), &ruleset))
    });
}

pub fn minimax_bench(c: &mut Criterion) {
    let board = black_box(load_object!(Board, "test_board-03", constants::_TEST_PATH));
    let current = black_box(0);
    let max_depth = black_box(max(
        constants::EXPONENT / board.get_snakes().len() as i32,
        1,
    ));
    let ruleset = load_object!(Ruleset, "test_board-03", constants::_TEST_PATH);
    let simulator = Simulator::new(ruleset);

    c.bench_function("minimax", |b| {
        b.iter(|| simulator.minimax(board.clone(), current, max_depth))
    });
}

pub fn minimax_8_bench(c: &mut Criterion) {
    let board = black_box(load_object!(Board, "test_board-05", constants::_TEST_PATH));
    let current = black_box(0);
    let max_depth = black_box(max(
        constants::EXPONENT / board.get_snakes().len() as i32,
        1,
    ));
    let ruleset = load_object!(Ruleset, "test_board-05", constants::_TEST_PATH);
    let simulator = Simulator::new(ruleset);

    c.bench_function("minimax", |b| {
        b.iter(|| simulator.minimax(board.clone(), current, max_depth))
    });
}

pub fn open_directions_bench(c: &mut Criterion) {
    let board = black_box(load_object!(Board, "food-01", constants::_TEST_PATH));
    let snake = black_box(&board.get_snakes()[0]);

    c.bench_function("open_directions", |b| {
        b.iter(|| board.open_directions(snake))
    });
}

pub fn board_clone_bench(c: &mut Criterion) {
    let board_1 = load_object!(Board, "test_board-04", constants::_TEST_PATH);
    let mut board_2 = load_object!(Board, "test_board-03", constants::_TEST_PATH);

    c.bench_function("board_clone", |b| b.iter(|| board_2 = board_1.clone()));
}

pub fn board_duplicate_bench(c: &mut Criterion) {
    let board_1 = load_object!(Board, "test_board-04", constants::_TEST_PATH);

    let mut board_2 = load_object!(Board, "test_board-03", constants::_TEST_PATH);

    c.bench_function("board_duplicate", |b| {
        b.iter(|| {
            board_2.set_height(board_1.get_height());
            board_2.set_width(board_1.get_width());

            board_2
                .get_food_mut()
                .resize_with(board_1.get_food().len(), Default::default);
            board_2.get_food_mut().copy_from_slice(board_1.get_food());

            board_2
                .get_hazards_mut()
                .resize_with(board_1.get_hazards().len(), Default::default);
            board_2
                .get_hazards_mut()
                .copy_from_slice(board_1.get_hazards());

            board_2
                .get_snakes_mut()
                .resize_with(board_1.get_snakes().len(), Default::default);
            //board_2.get_snakes_mut().clone_from_slice(board_1.get_snakes());

            for i in 0..board_1.get_snakes().len() {
                let snake_1 = &board_1.get_snakes()[i];
                let snake_2 = &mut board_2.get_snakes_mut()[i];

                snake_2.set_id(snake_1.get_id());
                snake_2.set_health(snake_1.get_health());

                snake_2.get_body_mut().clear();
                snake_2.get_body_mut().extend(snake_1.get_body().iter());

                snake_2.set_latency(snake_1._get_latency());
                snake_2.set_head(snake_1.get_head());
                snake_2.set_length(snake_1.get_length());
            }

            board_2.set_max_snakes(board_1.get_max_snakes());
            board_2.set_turn(board_1.get_turn());
        })
    });
}

criterion_group!(
    benches,
    body_collision_with_bench,
    get_option_bench,
    game_step_bench,
    minimax_bench,
    minimax_8_bench,
    open_directions_bench,
    check_area_bench,
    area_controlled_bench,
    board_clone_bench,
    board_duplicate_bench
);

criterion_main!(benches);
