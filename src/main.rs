mod s3;
mod http;
mod utils;
mod extractor;


use axum::{extract::{DefaultBodyLimit, Multipart, State}, routing::post, Json, Router};
use s3::{create_uploader, S3Uploader};
use http::responses::UploadResponse;
use utils::constants::{ MAX_SIZE, UPLOADS_FOLDER };
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    uploader: S3Uploader,

}


// State<S3Uploader> is how Axum injects the shared uploader into this handler.
// Axum reads the function parameters and automatically provides whatever is registered
// with .with_state() — same idea as dependency injection in a JS framework.

// Multipart is Axum's extractor for multipart/form-data requests (how browsers send files).
// Like State, Axum injects it automatically because it's declared as a parameter.
// `mut` is needed because reading fields advances an internal cursor — the extractor is stateful.

async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Json<UploadResponse> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "file" {
            // .bytes() collects the entire field into memory as raw bytes (Vec<u8>)

            let video_id = Uuid::new_v4();
            let video_key = format!("{}/{}.mp4", UPLOADS_FOLDER, video_id);
            let data = field.bytes().await.unwrap().to_vec();

            state.uploader.upload(&video_key, data).await.unwrap();
            
            return Json(UploadResponse {
                status: "ok",
                message: "video uploaded",
            });
        }
    }

    Json(UploadResponse {
        status: "error",
        message: "no file field found in request",
    })
}

#[tokio::main]
async fn main() {

    // Load the .env file into the process environment — same as require('dotenv').config() in JS
    // .ok() means: if the file doesn't exist, silently continue instead of crashing
    dotenvy::dotenv().ok();

    let uploader = create_uploader().await;

     let state = AppState {uploader };
    let app = Router::new()
        .route("/upload", post(upload))
        // DefaultBodyLimit rejects the request before it reaches the handler
        // if the Content-Length exceeds the limit — like multer's limits.fileSize in JS
        .layer(DefaultBodyLimit::max(MAX_SIZE))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
