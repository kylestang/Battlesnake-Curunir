/*
use actix_web::test;
use super::*;
use structures::{Game, Ruleset, InputBoard, Coordinate, InputSnake};

#[actix_rt::test]
async fn test_index_get() {
    let mut app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{}", resp.status());
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_move_post() {
    let data = MoveRequest::_new(
        Game::_new(
            String::from("game-00fe20da-94ad-11ea-bb37"),
            Ruleset::_new(
                String::from("standard"),
                String::from("v.1.2.3")
            ),
            500
        ),
        14,
        InputBoard::_new(
            11,
            11,
            vec![
                Coordinate::new(5, 5),
                Coordinate::new(9, 0),
                Coordinate::new(2, 6)
            ],
            vec![
                Coordinate::new(0, 0)
            ],
            vec![
                InputSnake::_new(
                    String::from("snake-508e96ac-94ad-11ea-bb37"),
                    String::from("My Snake"),
                    54,
                    vec![
                        Coordinate::new(0, 0),
                        Coordinate::new(1, 0),
                        Coordinate::new(2, 0)
                    ],
                    String::from("111"),
                    Coordinate::new(0, 0),
                    3,
                    String::from("why are we shouting??"),
                ),
                InputSnake::_new(
                    String::from("snake-b67f4906-94ae-11ea-bb37"),
                    String::from("Another Snake"),
                    16,
                    vec![
                        Coordinate::new(5, 4),
                        Coordinate::new(5, 3),
                        Coordinate::new(6, 3),
                        Coordinate::new(6, 2)
                    ],
                    String::from("222"),
                    Coordinate::new(5, 4),
                    4,
                    String::from("I'm not really sure..."),
                )
            ]
        ),
        InputSnake::_new(
            String::from("snake-508e96ac-94ad-11ea-bb37"),
            String::from("My Snake"),
            54,
            vec![
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(2, 0)
            ],
            String::from("111"),
            Coordinate::new(0, 0),
            3,
            String::from("why are we shouting??"),
        )
    );


    let mut app = test::init_service(App::new().service(game_move)).await;
    let req = test::TestRequest::post().set_json(&data).uri("/move").to_request();
    println!("{}", req.path());
    let resp = test::call_service(&mut app, req).await;
    println!("{}", resp.status());
    assert!(resp.status().is_success());
}
*/