use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use index_response::IndexResponse;
use move_request::MoveRequest;
use move_response::MoveResponse;

use crate::requests::*;
use crate::structures::*;

mod constants;
mod requests;
mod structures;

// Index response
#[get("/battlesnake/curunir")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(IndexResponse::new(
        constants::API_VERSION,
        constants::AUTHOR,
        constants::COLOR,
        constants::HEAD,
        constants::TAIL
    ))
}

// Game start
#[post("/battlesnake/curunir/start")]
async fn start() -> HttpResponse {
    println!("Start");
    HttpResponse::Ok().body("")
}

// Game move response
#[post("/battlesnake/curunir/move")]
async fn game_move(data: web::Json<MoveRequest>) -> HttpResponse {
    println!("Move");
    // Get data from MoveRequest
    let values = data.into_inner().into_values();
    // Create Board from InputBoard
    let board = values.2.into_board(values.3, values.1);
    // Get game from MoveRequest
    let game = values.0;
    // Respond with direction
    let direction = game.decision(board);
    HttpResponse::Ok().json(MoveResponse::new(direction, String::from("Hi!")))
}

// Game end
#[post("/battlesnake/curunir/end")]
async fn end() -> HttpResponse {
    println!("End");
    HttpResponse::Ok().body("")
}

// Start web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(start)
            .service(game_move)
            .service(end)
    })
    .bind("0.0.0.0:25571")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::test;

    use crate::load_object;

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/battlesnake/curunir").to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_move_post() {
        let data = load_object!(MoveRequest, "simple-02");

        let mut app = test::init_service(App::new().service(game_move)).await;
        let req = test::TestRequest::post().set_json(&data).uri("/battlesnake/curunir/move").to_request();
        println!("{}", req.path());
        let resp = test::call_service(&mut app, req).await;
        println!("{}", resp.status());
        assert!(resp.status().is_success());
    }
}
