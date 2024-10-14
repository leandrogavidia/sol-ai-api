use axum::{response::IntoResponse, routing::post, Json, Router};

use collections::{api::{ChatResponse, Request}, ProcessResponse};
use docs::read_markdown;
use process::{process_files, query_collection, delete_collection, query_example};

mod collections;
mod docs;
mod process;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .route("/api/query", post(query))
        .route("/api/process", post(process))
        .route("/api/delete", post(delete));

    Ok(app.into())
}

async fn query(Json(payload): Json<Request>) -> impl IntoResponse {
    let content = vec!["a".to_string()];
    println!("PAYLOAD: {}", payload.message);
    // let _ = query_collection(&payload.message);
    let _ = query_example();
    let response = ChatResponse {
        status: 200,
        content,
    };

    Json(response)
}

async fn process() -> impl IntoResponse {
    let _ = process_files("/src/files");

    let response = ProcessResponse {
        status: 200,
        message: format!("Files successfully proccessed!")
    };

    Json(response)
}

async fn delete() -> impl IntoResponse {
    let _ = delete_collection();

    let response = ProcessResponse {
        status: 200,
        message: format!("Files successfully proccessed!")
    };

    Json(response)
}

