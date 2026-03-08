mod s3;
mod http;
mod utils;
mod ffmpeg;
mod entities;
mod handlers;
mod extractors;
mod routes;


use s3::{create_uploader, S3Uploader};
use sqlx::PgPool;
use axum::extract::FromRef;
use routes::init_routes;

#[derive(Clone)]
pub struct AppState {
    uploader: S3Uploader,
    db: PgPool
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for S3Uploader {
    fn from_ref(state: &AppState) -> Self {
        state.uploader.clone()
    }
}


// State<AppState> is how Axum injects the shared state into this handler.
// Axum reads the function parameters and automatically provides whatever is registered
// with .with_state() — same idea as dependency injection in a JS framework.

// Multipart is Axum's extractor for multipart/form-data requests (how browsers send files).
// Like State, Axum injects it automatically because it's declared as a parameter.
// `mut` is needed because reading fields advances an internal cursor — the extractor is stateful.

#[tokio::main]
async fn main() {

    // Load the .env file into the process environment — same as require('dotenv').config() in JS
    // .ok() means: if the file doesn't exist, silently continue instead of crashing
    dotenvy::dotenv().ok();


    let uploader = create_uploader().await;
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let state = AppState { uploader, db: pool };

    let app = init_routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
