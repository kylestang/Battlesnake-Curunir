use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use index_response::IndexResponse;
use move_request::MoveRequest;
use move_response::MoveResponse;

use curunir::constants::*;
use curunir::requests::*;

// Index response
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(IndexResponse::new(API_VERSION, AUTHOR, COLOR, HEAD, TAIL))
}

// Game start
#[post("/start")]
async fn start() -> HttpResponse {
    println!("Start");
    HttpResponse::Ok().body("")
}

// Game move response
#[post("/move")]
async fn game_move(data: web::Json<MoveRequest>) -> HttpResponse {
    println!("Move");
    // Get data from MoveRequest
    let (input_game, turn, input_board, you) = data.into_inner().into_values();
    // Create Board from InputBoard
    let board = input_board.into_board(you, turn);
    // Respond with direction
    let direction = input_game.into_game().calculate_move(board);
    HttpResponse::Ok().json(MoveResponse::new(direction, String::from("Hi!")))
}

// Game end
#[post("/end")]
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
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::test;
    use curunir::load_object;

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::with_header("content-type", "text/plain")
            .uri("/")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_move_post() {
        let data = load_object!(MoveRequest, "simple-02", _TEST_PATH);

        let mut app = test::init_service(App::new().service(game_move)).await;
        let req = test::TestRequest::post()
            .set_json(&data)
            .uri("/move")
            .to_request();
        println!("{}", req.path());
        let resp = test::call_service(&mut app, req).await;
        println!("{}", resp.status());
        assert!(resp.status().is_success());
    }
}
