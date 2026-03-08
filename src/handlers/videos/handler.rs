use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::time::Duration;
use uuid::Uuid;
use sqlx::PgPool;

use crate::{s3::S3Uploader, entities::video::{get_video, list_videos}};
use super::response::{VideoResponse, StreamResponse, ListParams, ListResponse};

// GET /videos — returns a paginated list of videos
//
// `Query(params): Query<ListParams>` extracts and deserializes the query string.
// If ?limit or ?offset are missing, the Option fields are None — we apply defaults below.
// If they are present but not valid integers, Axum returns 400 before the handler runs.
pub async fn list_videos_handler(
    State(db): State<PgPool>,
    Query(params): Query<ListParams>,
) -> Response {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0).max(0);

    match list_videos(&db, limit, offset).await {
        Ok(videos) => {
            // `.into_iter().map(...).collect()` is the Rust equivalent of Array.map() in JS,
            // but it's lazy — no work happens until .collect() pulls all the values.
            let data = videos
                .into_iter()
                .map(|v| VideoResponse {
                    id: v.id.to_string(),
                    title: v.title,
                    status: v.status,
                    processed_key: v.processed_key,
                    tags: v.tags,
                    views: v.views,
                    thumbnail_url: v.thumbnail_url,
                })
                .collect();

            (StatusCode::OK, Json(ListResponse { data, limit, offset })).into_response()
        }
        Err(e) => {
            eprintln!("[list_videos] db error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListResponse { data: vec![], limit, offset }),
            )
                .into_response()
        }
    }
}

// GET /videos/:id — returns video metadata
//
// `Path(id): Path<Uuid>` extracts the `:id` segment from the URL and parses it as a UUID.
// If the segment isn't a valid UUID, Axum returns 400 automatically before the handler runs.
// In Express this would be `req.params.id`, with manual UUID validation.
pub async fn get_video_handler(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Response {
    match get_video(&db, id).await {
        Ok(Some(video)) => (
            StatusCode::OK,
            Json(VideoResponse {
                id: video.id.to_string(),
                title: video.title,
                status: video.status,
                processed_key: video.processed_key,
                tags: video.tags,
                views: video.views,
                thumbnail_url: video.thumbnail_url,
            }),
        )
            .into_response(),

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(VideoResponse {
                id: id.to_string(),
                title: String::new(),
                status: "not_found".to_string(),
                processed_key: None,
                tags: Vec::new(),
                views: 0,
                thumbnail_url: None,
            }),
        )
            .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(VideoResponse {
                id: id.to_string(),
                title: String::new(),
                status: "error".to_string(),
                processed_key: None,
                tags: Vec::new(),
                views: 0,
                thumbnail_url: None,
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
    State(db): State<PgPool>,
    State(uploader): State<S3Uploader>,
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

    // Sign a URL valid for 1 hour.
    match uploader.presign_url(&processed_key, Duration::from_secs(3600)).await {
        Ok(url) => (StatusCode::OK, Json(StreamResponse { url })).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(StreamResponse { url: String::new() }),
        )
            .into_response(),
    }
}
