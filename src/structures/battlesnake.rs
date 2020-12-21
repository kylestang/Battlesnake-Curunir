use std::collections::VecDeque;

use crate::board::Board;
use crate::coordinate::Coordinate;

#[derive(Clone, Debug, Eq)]
pub struct Battlesnake {
    id: i32,
    health: i32,
    body: VecDeque<Coordinate>,
    latency: i32,
    head: Coordinate,
    length: i32
}

impl Battlesnake {
    pub fn new(
        id: i32, health: i32, body: VecDeque<Coordinate>, latency: i32,
        head: Coordinate, length: i32
    ) -> Battlesnake {
        Battlesnake {id, health, body, latency, head, length}
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_body(&self) -> &VecDeque<Coordinate> {
        &self.body
    }

    pub fn _get_latency(&self) -> i32 {
        self.latency
    }

    pub fn get_head(&self) -> Coordinate {
        self.head
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }

    pub fn get_down(&self) -> Coordinate {
        self.head.get_down()
    }

    pub fn get_up(&self) -> Coordinate {
        self.head.get_up()
    }

    pub fn get_right(&self) -> Coordinate {
        self.head.get_right()
    }

    pub fn get_left(&self) -> Coordinate {
        self.head.get_left()
    }

    pub fn did_collide(&self, board: &Board) -> bool {
        let x = self.head.get_x();
        let y = self.head.get_y();

        if x < 0 || x > board.get_width() - 1 || y < 0 || y > board.get_height() - 1 {
            return true;
        }

        for snake in board.get_snakes() {
            if snake.get_id() == self.id {
                for i in 1..self.body.len() {
                    if self.body[i] == self.head {
                        return true;
                    }
                }
            } else {
                if snake.body.contains(&self.head) {
                    return true;
                }
            }
        }
        false
    }

    pub fn eat_food(&mut self) {
        self.health = 100;
        self.body.push_back(self.body.back().unwrap().clone());
        self.length += 1;
    }

    pub fn move_to(&mut self, pos: Coordinate) {
        self.body.pop_back();
        self.body.push_front(pos);
        self.head = self.body[0];
        self.health -= 1;
    }
}

impl PartialEq for Battlesnake {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
