use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub status: u16,
    pub content: Vec<String>,
}