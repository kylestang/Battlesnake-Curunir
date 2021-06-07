use serde::Serialize;

#[derive(Serialize)]
pub struct MoveResponse {
    r#move: String,
    shout: String,
}

impl MoveResponse {
    pub fn new(r#move: String, shout: String) -> MoveResponse {
        MoveResponse { r#move, shout }
    }
}
