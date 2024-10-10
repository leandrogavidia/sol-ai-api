use axum::{response::IntoResponse, routing::post, Json, Router};

use collections::api::{Request, Response};
use docs::read_markdown;
use process::process_files;

mod collections;
mod docs;
mod process;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = Router::new().route("/api/chat", post(chat));

    Ok(app.into())
}

async fn chat(Json(_payload): Json<Request>) -> impl IntoResponse {

    let _ = process_files("/src/files");

    let ore_content: String = read_markdown("/src/files/ore.md");
    let solana_allstars_content: String = read_markdown("/src/files/solana-allstars.md");
    let la_familia_content: String = read_markdown("/src/files/la-familia.md");
    let local_solana_content: String = read_markdown("/src/files/local-solana.md");
    let heavy_duty_builders_content: String = read_markdown("/src/files/heavy-duty-builders.md");

    let content: Vec<String> = vec![
        ore_content,
        solana_allstars_content,
        la_familia_content,
        local_solana_content,
        heavy_duty_builders_content
    ];

    let response = Response {
        status: 200,
        content,
    };

    Json(response)
}
