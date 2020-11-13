use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use structures::{MoveRequest, IndexResponse, Battlesnake, Board};
use decision::decision;
use std::collections::VecDeque;

mod structures;
mod decision;
mod constants;
mod functions;
mod draw;

#[cfg(test)]
mod tests;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(IndexResponse::new(
        constants::API_VERSION,
        constants::AUTHOR,
        constants::COLOR,
        constants::HEAD,
        constants::TAIL
    ))
}

#[post("/start")]
async fn start() -> HttpResponse {
    println!("Start");
    HttpResponse::Ok().body("")
}

#[post("/move")]
async fn game_move(data: web::Json<MoveRequest>) -> HttpResponse {
    println!("Move");
    
    let input_board = data.get_board();
    let mut snakes = Vec::new();

    let mut id: i32 = 1;
    for snake in input_board.get_snakes(){
        snakes.push(
            Battlesnake::new(
                if snake.get_id() == data.get_you().get_id() {0} else {id},
                snake.get_health(),
                VecDeque::from(snake.get_body().clone()),
                snake.get_latency().parse().unwrap(),
                snake.get_head(),
                snake.get_length()
            )
        );

        id += 1;
    }

    let board = Board::new(
        input_board.get_height(),
        input_board.get_width(),
        input_board.get_food().clone(),
        input_board.get_hazards().clone(),
        snakes
    );

    let input_you = data.get_you();
    let you = Battlesnake::new(
        0,
        input_you.get_health(),
        VecDeque::from(input_you.get_body().clone()),
        input_you.get_latency().parse().unwrap(),
        input_you.get_head(),
        input_you.get_length()
    );

    let game = data.get_game().clone();

    HttpResponse::Ok().json(decision(game, data.get_turn(), board, you))
}

#[post("/end")]
async fn end() -> HttpResponse {
    println!("End");
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(start)
            .service(game_move)
            .service(end)
    })
    .bind("0.0.0.0:25580")?
    .run()
    .await
}
