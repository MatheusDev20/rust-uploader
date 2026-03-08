use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use crate::handlers::upload::upload_handler;
use crate::handlers::video::{get_video_handler, stream_handler};

use tower_http::cors::{Any, CorsLayer};

use crate::{AppState, utils::constants::MAX_SIZE};

pub fn init_routes(state: AppState) -> Router {

    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .route("/upload", post(upload_handler))
        // GET /videos/:id        — metadata (title, status, processed_key)
        // GET /videos/:id/stream — temporary pre-signed S3 URL for the client to play
        .route("/videos/{id}", get(get_video_handler))
        .route("/videos/{id}/stream", get(stream_handler))
        .layer(cors)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
        .with_state(state)
}
