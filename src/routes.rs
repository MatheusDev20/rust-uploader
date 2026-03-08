use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};

use tower_http::cors::{Any, CorsLayer};

use crate::AppState;
use crate::utils::constants::MAX_SIZE;
use crate::handlers::videos::{upload_handler, get_video_handler, list_videos_handler, stream_handler};
use crate::handlers::resources::new_resource_handler;

pub fn init_routes(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);

    Router::new()
        /* Videos Routes */
        .route("/upload", post(upload_handler))
        .route("/videos", get(list_videos_handler))
        .route("/videos/{id}", get(get_video_handler))
        .route("/videos/{id}/stream", get(stream_handler))
        /* Resources Routes */
        .route("/resources", post(new_resource_handler))

        .layer(cors)
        .layer(DefaultBodyLimit::max(MAX_SIZE))
        .with_state(state)
}
