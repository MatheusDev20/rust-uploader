use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::entities::resource_contents::{ResourceSearchResult, search};

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    // Language for stemming — defaults to 'portuguese'.
    // Pass 'english' for zendesk docs, 'simple' to skip stemming entirely.
    pub language: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub data: Vec<ResourceSearchResult>,
    pub query: String,
    pub limit: i64,
    pub offset: i64,
}

pub async fn search_resources_handler(
    State(db): State<PgPool>,
    Query(params): Query<SearchParams>,
) -> Response {
    if params.q.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(SearchResponse { data: vec![], query: params.q, limit: 0, offset: 0 }),
        )
            .into_response();
    }

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0).max(0);
    let language = params.language.as_deref().unwrap_or("portuguese");

    match search(&db, &params.q, language, limit, offset).await {
        Ok(results) => (
            StatusCode::OK,
            Json(SearchResponse { data: results, query: params.q, limit, offset }),
        )
            .into_response(),

        Err(e) => {
            eprintln!("[search] db error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SearchResponse { data: vec![], query: params.q, limit, offset }),
            )
                .into_response()
        }
    }
}
