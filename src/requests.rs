use serde::{Serialize, Deserialize};

use crate::coordinate::Coordinate;
use crate::game::Game;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputSnake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Coordinate>,
    latency: String,
    head: Coordinate,
    length: i32,
    shout: String,
}

impl InputSnake {
    pub fn _new(
        id: String, name: String, health: i32, body: Vec<Coordinate>, latency: String,
        head: Coordinate, length: i32, shout: String
    ) -> InputSnake {
        InputSnake {id, name, health, body, latency, head, length, shout}
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn _get_name(&self) -> &String {
        &self.name
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_body(&self) -> &Vec<Coordinate> {
        &self.body
    }

    pub fn get_latency(&self) -> &String {
        &self.latency
    }

    pub fn get_head(&self) -> Coordinate {
        self.head
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputBoard {
    height: i32,
    width: i32,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<InputSnake>
}

impl InputBoard {
    pub fn _new(height: i32, width: i32, food: Vec<Coordinate>,
        hazards: Vec<Coordinate>, snakes: Vec<InputSnake>) -> InputBoard {
            InputBoard {height, width, food, hazards, snakes}
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_food(&self) -> &Vec<Coordinate> {
        &self.food
    }

    pub fn get_hazards(&self) -> &Vec<Coordinate> {
        &self.hazards
    }

    pub fn get_snakes(&self) -> &Vec<InputSnake> {
        &self.snakes
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveRequest {
    game: Game,
    turn: i32,
    board: InputBoard,
    you: InputSnake
}

impl MoveRequest {
    pub fn _new(game: Game, turn: i32, board: InputBoard, you: InputSnake) -> MoveRequest {
        MoveRequest {game, turn, board, you}
    }

    pub fn get_game(&self) -> &Game {
        &self.game
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }

    pub fn get_board(&self) -> &InputBoard {
        &self.board
    }

    pub fn get_you(&self) -> &InputSnake {
        &self.you
    }
}

#[derive(Serialize)]
pub struct IndexResponse {
    apiversion: &'static str,
    author: &'static str,
    color: &'static str,
    head: &'static str,
    tail: &'static str,
}

impl IndexResponse {
    pub fn new(
        apiversion: &'static str, author: &'static str,
        color: &'static str, head: &'static str, tail: &'static str
    ) -> IndexResponse {
        IndexResponse {apiversion, author, color, head, tail}
    }
}

#[derive(Serialize)]
pub struct MoveResponse {
    r#move: String,
    shout:String
}

impl MoveResponse {
    pub fn new(r#move: String, shout: String) -> MoveResponse {
        MoveResponse {r#move, shout}
    }
}
