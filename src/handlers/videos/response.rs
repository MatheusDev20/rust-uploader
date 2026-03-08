use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct VideoResponse {
    pub id: String,
    pub title: String,
    pub status: String,
    pub processed_key: Option<String>,
    pub tags: Vec<String>,
    pub views: i32,
    pub thumbnail_url: Option<String>,
}

#[derive(Serialize)]
pub struct StreamResponse {
    pub url: String,
}

// Query parameters for GET /videos — all optional, with sensible defaults.
// `#[derive(Deserialize)]` lets Axum parse `?limit=10&offset=20` into this struct
// automatically. Fields wrapped in Option<> are simply missing from the URL if not provided.
// In Express: const { limit = 20, offset = 0 } = req.query
#[derive(Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// The JSON envelope returned by GET /videos.
// Includes the page params alongside the data so the client knows what window it received
// and can compute the next page: next_offset = offset + data.len()
#[derive(Serialize)]
pub struct ListResponse {
    pub data: Vec<VideoResponse>,
    pub limit: i64,
    pub offset: i64,
}
