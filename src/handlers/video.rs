use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::time::Duration;
use uuid::Uuid;

use crate::{AppState, entities::video::get_video};

// The JSON shape returned by GET /videos/:id
// `#[derive(Serialize)]` auto-generates the code to convert this struct to JSON —
// same as implementing toJSON() in JS, but at compile time with zero runtime cost.
#[derive(Serialize)]
struct VideoResponse {
    id: String,
    title: String,
    status: String,
    // `Option<String>` serializes as `null` in JSON when the value is None.
    // In JS you'd just have `processed_key: null` — Rust makes the possibility explicit.
    processed_key: Option<String>,
}

// The JSON shape returned by GET /videos/:id/stream
#[derive(Serialize)]
struct StreamResponse {
    url: String,
}

// `impl IntoResponse` is a flexible return type — the function can return any type
// that Axum knows how to turn into an HTTP response. A tuple `(StatusCode, Json<T>)`
// satisfies this automatically. It's like returning `Response` in an Express handler,
// but with compile-time type checking.

// GET /videos/:id — returns video metadata
//
// `Path(id): Path<Uuid>` extracts the `:id` segment from the URL and parses it as a UUID.
// If the segment isn't a valid UUID, Axum returns 400 automatically before the handler runs.
// In Express this would be `req.params.id`, with manual UUID validation.
pub async fn get_video_handler(
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response {
    match get_video(&db, id).await {
        // `Ok(Some(video))` — found the row, map it to our response shape.
        Ok(Some(video)) => (
            StatusCode::OK,
            Json(VideoResponse {
                id: video.id.to_string(),
                title: video.title,
                status: video.status,
                processed_key: video.processed_key,
            }),
        )
            .into_response(),

        // `Ok(None)` — query succeeded but no row matched; return 404.
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(VideoResponse {
                id: id.to_string(),
                title: String::new(),
                status: "not_found".to_string(),
                processed_key: None,
            }),
        )
            .into_response(),

        // `Err(_)` — database error; return 500.
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(VideoResponse {
                id: id.to_string(),
                title: String::new(),
                status: "error".to_string(),
                processed_key: None,
            }),
        )
            .into_response(),
    }
}

// GET /videos/:id/stream — returns a temporary pre-signed S3 URL
//
// The client uses this URL directly in <video src="...">. The browser streams
// bytes from S3 — your server is not in the data path at all after this response.
pub async fn stream_handler(
    State(AppState { uploader, db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Response {
    let video = match get_video(&db, id).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(StreamResponse { url: String::new() }),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(StreamResponse { url: String::new() }),
            )
                .into_response()
        }
    };

    // If the video is still pending (transcoding not finished), there is no processed
    // file yet — return 409 Conflict so the client knows to poll and retry.
    let processed_key = match video.processed_key {
        Some(k) => k,
        None => {
            return (
                StatusCode::CONFLICT,
                Json(StreamResponse { url: String::new() }),
            )
                .into_response()
        }
    };

    // Sign a URL valid for 1 hour. The client can start streaming immediately
    // after receiving it; it doesn't need to be short — even 15 minutes is fine.
    match uploader.presign_url(&processed_key, Duration::from_secs(3600)).await {
        Ok(url) => (StatusCode::OK, Json(StreamResponse { url })).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(StreamResponse { url: String::new() }),
        )
            .into_response(),
    }
}
