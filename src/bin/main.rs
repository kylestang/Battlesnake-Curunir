use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use index_response::IndexResponse;
use move_request::MoveRequest;
use move_response::MoveResponse;

use curunir::requests::*;
use curunir::constants::*;

// Index response
#[get("/battlesnake/curunir")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(IndexResponse::new(
        API_VERSION,
        AUTHOR,
        COLOR,
        HEAD,
        TAIL
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

    macro_rules! load_object {
        (Board, $filename:expr) => {
            {
                let file: std::fs::File = std::fs::OpenOptions::new()
                    .read(true).open(format!("{}{}.json", curunir::constants::_TEST_PATH, $filename)).unwrap();
                let board: crate::move_request::MoveRequest = serde_json::from_reader(file).unwrap();
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
