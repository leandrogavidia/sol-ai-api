use axum::{ response::IntoResponse, routing::post, Json, Router};
use std::fs;
use std::env;

use collections::api::{ Request, Response };

mod collections;

fn read_markdown_file(file_path: &str) -> String {
    let file = fs::read_to_string(file_path).expect("EERR");
    file
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = Router::new().route("/api/chat", post(chat));

    Ok(app.into())
}

async fn chat(
    Json(payload): Json<Request>,
) -> impl IntoResponse {
    let current_dir = env::current_dir().unwrap();
    let file_path = format!("{}/src/ore.md", current_dir.to_str().unwrap());

    let ore_content = read_markdown_file(&file_path);

    let mut content: Vec<String> = vec![];

    content.push(ore_content);

    let response = Response {
        status: 200,
        content
    };

    Json(response)
}

