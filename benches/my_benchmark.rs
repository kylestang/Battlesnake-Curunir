use criterion::{black_box, criterion_group, criterion_main, Criterion};
use curunir::constants;
use curunir::load_object;
use curunir::requests::*;
use curunir::structures::coordinate::Coordinate;

use std::cmp::max;

pub fn area_controlled_bench(c: &mut Criterion) {
    let board = load_object!(Board, "test_board-03", constants::_TEST_PATH);

    c.bench_function("area_controlled", |b| b.iter(|| board.area_controlled()));
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

    c.bench_function("game_step", |b| {
        b.iter(|| board.clone().game_step(&ruleset))
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

    c.bench_function("minimax", |b| {
        b.iter(|| board.clone().minimax(&ruleset, current, max_depth))
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

    c.bench_function("minimax", |b| {
        b.iter(|| board.clone().minimax(&ruleset, current, max_depth))
    });
}

pub fn open_directions_bench(c: &mut Criterion) {
    let board = black_box(load_object!(Board, "food-01", constants::_TEST_PATH));
    let snake = black_box(&board.get_snakes()[0]);

    c.bench_function("open_directions", |b| {
        b.iter(|| board.open_directions(snake))
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
);

criterion_main!(benches);
