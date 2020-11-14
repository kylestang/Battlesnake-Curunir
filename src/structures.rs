use crate::constants::DIRECTIONS;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Serialize)]
pub struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate {x, y}
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    // Down, up, right, left
    pub fn get_adjacent(&self) -> [Coordinate; DIRECTIONS] {
        [
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y)
        ]
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: String,
    ruleset: Ruleset,
    timeout: i32
}

impl Game {
    pub fn _new(id: String, ruleset: Ruleset, timeout: i32) -> Game {
        Game {id, ruleset, timeout}
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn _get_ruleset(&self) -> &Ruleset {
        &self.ruleset
    }

    pub fn _get_timeout(&self) -> i32 {
        self.timeout
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ruleset {
    name: String,
    version: String
}

impl Ruleset {
    pub fn _new(name: String, version: String) -> Ruleset {
        Ruleset {name, version}
    }

    pub fn _get_name(&self) -> &String {
        &self.name
    }

    pub fn _get_version(&self) -> &String {
        &self.version
    }
}

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

    pub fn move_to(&mut self, pos: Coordinate) {
        self.body.pop_back();
        self.body.push_front(pos);
        self.head = self.body[0];
        self.health -= 1;
    }

    pub fn eat_food(&mut self) {
        self.health = 100;
        self.body.push_back(self.body.back().unwrap().clone());
        self.length += 1;
    }
}

impl PartialEq for Battlesnake {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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

#[derive(Clone, Debug)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>
}

impl Board {
    pub fn new(height: i32, width: i32, food: Vec<Coordinate>, hazards: Vec<Coordinate>, snakes: Vec<Battlesnake>) -> Board {
        Board {height, width, food, hazards, snakes}
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_food(&mut self) -> &mut Vec<Coordinate> {
        &mut self.food
    }

    pub fn _get_hazards(&self) -> &Vec<Coordinate> {
        &self.hazards
    }

    pub fn get_snakes(&mut self) -> &mut Vec<Battlesnake> {
        &mut self.snakes
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
