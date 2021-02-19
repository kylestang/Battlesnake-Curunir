use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::battlesnake::Battlesnake;
use crate::coordinate::Coordinate;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputSnake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Coordinate>,
    latency: String,
    head: Coordinate,
    length: usize,
    shout: String,
}

impl InputSnake {

    pub fn get_id(&self) -> &String {
        &self.id
    }

    // Convert self to a Battlesnake struct
    pub fn into_battlesnake(self, id: i32) -> Battlesnake {
        Battlesnake::new(
            id,
            self.health,
            VecDeque::from(self.body),
            self.latency.parse().unwrap_or(0),
            self.head,
            self.length
        )
    }
}
