use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub status: u16,
    pub content: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessResponse {
    pub status: u16,
    pub message: String,
}