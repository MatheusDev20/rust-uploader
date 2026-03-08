use axum::extract::State;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::entities::resource_contents::insert_content;
use crate::entities::resources::create_resource;
use crate::http::extractor::JsonBody;
use crate::extractors::language_for;

use crate::http::responses::{ApiResponse, internal_error, ok};

#[derive(Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub resource_type: String,
    pub source_type: String,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub async fn new_resource_handler(
    State(db): State<PgPool>,
    JsonBody(body): JsonBody<CreateResourceRequest>,
) -> ApiResponse {
    let id = Uuid::new_v4();
    let tags = body.tags.unwrap_or_default();

    if let Err(_) = create_resource(
        &db, id,
        &body.title, &body.slug,
        &body.resource_type, &body.source_type,
        &body.summary, &body.description, &body.url,
        &tags,
    ).await {
        return internal_error("failed to create resource");
    }

    // Build raw_text from the fields we already have — no external fetching needed.
    // title + summary + description + tags give enough signal for full-text search.

    let raw_text = [
        Some(body.title.as_str()),
        body.summary.as_deref(),
        body.description.as_deref(),
    ]
    .iter()
    .filter_map(|s| *s)
    .chain(tags.iter().map(String::as_str))
    .collect::<Vec<_>>()
    .join(" ");

    let language = language_for(&body.source_type);
    let content_id = Uuid::new_v4();

    if let Err(_) = insert_content(&db, content_id, id, &raw_text, language).await {
        return internal_error("failed to index resource content");
    }

    ok("resource created")
}
