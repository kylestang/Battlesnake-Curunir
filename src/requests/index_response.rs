use serde::Serialize;

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
